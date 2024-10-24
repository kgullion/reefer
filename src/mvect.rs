#![allow(unused_imports)]
use crate::{
    basis::{BNew, BProd, Basis, BasisCart, BasisInfo, IntoBasis, ZeroVector},
    collector::{CartCollector, CollectInto, Collector},
    field::Field,
    metric::{IntFromSwapParityWithOverlaps, Metric},
    utils::{
        typeset::{Intersect, IntersectMerge, Sort, Sorted, Union, UnionMerge},
        At, Branch, Contains, Count, CountOf, Flat, Flatten, Get, IdxOf, If, IndexOf, SwapPar,
    },
};
use core::marker::PhantomData;
use core::ops::{Add, BitAnd, BitOr, BitXor, Mul, Not, Sub};
use generic_array::{ArrayLength, GenericArray};
use typenum::{
    tarr, ATerm, Abs, Add1, And, Bit, Eq, IsEqual, IsNotEqual, Len, Length, NotEq, Or, Prod, Sum,
    TArr, TypeArray, Unsigned, Xor, B0, B1, U0, U1,
};

/// multivector
#[derive(Clone)]
pub struct Mvect<BS: BasisSet<M> + Len<Output: ArrayLength>, M: Metric, F: Field>(
    GenericArray<F, Length<BS>>,
    PhantomData<M>,
);

/// BasisSet stores the Bitmask of the Basis elements that are present in the multivector. Together with the metric, this is enough to recover each Basis.
pub trait BasisSet<M: Metric>: TypeArray {
    type Output;
}
impl<M: Metric> BasisSet<M> for ATerm {
    type Output = ATerm;
}
impl<BS: BasisSet<M> + Len<Output: Unsigned + ArrayLength + Add<B1>>, U: Unsigned, M: Metric>
    BasisSet<M> for TArr<U, BS>
where
    Basis<U, M, B0>: BasisInfo,
{
    type Output = TArr<U, BS>;
}
impl<BS: BasisSet<M> + Len<Output: Unsigned + ArrayLength>, M: Metric, F: Field> Mvect<BS, M, F> {
    /// Create a new multivector from a GenericArray of field elements.
    #[inline(always)]
    pub fn new(data: GenericArray<F, Length<BS>>) -> Self {
        Mvect(data, PhantomData)
    }
    /// Length of the multivector
    #[inline(always)]
    pub fn len(&self) -> usize {
        self.0.len()
    }
}
pub trait IntoBasisSet {
    type Output;
}
impl IntoBasisSet for ZeroVector {
    type Output = tarr![];
}
impl<U: Unsigned + CountOf<B1> + Add<B1>, M: Metric, S: Bit> IntoBasisSet for Basis<U, M, S>
where
    Basis<U, M, S>: BasisInfo,
{
    type Output = tarr![U];
}

// --------------------------------------------
trait MvInfo {
    type BasisSet: BasisSet<Self::Metric>;
    type Field: Field;
    type Metric: Metric;
}
impl<BS: BasisSet<M> + Len<Output: ArrayLength>, M: Metric, F: Field> MvInfo for Mvect<BS, M, F> {
    type BasisSet = BS;
    type Field = F;
    type Metric = M;
}

// --------------------------------------------
// Default - create a new multivector with all elements set to zero
impl<BS: BasisSet<M> + Len<Output: ArrayLength>, M: Metric, F: Field> core::default::Default
    for Mvect<BS, M, F>
