use super::{
    Axis, Blade, Blank, Blunt, Canon, ConstOne, Honed, Oriented, Sorted, SquareMap, Squared,
};
use crate::err;
use crate::sort::{parity_merge, parity_sort};
use itertools::Itertools;
use std::ops::{Mul, Neg};

impl Default for Blade {
    fn default() -> Self {
        Blade::Zero
    }
}

impl Mul for Canon {
    type Output = Blunt;
    fn mul(self, rhs: Self) -> Self::Output {
        match parity_merge(self.0, rhs.0) {
            (false, frame) => Blunt::Pos(Sorted(frame)),
            (true, frame) => Blunt::Neg(Sorted(frame)),
        }
    }
}
impl Mul for Blade {
    type Output = Blunt;
    fn mul(self, other: Self) -> Self::Output {
        let (blade_parity, lhs, rhs) = match (self, other) {
            (Blade::Zero, _) => return Blunt::Zero,
            (_, Blade::Zero) => return Blunt::Zero,
            (Blade::Pos(l), Blade::Pos(r)) | (Blade::Neg(l), Blade::Neg(r)) => (false, l.0, r.0),
            (Blade::Pos(l), Blade::Neg(r)) | (Blade::Neg(l), Blade::Pos(r)) => (true, l.0, r.0),
        };
        let (merge_parity, frame) = parity_merge(lhs, rhs);
        match blade_parity ^ merge_parity {
            false => Blunt::Pos(Sorted(frame)),
            true => Blunt::Neg(Sorted(frame)),
        }
    }
}
impl<T> Mul<Oriented<T>> for Squared {
    type Output = Oriented<T>;
    fn mul(self, rhs: Oriented<T>) -> Self::Output {
        match (self, rhs) {
            (Self::Zero, _) => Oriented::Zero,
            (Self::Pos(()), rhs) => rhs,
            (Self::Neg(()), rhs) => -rhs,
        }
    }
}

impl From<Squared> for Blade {
    fn from(value: Squared) -> Self {
        match value {
            Squared::Zero => Blade::Zero,
            Squared::Pos(()) => Blade::One,
            Squared::Neg(()) => -Blade::One,
        }
    }
}
impl From<Axis> for Blade {
    fn from(value: Axis) -> Self {
        Blade::Pos(Honed(vec![value]))
    }
}
impl From<Canon> for Blade {
    fn from(canon: Canon) -> Self {
        Blade::Pos(canon)
    }
}
impl From<Blank> for Blunt {
    fn from(value: Blank) -> Self {
        let (parity, mut frame) = match value {
            Blank::Zero => return Blunt::Zero,
            Blank::Pos(frame) => (false, frame),
            Blank::Neg(frame) => (true, frame),
        };
        match parity ^ parity_sort(&mut frame) {
            false => Blunt::Pos(Sorted(frame)),
            true => Blunt::Neg(Sorted(frame)),
        }
    }
}
impl From<Blade> for Blunt {
    fn from(value: Blade) -> Self {
        match value {
            Blade::Zero => Blunt::Zero,
            Blade::Pos(Honed(frame)) => Blunt::Pos(Sorted(frame)),
            Blade::Neg(Honed(frame)) => Blunt::Neg(Sorted(frame)),
        }
    }
}
impl TryFrom<Blunt> for Blade {
    type Error = syn::Error;
    fn try_from(value: Blunt) -> Result<Self, Self::Error> {
        match value {
            Blunt::Zero => return Ok(Blade::Zero),
            Blunt::Pos(Sorted(frame)) => (!has_dups(&frame)).then_some(Blade::Pos(Honed(frame))),
            Blunt::Neg(Sorted(frame)) => (!has_dups(&frame)).then_some(Blade::Neg(Honed(frame))),
        }
        .ok_or(err!("not honed"))
    }
}

