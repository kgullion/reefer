use crate::{
    basis::dual::{DualPar, DualParity},
    field::Field,
    metric::Metric,
    mvect::{basis_set::BasisSet, Mvect},
    ta,
    traits::{Dual, Undual},
    utils::reverse::Reverse,
};
use core::ops;
use generic_array::ArrayLength;
use typenum::{Bit, Len, TypeArray, Unsigned, Xor};

// -------------------------------------------------------------------------------------
// Dual
impl<
        A: BasisSet<M>
            + Len<Output: ArrayLength>
            + DualBs<M, F, Output: BasisSet<M> + Len<Output: ArrayLength>>
            + DualBsFlipper<M, F>,
        M: Metric,
        F: Field + core::fmt::Debug,
    > Dual for Mvect<A, M, F>
{
    type Output = Mvect<<A as DualBs<M, F>>::Output, M, F>;
    #[inline(always)]
    fn dual(self) -> Self::Output {
        let mut out = Self::Output::default();
        let data: &mut [F] = &mut out.0;
        data.clone_from_slice(&self.0);
        <A as DualBsFlipper<M, F>>::flip(data);
        data.reverse();
        out
    }
}

// -------------------------------------------------------------------------------------
// Undual
impl<
        A: BasisSet<M>
            + Len<Output: ArrayLength>
            + DualBs<M, F, Output: BasisSet<M> + Len<Output: ArrayLength>>
            + DualBsFlipper<M, F>,
        M: Metric,
        F: Field + core::fmt::Debug,
    > Undual for Mvect<A, M, F>
{
    type Output = Mvect<<A as DualBs<M, F>>::Output, M, F>;
    #[inline(always)]
    fn undual(self) -> Self::Output {
        let mut out = Self::Output::default();
        let data: &mut [F] = &mut out.0;
        data.clone_from_slice(&self.0);
        data.reverse();
        <A as DualBsFlipper<M, F>>::flip(data);
        out
    }
}

// ----
pub trait DualBs<M: Metric, F: Field>: BasisSet<M> {
    type Output: BasisSet<M>;
}
impl<A: BasisSet<M> + DualMap<M, Output: Reverse<Output: BasisSet<M>>>, M: Metric, F: Field>
    DualBs<M, F> for A
{
    type Output = <<A as DualMap<M>>::Output as Reverse>::Output;
}

// ----
pub trait DualBsFlipper<M: Metric, F: Field>: BasisSet<M> {
    fn flip(data: &mut [F]);
}
impl<M: Metric, F: Field> DualBsFlipper<M, F> for ta![] {
    #[inline(always)]
    fn flip(_data: &mut [F]) {}
}
impl<U: Unsigned, A: BasisSet<M>, M: Metric, F: Field> DualBsFlipper<M, F> for ta![U | A]
where
    ta![U | A]: BasisSet<M>,
    A: DualBsFlipper<M, F>,
    U: DualPar<M>,
    F: core::fmt::Debug,
{
    #[inline(always)]
    fn flip(data: &mut [F]) {
        if DualParity::<U, M>::BOOL {
            data[0] = -data[0].clone();
        }
        <A as DualBsFlipper<M, F>>::flip(&mut data[1..]);
    }
}

// ----
pub trait DualMap<M: Metric>: TypeArray {
    type Output: TypeArray;
}
impl<M: Metric> DualMap<M> for ta![] {
    type Output = ta![];
}
impl<U: Unsigned + ops::BitXor<M::Psuedoscalar>, A: DualMap<M>, M: Metric> DualMap<M>
    for ta![U | A]
{
    type Output = ta![Xor<U, M::Psuedoscalar> | A::Output];
}

// tests
#[cfg(test)]
mod tests {
    use crate::{
        traits::{Dual, Undual},
        vga3d::{scalar as c, x, xy, xyz, xz, y, yz, z},
    };

    #[test]
    fn test_dual_vga() {
        // !(1 + 2x + 3y + 5z + 7xy + 11xz + 13yz + 17xyz)
        // = -17 - 13x + 11y - 7z + 5xy - 3xz + 2yz + 1xyz
        let a =
            1.0 * c + 2.0 * x + 3.0 * y + 5.0 * z + 7.0 * xy + 11.0 * xz + 13.0 * yz + 17.0 * xyz;

        let expected =
            -17.0 * c - 13.0 * x + 11.0 * y - 7.0 * z + 5.0 * xy - 3.0 * xz + 2.0 * yz + 1.0 * xyz;
        let actual = a.dual();

        println!("a = {}", a);
        println!("expected = {}", expected);
        println!("actual   = {}", actual);
        println!("diff     = {}", expected - actual);

        assert!(expected == actual);
    }
    #[test]
    fn test_undual_vga() {
        let a =
            1.0 * c + 2.0 * x + 3.0 * y + 5.0 * z + 7.0 * xy + 11.0 * xz + 13.0 * yz + 17.0 * xyz;
        assert!(a.dual().undual() == a);
    }
}