// where
//     Self: MvectInfo,
{
    /// Create a new multivector from a GenericArray of field elements.
    #[inline(always)]
    fn default() -> Self {
        Mvect(GenericArray::<F, Length<BS>>::default(), PhantomData)
    }
}
// --------------------------------------------
// IntoMv - convert a Basis or ZeroVector type into a Mvect instance
pub trait IntoMv<F: Field> {
    type Output;
    fn into_mv() -> Self::Output;
}
impl<F: Field, U: Unsigned, M: Metric, S: Bit> IntoMv<F> for Basis<U, M, S>
where
    Basis<U, M, S>: BasisInfo,
    Basis<U, M, B0>: BasisInfo,
{
    type Output = Mvect<tarr![U], M, F>;
    fn into_mv() -> Self::Output {
        let mut out = Mvect::<tarr![U], M, F>::default();
        if S::BOOL {
            out.0[0] = -F::one();
        } else {
            out.0[0] = F::one();
        }
        out
    }
}
// --------------------------------------------
// PartialEq - compare two multivectors
struct MvPartialEq;
// Collect the results of comparing two multivectors
impl<'a, F: Field> Collector<F, bool> for MvPartialEq {
    fn collect_both(out: bool, left: &F, right: &F) -> bool {
        out && left == right
    }
    fn collect_just_left(out: bool, left: &F) -> bool {
        out && left == &F::zero()
    }
    fn collect_just_right(out: bool, right: &F) -> bool {
        out && &F::zero() == right
    }
}
// PartialEq
impl<
        BS: BasisSet<M> + Len<Output: ArrayLength>,
        RBS: BasisSet<M> + Len<Output: ArrayLength>,
        M: Metric,
        F: Field + CollectInto<F, MvPartialEq, bool, BS, RBS>,
    > core::cmp::PartialEq<Mvect<RBS, M, F>> for Mvect<BS, M, F>
{
    fn eq(&self, other: &Mvect<RBS, M, F>) -> bool {
        MvPartialEq::do_collect::<BS, RBS>(true, &self.0, &other.0)
    }
}
// Implement Total Eq for Fields that support it
impl<
        BS: BasisSet<M> + Len<Output: ArrayLength>,
        M: Metric,
        F: core::cmp::Eq + Field + CollectInto<F, MvPartialEq, bool, BS, BS>,
    > core::cmp::Eq for Mvect<BS, M, F>
{
}
// --------------------------------------------
// MvAdd - add two multivectors
struct MvAdd;
// Collect the results of adding two multivectors
impl<F: Field> Collector<F, &mut [F]> for MvAdd {
    fn collect_both<'a>(out: &'a mut [F], left: &F, right: &F) -> &'a mut [F] {
        out[0] += left.clone();
        out[0] += right.clone();
        &mut out[1..]
    }
    fn collect_just_left<'a>(out: &'a mut [F], left: &F) -> &'a mut [F] {
        out[0] += left.clone();
        &mut out[1..]
    }
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
    fn add(self, rhs: Mvect<RBS, M, F>) -> Self::Output {
        &self + &rhs
    }
}
// --------------------------------------------
// Sub - subtract two multivectors
struct MvSub;
// Collect the results of subtracting two multivectors
impl<F: Field> Collector<F, &mut [F]> for MvSub {
    fn collect_both<'a>(out: &'a mut [F], left: &F, right: &F) -> &'a mut [F] {
        out[0] -= left.clone();
        out[0] -= right.clone();
        &mut out[1..]
    }
    fn collect_just_left<'a>(out: &'a mut [F], left: &F) -> &'a mut [F] {
        out[0] -= left.clone();
        &mut out[1..]
    }
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
    fn sub(self, rhs: Mvect<RBS, M, F>) -> Self::Output {
        &self - &rhs
    }
}
// --------------------------------------------
// multivector multiplication
// ----
// MvMul - does the runtime work
pub trait MvMul<F, OUT, A: BasisSet<Self>, B: BasisSet<Self>>: Metric + Sized {
    fn mv_mul(out: &mut [F], left: &[F], right: &[F]);
}
impl<B: BasisSet<M>, OUT: BasisSet<M>, M: Metric, F: Field> MvMul<F, OUT, tarr![], B> for M {
    fn mv_mul(_out: &mut [F], _left: &[F], _right: &[F]) {}
}
impl<
        L: Unsigned,
        A: BasisSet<M> + Len<Output: ArrayLength>,
        B: BasisSet<M> + Len<Output: ArrayLength>,
        OUT: BasisSet<M> + Len<Output: ArrayLength>,
        M: Metric + MvMulInner<F, OUT, L, B> + MvMul<F, OUT, A, B>,
        F: Field,
    > MvMul<F, OUT, TArr<L, A>, B> for M
where
    TArr<L, A>: BasisSet<M>,
{
    fn mv_mul(out: &mut [F], left: &[F], right: &[F]) {
        <M as MvMulInner<F, OUT, L, B>>::mv_mul_inner(out, &left[0], right);
        <M as MvMul<F, OUT, A, B>>::mv_mul(out, &left[1..], right);
    }
}
// MvMulInner
pub trait MvMulInner<F, OUT, L, B>: Metric {
    fn mv_mul_inner(out: &mut [F], left: &F, right: &[F]);
}
impl<L: Unsigned, OUT: BasisSet<M>, M: Metric, F: Field> MvMulInner<F, OUT, L, tarr![]> for M {
    fn mv_mul_inner(_out: &mut [F], _left: &F, _right: &[F]) {}
}
impl<
        L: Unsigned,
        R: Unsigned,
        B: BasisSet<M> + Len<Output: ArrayLength>,
        OUT: BasisSet<M> + Len<Output: ArrayLength>,
        M: Metric + MvMulInner<F, OUT, L, B>,
        F: Field,
    > MvMulInner<F, OUT, L, TArr<R, B>> for M
where
    Basis<L, M, B0>: BasisInfo + Mul<Basis<R, M, B0>, Output: CartCollector<F, OUT>>,
    Basis<R, M, B0>: BasisInfo,
{
    fn mv_mul_inner(out: &mut [F], left: &F, right: &[F]) {
        <Prod<Basis<L, M, B0>, Basis<R, M, B0>> as CartCollector<F, OUT>>::collect(
            out, left, &right[0],
        );
        <M as MvMulInner<F, OUT, L, B>>::mv_mul_inner(out, left, &right[1..]);
    }
}
// ----
// ProdType - does the comptime work
pub trait ProdType<A: BasisSet<Self>, B: BasisSet<Self>>: Metric + Sized {
    type Output: BasisSet<Self>;
}
// 0 * B = 0
impl<B: BasisSet<M>, M: Metric> ProdType<tarr![], B> for M {
    type Output = tarr![];
}
// [L|A] * B = A*B + L*B
impl<
        L: Unsigned,
        A: BasisSet<M> + Len<Output: ArrayLength>,
        B: BasisSet<M> + Len<Output: ArrayLength>,
        M: Metric + ProdType<A, B> + ProdTypeInner<L, B>,
    > ProdType<TArr<L, A>, B> for M