impl ConstOne for Squared {
    const One: Self = Self::Pos(());
}
impl ConstOne for Blade {
    const One: Self = Self::Pos(Honed(vec![]));
}
impl ConstOne for Canon {
    const One: Self = Honed(vec![]);
}
impl ConstOne for Blunt {
    const One: Self = Self::Pos(Sorted(vec![]));
}
impl ConstOne for Blank {
    const One: Self = Self::Pos(vec![]);
}
impl<T> Neg for Oriented<T> {
    type Output = Self;
    fn neg(self) -> Self::Output {
        match self {
            Self::Neg(t) => Self::Pos(t),
            Self::Pos(t) => Self::Neg(t),
            Self::Zero => Self::Zero,
        }
    }
}
impl Squared {
    fn pow(self, n: usize) -> Self {
        match (self, n) {
            (_, 0) => Self::One,
            (Self::Zero, _) => Self::Zero,
            (_, n) if n & 2 == 0 => Self::One,
            (sq, _) => sq,
        }
    }
}

impl Axis {
    fn square(&self, squares: &SquareMap) -> Squared {
        let err_msg = format!("axis {self} missing square!");
        squares.0.get(&self).expect(&err_msg).clone()
    }
}

fn has_dups<T: PartialEq>(slice: &[T]) -> bool {
    slice.iter().tuple_windows().any(|(a, b)| a == b)
}
impl Blunt {
    pub(super) fn dual_hone(self, squares: &SquareMap) -> Blade {
        let (mut parity, sorted_frame) = match self {
            Blunt::Zero => return Blade::Zero,
            Blunt::Pos(Sorted(frame)) => (false, frame),
            Blunt::Neg(Sorted(frame)) => (true, frame),
        };
        let canon = if !has_dups(&sorted_frame) {
            Honed(sorted_frame)
        } else {
            let mut honed_frame = vec![];
            for (n, axis) in sorted_frame.into_iter().dedup_with_count() {
                match n {
                    1 => honed_frame.push(axis),
                    2 => parity ^= matches!(axis.square(squares), Squared::Neg(())),
                    _ => unreachable!(),
                }
            }
            Honed(honed_frame)
        };
        match parity {
            false => Blade::Pos(canon),
            true => Blade::Neg(canon),
        }
    }
    pub(crate) fn hone(self, sq: &SquareMap) -> Blade {
        let (mut parity, frame) = match self {
            Blunt::Zero => return Blade::Zero,
            Blunt::Pos(Sorted(frame)) => (false, frame),
            Blunt::Neg(Sorted(frame)) => (true, frame),
        };
        let canon = if !has_dups(&frame) {
            Honed(frame)
        } else {
            let mut honed_frame = vec![];
            for (n, axis) in frame.into_iter().dedup_with_count() {
                match Blade::from(axis).pow(n, sq) {
                    Blade::Zero => return Blade::Zero,
                    Blade::Pos(Honed(axis)) => honed_frame.extend(axis),
                    Blade::Neg(Honed(axis)) => {
                        parity ^= true;
                        honed_frame.extend(axis)
                    }
                }
            }
            Honed(honed_frame)
        };
        match parity {
            false => Blade::Pos(canon),
            true => Blade::Neg(canon),
        }
    }
}

impl Blade {
    pub fn len(&self) -> usize {
        match self {
            Blade::Zero => 0,
            Blade::Pos(canon) | Blade::Neg(canon) => canon.0.len(),
        }
    }
    pub fn rev(self) -> Self {
        match self.len() % 4 {
            0 | 1 => self,
            2 | 3 => -self,
            _ => unreachable!(),
        }
    }
    pub fn square(&self, squares: &SquareMap) -> Squared {
        let canon = match self {
            Blade::Pos(canon) | Blade::Neg(canon) => canon,
            _ => return Squared::Zero,
        };
        canon
            .0
            .iter()
            .map(|axis| axis.square(squares))
            .fold(Squared::One, Squared::mul)
    }
    pub fn pow(self, n: usize, sq: &SquareMap) -> Blade {
        match n {
            0 => Blade::One,
            1 => self,
            2 => self.square(sq).into(),
            3 => self.square(sq) * self,
            n if n % 2 == 0 => self.square(sq).pow(n / 2).into(),
            n => self.square(sq).pow(n / 2) * self,
        }
    }
}

impl Blank {
    pub fn hone(self, sq: &SquareMap) -> Blade {
        Blunt::from(self).hone(sq)
    }
}

