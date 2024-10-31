use crate::{
    basis::{Basis, ZeroVect},
    collector::{CollectInto, Collector},
    field::Field,
    metric::Metric,
    mvect::{basis_set::BasisSet, Mvect},
    ta,
    utils::{
        contains::{IdxOf, IndexOf},
        typeset::{Union, UnionMerge},
    },
};
use core::ops::{Add, Sub};
use generic_array::ArrayLength;
use typenum::{Len, Unsigned, B0, B1, U0};

// --------------------------------------------
// MvAdd - add two multivectors
struct MvAdd;
// Collect the results of adding two multivectors
impl<F: Field> Collector<F, &mut [F]> for MvAdd {
    #[inline(always)]
    fn collect_both<'a>(out: &'a mut [F], left: &F, right: &F) -> &'a mut [F] {
        out[0] += left.clone();
        out[0] += right.clone();
        &mut out[1..]
    }
    #[inline(always)]
    fn collect_just_left<'a>(out: &'a mut [F], left: &F) -> &'a mut [F] {
        out[0] += left.clone();
        &mut out[1..]
    }
    #[inline(always)]
    fn collect_just_right<'a>(out: &'a mut [F], right: &F) -> &'a mut [F] {
        out[0] += right.clone();
        &mut out[1..]
    }
}
// &mv + &mv
impl<
        LBS: BasisSet<M>
            + Len<Output: ArrayLength>
            + UnionMerge<RBS, Output: BasisSet<M> + Len<Output: ArrayLength>>,
        RBS: BasisSet<M> + Len<Output: ArrayLength>,
        M: Metric,
        F: Field + for<'a> CollectInto<F, MvAdd, &'a mut [F], LBS, RBS>,
    > Add<&Mvect<RBS, M, F>> for &Mvect<LBS, M, F>
{
    type Output = Mvect<Union<LBS, RBS>, M, F>;
    #[inline(always)]
    fn add(self, rhs: &Mvect<RBS, M, F>) -> Self::Output {
        let mut out = Self::Output::default();
        MvAdd::do_collect::<LBS, RBS>(&mut out.0, &self.0, &rhs.0);
        out
    }
}
// mv + mv
impl<
        LBS: BasisSet<M>
            + Len<Output: ArrayLength>
            + UnionMerge<RBS, Output: BasisSet<M> + Len<Output: ArrayLength>>,
        RBS: BasisSet<M> + Len<Output: ArrayLength>,
        M: Metric,
        F: Field + for<'a> CollectInto<F, MvAdd, &'a mut [F], LBS, RBS>,
    > Add<Mvect<RBS, M, F>> for Mvect<LBS, M, F>
{
    type Output = Mvect<Union<LBS, RBS>, M, F>;
    #[inline(always)]
    fn add(self, rhs: Mvect<RBS, M, F>) -> Self::Output {
        &self + &rhs
    }
}
// --------------------------------------------
// Sub - subtract two multivectors
struct MvSub;
// Collect the results of subtracting two multivectors
impl<F: Field> Collector<F, &mut [F]> for MvSub {
    #[inline(always)]
    fn collect_both<'a>(out: &'a mut [F], left: &F, right: &F) -> &'a mut [F] {
        out[0] += left.clone();
        out[0] -= right.clone();
        &mut out[1..]
    }
    #[inline(always)]
    fn collect_just_left<'a>(out: &'a mut [F], left: &F) -> &'a mut [F] {
        out[0] += left.clone();
        &mut out[1..]
    }
    #[inline(always)]
    fn collect_just_right<'a>(out: &'a mut [F], right: &F) -> &'a mut [F] {
        out[0] -= right.clone();
        &mut out[1..]
    }
}
impl<
        LBS: BasisSet<M>
            + Len<Output: ArrayLength>
            + UnionMerge<RBS, Output: BasisSet<M> + Len<Output: ArrayLength>>,
        RBS: BasisSet<M> + Len<Output: ArrayLength>,
        M: Metric,
        F: Field + for<'a> CollectInto<F, MvSub, &'a mut [F], LBS, RBS>,
    > Sub<&Mvect<RBS, M, F>> for &Mvect<LBS, M, F>
{
    type Output = Mvect<Union<LBS, RBS>, M, F>;
    #[inline(always)]
    fn sub(self, rhs: &Mvect<RBS, M, F>) -> Self::Output {
        let mut out = Self::Output::default();
        MvSub::do_collect::<LBS, RBS>(&mut out.0, &self.0, &rhs.0);
        out
    }
}
impl<
        LBS: BasisSet<M>
            + Len<Output: ArrayLength>
            + UnionMerge<RBS, Output: BasisSet<M> + Len<Output: ArrayLength>>,
        RBS: BasisSet<M> + Len<Output: ArrayLength>,
        M: Metric,
        F: Field + for<'a> CollectInto<F, MvSub, &'a mut [F], LBS, RBS>,
    > Sub<Mvect<RBS, M, F>> for Mvect<LBS, M, F>
{
    type Output = Mvect<Union<LBS, RBS>, M, F>;
    #[inline(always)]
    fn sub(self, rhs: Mvect<RBS, M, F>) -> Self::Output {
        &self - &rhs
    }
}
// -------------------------------------------------------------------------------------
// ZeroVect + Field
impl<M: Metric, F: Field> Add<F> for ZeroVect<M> {
    type Output = Mvect<ta![U0], M, F>;
    #[inline(always)]
    fn add(self, rhs: F) -> Self::Output {
        let mut out = Mvect::<ta![U0], M, F>::default();
        out.0[0] = rhs;
        out
    }
}
impl<M: Metric, F: Field> Sub<F> for ZeroVect<M> {
    type Output = Mvect<ta![U0], M, F>;
    #[inline(always)]
    fn sub(self, rhs: F) -> Self::Output {
        let mut out = Mvect::<ta![U0], M, F>::default();
        out.0[0] -= rhs;
        out
    }
}
// Basis + Field
impl<U: Unsigned, M: Metric, F: Field> Add<F> for Basis<U, M, B0>
where
    ta![U0]: UnionMerge<ta![U], Output: BasisSet<M> + Len<Output: ArrayLength> + IndexOf<U>>,
{
    type Output = Mvect<Union<ta![U0], ta![U]>, M, F>;
    #[inline(always)]
    fn add(self, rhs: F) -> Self::Output {
        let mut out = Self::Output::default();
        out.0[IdxOf::<Union<ta![U0], ta![U]>, U>::USIZE] += F::one();
        out.0[0] += rhs;
        out
    }
}
impl<U: Unsigned, M: Metric, F: Field> Add<F> for Basis<U, M, B1>
where
    ta![U0]: UnionMerge<ta![U], Output: BasisSet<M> + Len<Output: ArrayLength> + IndexOf<U>>,
{
    type Output = Mvect<Union<ta![U0], ta![U]>, M, F>;
    #[inline(always)]
    fn add(self, rhs: F) -> Self::Output {
        let mut out = Self::Output::default();
        out.0[IdxOf::<Union<ta![U0], ta![U]>, U>::USIZE] -= F::one();
        out.0[0] += rhs;
        out
    }
}
impl<U: Unsigned, M: Metric, F: Field> Sub<F> for Basis<U, M, B0>
where
    ta![U0]: UnionMerge<ta![U], Output: BasisSet<M> + Len<Output: ArrayLength> + IndexOf<U>>,
{
    type Output = Mvect<Union<ta![U0], ta![U]>, M, F>;
    #[inline(always)]
    fn sub(self, rhs: F) -> Self::Output {
        let mut out = Self::Output::default();
        out.0[IdxOf::<Union<ta![U0], ta![U]>, U>::USIZE] += F::one();
        out.0[0] -= rhs;
        out
    }
}
impl<U: Unsigned, M: Metric, F: Field> Sub<F> for Basis<U, M, B1>
where
    ta![U0]: UnionMerge<ta![U], Output: BasisSet<M> + Len<Output: ArrayLength> + IndexOf<U>>,
{
    type Output = Mvect<Union<ta![U0], ta![U]>, M, F>;
    #[inline(always)]
    fn sub(self, rhs: F) -> Self::Output {
        let mut out = Self::Output::default();
        out.0[IdxOf::<Union<ta![U0], ta![U]>, U>::USIZE] -= F::one();
        out.0[0] -= rhs;
        out
    }
}
