use super::{Blade, Canon, CanonMap, ConstOne, Mvect, One, Shape, SquareMap, Squared, Zero};
use crate::{
    build::BladeValue,
    cas::CasExpr,
    err,
    geometry::{Battery, Blunt},
    traits::Squareroot,
};
use itertools::Itertools;
use proc_macro2::Span;
use quote::format_ident;
use std::{
    collections::btree_map::{self, Entry},
    fmt::{Debug, Display},
    ops::{Add, AddAssign, DivAssign, Mul, Neg},
};

impl Mvect<'_, CasExpr> {
    pub fn simplify(self) -> Self {
        Self(
            self.0
                .into_iter()
                .map(|(c, v)| (c, v.simplify()))
                .filter(|(_, v)| v != &CasExpr::zero())
                .collect(),
            self.1,
        )
    }
    pub fn try_into_expr(self, batteries: &CanonMap, span: Span) -> syn::Result<syn::Expr> {
        let mut mv = self;
        let sq = mv.1;
        let shape = mv.try_into_shape(batteries, span)?;
        let mut expr = syn::ExprStruct {
            attrs: vec![],
            qself: None,
            path: format_ident!("{shape}").into(),
            brace_token: Default::default(),
            fields: Default::default(),
            dot2_token: None,
            rest: None,
        };
        for blank in shape {
            expr.fields.push(syn::FieldValue {
                attrs: vec![],
                member: syn::Member::Named(format_ident!("{blank}")),
                colon_token: Some(Default::default()),
                expr: match Blunt::hone(blank.into(), sq) {
                    Blade::Zero => unreachable!(),
                    Blade::Pos(canon) => match mv.0.remove(&canon) {
                        Some(cas_expr) => syn::Expr::try_from(cas_expr),
                        None => syn::Expr::try_from(CasExpr::zero()),
                    },
                    Blade::Neg(canon) => match mv.0.remove(&canon) {
                        Some(cas_expr) => syn::Expr::try_from(-cas_expr),
                        None => syn::Expr::try_from(CasExpr::zero()),
                    },
                }?,
            })
        }
        Ok(syn::Expr::Struct(expr))
    }
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}
impl<T: Display> Display for Mvect<'_, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(
            self.0
                .iter()
                .map(|(canon, value)| format!("{} * {}", value, Blade::from(canon.clone())))
                .join(" + ")
                .as_str(),
        )
    }
}
impl<T: Clone + Neg<Output = T> + Debug + Display> Mvect<'_, T> {
    pub fn get(&self, blade: &Blade) -> Option<T> {
        match blade {
            Blade::Zero => None,
            Blade::Pos(canon) => self.0.get(canon).cloned(),
            Blade::Neg(canon) => self.0.get(canon).cloned().map(Neg::neg),
        }
    }
    pub fn take(&mut self, blade: &Blade) -> Option<T> {
        match blade {
            Blade::Zero => None,
            Blade::Pos(canon) => self.0.remove(canon),
            Blade::Neg(canon) => self.0.remove(canon).map(Neg::neg),
        }
    }
    pub fn try_into_shape(&self, batts: &CanonMap, span: Span) -> syn::Result<Shape> {
        if self.0.is_empty() {
            return Ok(Shape(vec![]));
        }
        let batt: Battery = self.0.keys().cloned().collect();
        batts
            .0
            .get(&batt)
            .cloned()
            .ok_or(err!(span, format!("shape not found: {batt}")))
    }
}

impl<
    T: Clone
        + Debug
        + AddAssign
        + DivAssign
        + Add<Output = T>
        + Mul<Output = T>
        + Squareroot<Output = T>
        + Neg<Output = T>
        + Zero
        + One
        + Squareroot<Output = T>
        + Display,