#[allow(unused)]
macro_rules! bl {
    ($enc:expr) => {{
        let bl: crate::geometry::Blank = stringify!($enc).parse().unwrap();
        let bl: crate::geometry::Blunt = bl.into();
        let bl: crate::geometry::Blade = bl.try_into().unwrap();
        bl
    }};
}

#[allow(unused)]
macro_rules! ax {
    ($name:ident) => {
        stringify!($name).parse::<crate::geometry::Axis>().unwrap()
    };
}

#[allow(unused)]
macro_rules! sq {
    (-1) => {
        -Squared::One
    };
    (0) => {
        Squared::Zero
    };
    (1) => {
        Squared::One
    };
}

#[cfg(test)]
mod tests {
    use crate::geometry::*;

    #[test]
    fn test_blade() {
        let pga2d = SquareMap(
            [(ax!(e0), sq!(0)), (ax!(e1), sq!(1)), (ax!(e2), sq!(1))]
                .into_iter()
                .collect(),
            bl!(e012),
        );
        macro_rules! bl_test {
            ($a:ident == $b:expr) => {
                let a = bl!($a);
                let b = bl!($b);
                assert_eq!(a, b)
            };
            ($a:ident * $b:ident == $c:expr) => {
                let a = bl!($a);
                let b = bl!($b);
                let c = bl!($c);
                assert_eq!((a * b).hone(&pga2d), c)
            };
        }

        bl_test!(e01 == -e10);
        bl_test!(e10 == -e01);
        bl_test!(e02 == -e20);
        bl_test!(e20 == -e02);
        bl_test!(e12 == -e21);
        bl_test!(e21 == -e12);
        bl_test!(e012 == -e210);

        bl_test!(e0 * e0 == 0);
        bl_test!(e0 * e1 == e01);
        bl_test!(e0 * e2 == e02);
        bl_test!(e1 * e0 == -e01);
        bl_test!(e1 * e1 == 1);
        bl_test!(e1 * e2 == e12);
        bl_test!(e2 * e0 == -e02);
        bl_test!(e2 * e1 == -e12);
        bl_test!(e2 * e2 == 1);

        bl_test!(e0 * e01 == 0);
        bl_test!(e0 * e02 == 0);
        bl_test!(e0 * e12 == e012);
        bl_test!(e0 * e10 == 0);
        bl_test!(e0 * e20 == 0);
        bl_test!(e0 * e21 == -e012);
        bl_test!(e1 * e01 == -e0);
        bl_test!(e1 * e02 == -e012);
        bl_test!(e1 * e12 == e2);
        bl_test!(e1 * e10 == e0);
        bl_test!(e1 * e20 == e012);
        bl_test!(e1 * e21 == -e2);
        bl_test!(e2 * e01 == e012);
        bl_test!(e2 * e02 == -e0);
        bl_test!(e2 * e12 == -e1);
        bl_test!(e2 * e10 == -e012);
        bl_test!(e2 * e20 == e0);
        bl_test!(e2 * e21 == e1);

        bl_test!(e01 * e0 == 0);
        bl_test!(e02 * e0 == 0);
        bl_test!(e12 * e0 == e012);
        bl_test!(e10 * e0 == 0);
        bl_test!(e20 * e0 == 0);
        bl_test!(e21 * e0 == -e012);
        bl_test!(e01 * e1 == e0);
        bl_test!(e02 * e1 == -e012);
        bl_test!(e12 * e1 == -e2);
        bl_test!(e10 * e1 == -e0);
        bl_test!(e20 * e1 == e012);
        bl_test!(e21 * e1 == e2);
        bl_test!(e01 * e2 == e012);
        bl_test!(e02 * e2 == e0);
        bl_test!(e12 * e2 == e1);
        bl_test!(e10 * e2 == -e012);
        bl_test!(e20 * e2 == -e0);
        bl_test!(e21 * e2 == -e1);

        bl_test!(e01 * e01 == 0);
        bl_test!(e01 * e02 == 0);
        bl_test!(e01 * e12 == e02);
        bl_test!(e02 * e01 == 0);
        bl_test!(e02 * e02 == 0);
        bl_test!(e02 * e12 == -e01);
        bl_test!(e12 * e01 == -e02);
        bl_test!(e12 * e02 == e01);
        bl_test!(e12 * e12 == -1);
    }
}