where
    TArr<L, A>: BasisSet<M>,
    <M as ProdType<A, B>>::Output:
        UnionMerge<<M as ProdTypeInner<L, B>>::Output, Output: BasisSet<M>>,
    <M as ProdTypeInner<L, B>>::Output: BasisSet<M>,
{
    type Output = Union<<M as ProdType<A, B>>::Output, <M as ProdTypeInner<L, B>>::Output>;
}
/// ProdTypeInner - does the compile-time type work
pub trait ProdTypeInner<L: Unsigned, B: TypeArray> {
    type Output: TypeArray;
}
// L*0 = 0
impl<L: Unsigned, M: Metric> ProdTypeInner<L, tarr![]> for M {
    type Output = tarr![];
}
// L*[R|B] = L*R + L*B
impl<L: Unsigned, R: Unsigned, B: BasisSet<M>, M: Metric + ProdTypeInner<L, B>>
    ProdTypeInner<L, TArr<R, B>> for M
where
    Basis<L, M, B0>: BasisInfo + Mul<Basis<R, M, B0>, Output: IntoBasisSet<Output: BasisSet<M>>>,
    Basis<R, M, B0>: BasisInfo,
    <Prod<Basis<L, M, B0>, Basis<R, M, B0>> as IntoBasisSet>::Output:
        UnionMerge<<M as ProdTypeInner<L, B>>::Output>,
{
    type Output = Union<
        <Prod<Basis<L, M, B0>, Basis<R, M, B0>> as IntoBasisSet>::Output,
        <M as ProdTypeInner<L, B>>::Output,
    >;
}
// ----
// Mul - multiply two multivectors 🎉
impl<
        A: BasisSet<M> + Len<Output: ArrayLength>,
        B: BasisSet<M> + Len<Output: ArrayLength>,
        M: Metric
            + ProdType<A, B, Output: BasisSet<M> + Len<Output: ArrayLength>>
            + MvMul<F, <M as ProdType<A, B>>::Output, A, B>,
        F: Field,
    > core::ops::Mul<Mvect<B, M, F>> for Mvect<A, M, F>
{
    type Output = Mvect<<M as ProdType<A, B>>::Output, M, F>;
    fn mul(self, rhs: Mvect<B, M, F>) -> Self::Output {
        let mut out = Self::Output::default();
        <M as MvMul<F, <M as ProdType<A, B>>::Output, A, B>>::mv_mul(&mut out.0, &self.0, &rhs.0);
        out
    }
}

// tests
#[cfg(test)]
mod tests {
    use super::*;
    use typenum::{tarr, B0, B1, P1, U0, U1, U2, U3, U4, U5, U6, U7, Z0};

    type Metric = tarr![Z0, P1, P1];
    type Pga2d<U> = Basis<U, Metric, B0>;

    type Scalar = Pga2d<U0>;
    type E0 = Pga2d<U1>;
    type E1 = Pga2d<U2>;
    type E01 = Pga2d<U3>;
    type E2 = Pga2d<U4>;
    type E02 = Pga2d<U5>;
    type E12 = Pga2d<U6>;
    type E012 = Pga2d<U7>;

    #[test]
    fn test_default() {
        type M = tarr![Z0, P1, P1];
        type BS = <tarr![U0, U1, U2, U4] as BasisSet<M>>::Output;
        let mv = Mvect::<BS, M, f32>::default();
        assert_eq!(mv.len(), 4);
        for &elem in mv.0.iter() {
            assert_eq!(elem, 0.0);
        }
    }

    #[test]
    fn test_into_mv() {
        let expected = Mvect::<tarr![U3], Metric, f32>::new(GenericArray::<f32, U1>::from([1.0]));
        let actual = <E01 as IntoMv<f32>>::into_mv();
        assert!(expected == actual);
    }

    #[test]
    fn test_eq() {
        let mv0 = <E01 as IntoMv<f32>>::into_mv();
        let mv1 = <E01 as IntoMv<f32>>::into_mv();
        assert!(mv0 == mv1);
        // TODO: test zeros vs not stored vals
    }

    #[test]
    fn test_add() {
        let e = <Scalar as IntoMv<f32>>::into_mv();
        let e0 = <E0 as IntoMv<f32>>::into_mv();

        let c = e + e0;
    }

    #[test]
    fn test_mul() {
        let e = <Scalar as IntoMv<f32>>::into_mv();
        let e1 = <E1 as IntoMv<f32>>::into_mv();

        let c = e * e1;
    }
}