> Mvect<'_, T>
{
    pub fn add(self, rhs: Self) -> Self {
        rhs.into_iter()
            .fold(self, |mv, (canon, value)| mv.add_canon_value(canon, value))
    }
    pub fn sub(self, rhs: Self) -> Self {
        self.add(rhs.neg())
    }
    pub fn mul(self, other: Self) -> Self {
        debug_assert!(self.1 as *const _ == other.1 as *const _);
        let sq = self.1;
        self.into_iter()
            .cartesian_product(other.into_iter().collect_vec())
            .fold(Self(Default::default(), sq), |mv, ((lc, lv), (rc, rv))| {
                mv.add_blade_value((lc * rc).hone(sq), lv * rv)
            })
    }
    pub fn commutate(self, other: Self) -> Self {
        debug_assert!(self.1 as *const _ == other.1 as *const _);
        let sq = self.1;
        self.into_iter()
            .cartesian_product(other.into_iter().collect_vec())
            .fold(Self(Default::default(), sq), |mv, ((lc, lv), (rc, rv))| {
                let fwd = lc.clone().mul(rc.clone());
                let rev = rc.mul(lc);
                if fwd != rev {
                    mv.add_blade_value(fwd.hone(sq), lv * rv)
                } else {
                    mv
                }
            })
    }
    pub fn anticomm(self, other: Self) -> Self {
        debug_assert!(self.1 as *const _ == other.1 as *const _);
        let sq = self.1;
        self.into_iter()
            .cartesian_product(other.into_iter().collect_vec())
            .fold(Self(Default::default(), sq), |mv, ((lc, lv), (rc, rv))| {
                let fwd = lc.clone().mul(rc.clone());
                let rev = rc.mul(lc);
                if fwd == rev {
                    mv.add_blade_value(fwd.hone(sq), lv * rv)
                } else {
                    mv
                }
            })
    }
    pub fn sandwich(self, rhs: Self) -> Self {
        self.clone().mul(rhs).mul(self.rev())
    }
    pub fn regressive(self, rhs: Self) -> Self {
        let ps = &self.1.1;
        self.dual(ps.clone())
            .wedge(rhs.dual(ps.clone()))
            .undual(ps.clone())
    }
    pub fn wedge(self, rhs: Self) -> Self {
        self.graded_product(rhs, |l, r| Some(l + r))
    }
    pub fn dot(self, rhs: Self) -> Self {
        self.graded_product(rhs, |l, r| (l == r).then_some(0))
    }
    pub fn fat_dot(self, rhs: Self) -> Self {
        self.graded_product(rhs, |l, r| Some(if l <= r { r - l } else { l - r }))
    }
    pub fn lcontract(self, rhs: Self) -> Self {
        self.graded_product(rhs, |l, r| if l <= r { Some(r - l) } else { None })
    }
    pub fn rcontract(self, rhs: Self) -> Self {
        self.graded_product(rhs, |l, r| if l >= r { Some(l - r) } else { None })
    }
    pub fn inv(self) -> Self {
        todo!()
    }
    pub fn linv(self) -> Self {
        todo!()
    }
    pub fn div(self, rhs: Self) -> Self {
        self.mul(rhs.inv())
    }
    pub fn ldiv(self, rhs: Self) -> Self {
        self.linv().mul(rhs)
    }
    pub fn exp(self) -> Self {
        todo!()
    }
    pub fn log(self) -> Self {
        todo!()
    }
    pub fn sqrt(self) -> Self {
        if self.0.len() <= 1 {
            Self(
                self.0.into_iter().map(|(c, v)| (c, v.sqrt())).collect(),
                self.1,
            )
        } else {
            todo!()
        }
    }
    pub fn square(self) -> Self {
        self.clone().mul(self)
    }
    pub fn pow(self, n: usize) -> Self {
        match n {
            0 => Self([(Canon::One, T::one())].into_iter().collect(), self.1),
            1 => self,
            2 => self.square(),
            n if n % 2 == 0 => self.pow(n / 2).pow(2),
            n => self.clone().pow(n / 2).pow(2).mul(self),
        }
    }
    pub fn grade(self, n: usize) -> Self {
        Self(
            self.0
                .into_iter()
                .filter(|(canon, _)| canon.0.len() == n)
                .collect(),
            self.1,
        )
    }
    pub fn neg(self) -> Self {
        self.involution(|_| true)
    }
    pub fn aut(self) -> Self {
        self.involution(|canon| canon.0.len() % 2 == 1)
    }
    pub fn rev(self) -> Self {
        self.involution(|canon| canon.0.len() % 4 >= 2)
    }
    pub fn conj(self) -> Self {
        self.involution(|canon| (canon.0.len() + 3) % 4 < 2)
    }
    pub fn normed(mut self) -> Self {
        let n = self._norm();
        self.0.values_mut().for_each(|v| *v /= n.clone());
        self
    }
    pub fn norm(self) -> Self {
        let sq = self.1;
        Self([(Canon::One, self._norm())].into_iter().collect(), sq)
    }
    fn _norm(&self) -> T {
        self.0
            .iter()
            .fold(T::zero(), |norm, (canon, value)| {
                match Blade::from(canon.clone()).square(self.1) {
                    Squared::Zero => norm,
                    _ => norm + value.clone() * value.clone(),
                }
            })
            .sqrt()
    }
    pub fn dual(self, ps: Blade) -> Self {
        self.undual(ps.rev())
    }
    pub fn undual(self, ps: Blade) -> Self {
        self.0
            .into_iter()
            .fold(Self(Default::default(), self.1), |mv, (canon, value)| {
                let bl = Blade::from(canon).mul(ps.clone());
                mv.add_blade_value(bl.dual_hone(self.1), value)
            })
    }
    pub fn ldual(self, ps: Blade) -> Self {
        self.lundual(ps.rev())
    }
    pub fn lundual(self, ps: Blade) -> Self {
        self.0
            .into_iter()
            .fold(Self(Default::default(), self.1), |mv, (canon, value)| {
                let bl = ps.clone().mul(canon.into());
                mv.add_blade_value(bl.dual_hone(self.1), value)
            })
    }
    pub fn add_blade_value(self, blade: Blade, value: T) -> Self {
        match blade {
            Blade::Zero => self,
            Blade::Pos(canon) => self.add_canon_value(canon, value),
            Blade::Neg(canon) => self.add_canon_value(canon, -value),
        }
    }
    fn add_canon_value(mut self, canon: Canon, value: T) -> Self {
        match self.0.entry(canon) {
            Entry::Vacant(entry) => {
                entry.insert(value);
            }
            Entry::Occupied(mut entry) => {
                entry.get_mut().add_assign(value);
            }
        };
        self
    }
    fn involution(mut self, predicate: fn(&Canon) -> bool) -> Self {
        for (canon, value) in self.0.iter_mut() {
            if predicate(canon) {
                *value = -value.clone()
            }
        }
        self
    }
    fn graded_product(self, other: Self, grader: fn(usize, usize) -> Option<usize>) -> Self {
        debug_assert!(self.1 as *const _ == other.1 as *const _);
        let sq = self.1;
        let mut prod = Self(Default::default(), self.1);
        let lgroups = self.group_by_grade();
        let rgroups = other.group_by_grade();
        for ((lg, lbs), (rg, rbs)) in lgroups.into_iter().cartesian_product(rgroups) {
            if let Some(grade) = grader(lg, rg) {
                for ((lc, lv), (rc, rv)) in lbs.into_iter().cartesian_product(rbs) {
                    let partial = (lc * rc).hone(sq);
                    if partial.len() == grade {
                        prod = prod.add_blade_value(partial, lv * rv)
                    }
                }
            };
        }
        prod
    }
}

impl<T: Clone> IntoIterator for Mvect<'_, T> {
    type IntoIter = btree_map::IntoIter<Canon, T>;
    type Item = (Canon, T);
    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<'a> Mvect<'a, CasExpr> {
    pub fn from_iter<T: IntoIterator<Item = BladeValue>>(
        iter: T,
        sq: &'a SquareMap,
    ) -> syn::Result<Self> {
        let mut mv = Self(Default::default(), sq);
        for bv in iter {
            mv = mv.add_blade_value(bv.blade.hone(sq), bv.expr.try_into()?)
        }
        Ok(mv)
    }
}

impl<'a, T: 'a + Clone> Mvect<'a, T> {
    fn group_by_grade(self) -> Vec<(usize, Vec<(Canon, T)>)> {
        self.into_iter()
            .chunk_by(|(canon, _value)| canon.0.len())
            .into_iter()
            .map(|(n, g)| (n, g.collect()))
            .collect()
    }
}
