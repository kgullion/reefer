use crate::{
    basis::{Basis, ZeroVect},
    collector::CartCollector,
    field::Field,
    marker::{
        CommutatorMarker, FatDotMarker, GeoProdMarker, InnerProdMarker, LeftContractionMarker,
        MvMulMarker, OuterProdMarker, RightContractionMarker, ScalarProdMarker,
    },
    metric::Metric,
    mvect::{basis_set::BasisSet, Mvect},
    ta,
    traits::{Commutator, Dual, FatDot, Inverse, Sandwich, ScalarProduct, Undual},
    utils::{
        typeset::{Union, UnionMerge},
        Branch, If,
    },
};
use core::marker::PhantomData;
use generic_array::{ArrayLength, GenericArray};
use typenum::{Bit, Len, Prod, TypeArray, Unsigned, Xor, B0, B1};

// --------------------------------------------
// multivector multiplication
// ----

// MvMul - does the runtime work
pub trait MvMulRun<K, F, OUT, A: BasisSet<Self>, B: BasisSet<Self>>: Metric + Sized {
    fn mv_mul(out: &mut [F], left: &[F], right: &[F]);
}
impl<B: BasisSet<M>, OUT: BasisSet<M>, M: Metric, F: Field, K> MvMulRun<K, F, OUT, ta![], B> for M {
    #[inline(always)]
    fn mv_mul(_out: &mut [F], _left: &[F], _right: &[F]) {}
}
impl<
        L: Unsigned,
        A: BasisSet<M> + Len<Output: ArrayLength>,
        B: BasisSet<M> + Len<Output: ArrayLength>,
        OUT: BasisSet<M> + Len<Output: ArrayLength>,
        M: Metric + MvMulRunInner<K, F, OUT, L, B> + MvMulRun<K, F, OUT, A, B>,
        F: Field,
        K,
    > MvMulRun<K, F, OUT, ta![L | A], B> for M
where
    ta![L | A]: BasisSet<M>,
{
    #[inline(always)]
    fn mv_mul(out: &mut [F], left: &[F], right: &[F]) {
        <M as MvMulRunInner<K, F, OUT, L, B>>::mv_mul_inner(out, &left[0], right);
        <M as MvMulRun<K, F, OUT, A, B>>::mv_mul(out, &left[1..], right);
    }
}
// MvMulInner
pub trait MvMulRunInner<K, F, OUT, L, B>: Metric {
    fn mv_mul_inner(out: &mut [F], left: &F, right: &[F]);
}
impl<L: Unsigned, OUT: BasisSet<M>, M: Metric, F: Field, K> MvMulRunInner<K, F, OUT, L, ta![]>
    for M
{
    #[inline(always)]
    fn mv_mul_inner(_out: &mut [F], _left: &F, _right: &[F]) {}
}
impl<
        L: Unsigned,
        R: Unsigned,
        B: BasisSet<M> + Len<Output: ArrayLength>,
        OUT: BasisSet<M> + Len<Output: ArrayLength>,
        M: Metric + MvMulRunInner<K, F, OUT, L, B>,
        F: Field,
        K: MvMulMarker<L, R>,
    > MvMulRunInner<K, F, OUT, L, ta![R | B]> for M
where
    Basis<L, M, B0>: core::ops::Mul<Basis<R, M, B0>, Output: CartCollector<F, OUT>>,
{
    #[inline(always)]
    fn mv_mul_inner(out: &mut [F], left: &F, right: &[F]) {
        if <K as MvMulMarker<L, R>>::Output::BOOL {
            <Prod<Basis<L, M, B0>, Basis<R, M, B0>> as CartCollector<F, OUT>>::collect(
                out, left, &right[0],
            );
        }
        <M as MvMulRunInner<K, F, OUT, L, B>>::mv_mul_inner(out, left, &right[1..]);
    }
}
// ----
// ProdType - does the comptime work
pub trait MvMulType<K, A: BasisSet<Self>, B: BasisSet<Self>>: Metric + Sized {
    type Output: BasisSet<Self>;
}
// 0 * B = 0
impl<B: BasisSet<M>, M: Metric, K> MvMulType<K, ta![], B> for M {
    type Output = ta![];
}
// [L|A] * B = A*B + L*B
impl<
        L: Unsigned,
        A: BasisSet<M> + Len<Output: ArrayLength>,
        B: BasisSet<M> + Len<Output: ArrayLength>,
        M: Metric + MvMulType<K, A, B> + MvMulTypeInner<K, L, B>,
        K,
    > MvMulType<K, ta![L | A], B> for M
where
    ta![L | A]: BasisSet<M>,
    <M as MvMulType<K, A, B>>::Output:
        UnionMerge<<M as MvMulTypeInner<K, L, B>>::Output, Output: BasisSet<M>>,
    <M as MvMulTypeInner<K, L, B>>::Output: BasisSet<M>,
{
    type Output = Union<<M as MvMulType<K, A, B>>::Output, <M as MvMulTypeInner<K, L, B>>::Output>;
}
/// ProdTypeInner - does the compile-time type work
pub trait MvMulTypeInner<K, L: Unsigned, B: TypeArray> {
    type Output: TypeArray;
}
// L*0 = 0
impl<L: Unsigned, M: Metric, K> MvMulTypeInner<K, L, ta![]> for M {
    type Output = ta![];
}
// L*[R|B] = L*R + L*B
impl<L: Unsigned, R: Unsigned, B: BasisSet<M>, M: Metric + MvMulTypeInner<K, L, B>, K>
    MvMulTypeInner<K, L, ta![R | B]> for M
where
    K: MvMulMarker<L, R>,
    Basis<L, M, B0>: core::ops::Mul<Basis<R, M, B0>, Output: IntoBasisSet<Output: BasisSet<M>>>,
    <K as MvMulMarker<L, R>>::Output: Branch<
        <Prod<Basis<L, M, B0>, Basis<R, M, B0>> as IntoBasisSet>::Output,
        ta![],
        Output: TypeArray,
    >,
    <M as MvMulTypeInner<K, L, B>>::Output: UnionMerge<
        If<
            <K as MvMulMarker<L, R>>::Output,
            <Prod<Basis<L, M, B0>, Basis<R, M, B0>> as IntoBasisSet>::Output,
            ta![],
        >,
    >,
{
    type Output = Union<
        <M as MvMulTypeInner<K, L, B>>::Output,
        If<
            <K as MvMulMarker<L, R>>::Output,
            <Prod<Basis<L, M, B0>, Basis<R, M, B0>> as IntoBasisSet>::Output,
            ta![],
        >,
    >;
}
// --------------------------------------------
/// IntoBasisSet - convert a Basis or ZeroVect<M>or type into a BasisSet
pub trait IntoBasisSet {
    type Output: TypeArray;
}
impl<M: Metric> IntoBasisSet for ZeroVect<M> {
    type Output = ta![];
}
impl<U: Unsigned, M: Metric, S: Bit> IntoBasisSet for Basis<U, M, S> {
    type Output = ta![U];
}
/// internal helper for implementing multiplication operators
#[inline(always)]
fn mv_mul_runner<
    K,
    A: BasisSet<M>,
    B: BasisSet<M>,
    M: Metric
        + MvMulType<K, A, B, Output: BasisSet<M> + Len<Output: ArrayLength>>
        + MvMulRun<K, F, <M as MvMulType<K, A, B>>::Output, A, B>,
    F: Field,
>(
    out: &mut [F],
    left: &[F],
    right: &[F],
) {
    <M as MvMulRun<K, F, <M as MvMulType<K, A, B>>::Output, A, B>>::mv_mul(out, left, right)
}
// ----
// Mul - geometric product of two multivectors
impl<
        A: BasisSet<M> + Len<Output: ArrayLength>,
        B: BasisSet<M> + Len<Output: ArrayLength>,
        M: Metric
            + MvMulType<GeoProdMarker, A, B, Output: BasisSet<M> + Len<Output: ArrayLength>>
            + MvMulRun<GeoProdMarker, F, <M as MvMulType<GeoProdMarker, A, B>>::Output, A, B>,
        F: Field,
    > core::ops::Mul<Mvect<B, M, F>> for Mvect<A, M, F>
{
    type Output = Mvect<<M as MvMulType<GeoProdMarker, A, B>>::Output, M, F>;
    #[inline(always)]
    fn mul(self, rhs: Mvect<B, M, F>) -> Self::Output {
        let mut out = Self::Output::default();
        mv_mul_runner::<GeoProdMarker, A, B, M, F>(&mut out.0, &self.0, &rhs.0);
        out
    }
}
// ----
// Outer Product
impl<
        A: BasisSet<M> + Len<Output: ArrayLength>,
        B: BasisSet<M> + Len<Output: ArrayLength>,
        M: Metric
            + MvMulType<OuterProdMarker, A, B, Output: BasisSet<M> + Len<Output: ArrayLength>>
            + MvMulRun<OuterProdMarker, F, <M as MvMulType<OuterProdMarker, A, B>>::Output, A, B>,
        F: Field,
    > core::ops::BitXor<Mvect<B, M, F>> for Mvect<A, M, F>
{
    type Output = Mvect<<M as MvMulType<OuterProdMarker, A, B>>::Output, M, F>;
    #[inline(always)]
    fn bitxor(self, rhs: Mvect<B, M, F>) -> Self::Output {
        let mut out = Self::Output::default();
        mv_mul_runner::<OuterProdMarker, A, B, M, F>(&mut out.0, &self.0, &rhs.0);
        out
    }
}
// ----
// Regressive Product
impl<
        A: BasisSet<M> + Len<Output: ArrayLength>,
        B: BasisSet<M> + Len<Output: ArrayLength>,
        M: Metric,
        F: Field,
    > core::ops::BitAnd<Mvect<B, M, F>> for Mvect<A, M, F>
where
    Self: Dual<Output: core::ops::BitXor<<Mvect<B, M, F> as Dual>::Output, Output: Undual>>,
    Mvect<B, M, F>: Dual,
{
    type Output = <Xor<<Self as Dual>::Output, <Mvect<B, M, F> as Dual>::Output> as Undual>::Output;
    #[inline(always)]
    fn bitand(self, rhs: Mvect<B, M, F>) -> Self::Output {
        (self.dual() ^ rhs.dual()).undual()
    }
}
// ----
// Commutator Product
impl<
        A: BasisSet<M> + Len<Output: ArrayLength>,
        B: BasisSet<M> + Len<Output: ArrayLength>,
        M: Metric
            + MvMulType<CommutatorMarker, A, B, Output: BasisSet<M> + Len<Output: ArrayLength>>
            + MvMulRun<CommutatorMarker, F, <M as MvMulType<CommutatorMarker, A, B>>::Output, A, B>,
        F: Field,
    > Commutator<Mvect<B, M, F>> for Mvect<A, M, F>
{
    type Output = Mvect<<M as MvMulType<CommutatorMarker, A, B>>::Output, M, F>;
    #[inline(always)]
    fn commutator(self, rhs: Mvect<B, M, F>) -> Self::Output {
        let mut out = Self::Output::default();
        mv_mul_runner::<CommutatorMarker, A, B, M, F>(&mut out.0, &self.0, &rhs.0);
        out
    }
}
// ----
// Inner Product
impl<
        A: BasisSet<M> + Len<Output: ArrayLength>,
        B: BasisSet<M> + Len<Output: ArrayLength>,
        M: Metric
            + MvMulType<InnerProdMarker, A, B, Output: BasisSet<M> + Len<Output: ArrayLength>>
            + MvMulRun<InnerProdMarker, F, <M as MvMulType<InnerProdMarker, A, B>>::Output, A, B>,
        F: Field,
    > core::ops::BitOr<Mvect<B, M, F>> for Mvect<A, M, F>
{
    type Output = Mvect<<M as MvMulType<InnerProdMarker, A, B>>::Output, M, F>;
    #[inline(always)]
    fn bitor(self, rhs: Mvect<B, M, F>) -> Self::Output {
        let mut out = Self::Output::default();
        mv_mul_runner::<InnerProdMarker, A, B, M, F>(&mut out.0, &self.0, &rhs.0);
        out
    }
}
// ----
// Left Contraction
impl<
        A: BasisSet<M> + Len<Output: ArrayLength>,
        B: BasisSet<M> + Len<Output: ArrayLength>,
        M: Metric
            + MvMulType<LeftContractionMarker, A, B, Output: BasisSet<M> + Len<Output: ArrayLength>>
            + MvMulRun<
                LeftContractionMarker,
                F,
                <M as MvMulType<LeftContractionMarker, A, B>>::Output,
                A,
                B,
            >,
        F: Field,
    > core::ops::Shl<Mvect<B, M, F>> for Mvect<A, M, F>
{
    type Output = Mvect<<M as MvMulType<LeftContractionMarker, A, B>>::Output, M, F>;
    #[inline(always)]
    fn shl(self, rhs: Mvect<B, M, F>) -> Self::Output {
        let mut out = Self::Output::default();
        mv_mul_runner::<LeftContractionMarker, A, B, M, F>(&mut out.0, &self.0, &rhs.0);
        out
    }
}
// ----
// Right Contraction
impl<
        A: BasisSet<M> + Len<Output: ArrayLength>,
        B: BasisSet<M> + Len<Output: ArrayLength>,
        M: Metric
            + MvMulType<RightContractionMarker, A, B, Output: BasisSet<M> + Len<Output: ArrayLength>>
            + MvMulRun<
                RightContractionMarker,
                F,
                <M as MvMulType<RightContractionMarker, A, B>>::Output,
                A,
                B,
            >,
        F: Field,
    > core::ops::Shr<Mvect<B, M, F>> for Mvect<A, M, F>
{
    type Output = Mvect<<M as MvMulType<RightContractionMarker, A, B>>::Output, M, F>;
    #[inline(always)]
    fn shr(self, rhs: Mvect<B, M, F>) -> Self::Output {
        let mut out = Self::Output::default();
        mv_mul_runner::<RightContractionMarker, A, B, M, F>(&mut out.0, &self.0, &rhs.0);
        out
    }
}

// ----
// Scalar Product
impl<
        A: BasisSet<M> + Len<Output: ArrayLength>,
        B: BasisSet<M> + Len<Output: ArrayLength>,
        M: Metric
            + MvMulType<ScalarProdMarker, A, B, Output: BasisSet<M> + Len<Output: ArrayLength>>
            + MvMulRun<ScalarProdMarker, F, <M as MvMulType<ScalarProdMarker, A, B>>::Output, A, B>,
        F: Field,
    > ScalarProduct<Mvect<B, M, F>> for Mvect<A, M, F>
{
    type Output = Mvect<<M as MvMulType<ScalarProdMarker, A, B>>::Output, M, F>;
    #[inline(always)]
    fn scalar_prod(self, rhs: Mvect<B, M, F>) -> Self::Output {
        let mut out = Self::Output::default();
        mv_mul_runner::<ScalarProdMarker, A, B, M, F>(&mut out.0, &self.0, &rhs.0);
        out
    }
}
// ----
// FatDot Product
impl<
        A: BasisSet<M> + Len<Output: ArrayLength>,
        B: BasisSet<M> + Len<Output: ArrayLength>,
        M: Metric
            + MvMulType<FatDotMarker, A, B, Output: BasisSet<M> + Len<Output: ArrayLength>>
            + MvMulRun<FatDotMarker, F, <M as MvMulType<FatDotMarker, A, B>>::Output, A, B>,
        F: Field,
    > FatDot<Mvect<B, M, F>> for Mvect<A, M, F>
{
    type Output = Mvect<<M as MvMulType<FatDotMarker, A, B>>::Output, M, F>;
    #[inline(always)]
    fn fat_dot(self, rhs: Mvect<B, M, F>) -> Self::Output {
        let mut out = Self::Output::default();
        mv_mul_runner::<FatDotMarker, A, B, M, F>(&mut out.0, &self.0, &rhs.0);
        out
    }
}
// --------------------------------------------
// Field * Basis
impl<F: Field, M: Metric> core::ops::Mul<F> for ZeroVect<M> {
    type Output = Mvect<ta![], M, F>;
    #[inline(always)]
    fn mul(self, _: F) -> Self::Output {
        Self::Output::default()
    }
}
impl<U: Unsigned, M: Metric, F: Field> core::ops::Mul<F> for Basis<U, M, B0> {
    type Output = Mvect<ta![U], M, F>;
    #[inline(always)]
    fn mul(self, rhs: F) -> Self::Output {
        let mut out = GenericArray::default();
        out[0] = rhs;
        Mvect(out, PhantomData)
    }
}
impl<U: Unsigned, M: Metric, F: Field> core::ops::Mul<F> for Basis<U, M, B1> {
    type Output = Mvect<ta![U], M, F>;
    #[inline(always)]
    fn mul(self, rhs: F) -> Self::Output {
        let mut out = GenericArray::default();
        out[0] = -rhs;
        Mvect(out, PhantomData)
    }
}
impl<A: BasisSet<M> + Len<Output: ArrayLength>, M: Metric, F: Field> core::ops::Mul<F>
    for Mvect<A, M, F>
{
    type Output = Mvect<A, M, F>;
    #[inline(always)]
    fn mul(self, rhs: F) -> Self::Output {
        let mut out = self;
        for i in 0..out.0.len() {
            out.0[i] *= rhs.clone();
        }
        out
    }
}

// --------------------------------------------
// Sandwich Product -- Rem
impl<
        A: BasisSet<M> + Len<Output: ArrayLength>,
        B: BasisSet<M> + Len<Output: ArrayLength>,
        M: Metric,
        F: Field,
    > Sandwich<Mvect<B, M, F>> for Mvect<A, M, F>
where
    Mvect<A, M, F>: Inverse
        + core::ops::Mul<Mvect<B, M, F>, Output: core::ops::Mul<<Mvect<A, M, F> as Inverse>::Output>>,
{
    type Output = Prod<Prod<Mvect<A, M, F>, Mvect<B, M, F>>, <Mvect<A, M, F> as Inverse>::Output>;
    #[inline(always)]
    fn sandwich(self, rhs: Mvect<B, M, F>) -> Option<Self::Output> {
        Some(self.clone() * rhs * self.inverse()?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::pga2d::{e0, e01, e012, e02, e1, e12, e2, scalar as e};

    #[test]
    fn test_geo_prod() {
        // (1 + 2e0 + 3e1 + 5e2 + 7e01 + 11e02 + 13e12 + 17e012)
        // *(19 + 23e0 + 29e1 + 31e2 + 37e01 + 41e02 + 43e12 + 47e012)
        // =−298 − 1053e0 + 274e1 + 122e2 + 981e01 − 617e02 + 238e12 + 715e012
        let a = 1.0 * e
            + 2.0 * e0
            + 3.0 * e1
            + 5.0 * e2
            + 7.0 * e01
            + 11.0 * e02
            + 13.0 * e12
            + 17.0 * e012;
        let b = 19.0 * e
            + 23.0 * e0
            + 29.0 * e1
            + 31.0 * e2
            + 37.0 * e01
            + 41.0 * e02
            + 43.0 * e12
            + 47.0 * e012;
        let expected = -298.0 * e - 1053.0 * e0 + 274.0 * e1 - 122.0 * e2 + 981.0 * e01
            - 617.0 * e02
            + 238.0 * e12
            + 715.0 * e012;
        let actual = a * b;

        println!("a = {}", a);
        println!("b = {}", b);
        println!("expected = {}", expected);
        println!("actual   = {}", actual);
        println!("diff     = {}", expected - actual);

        assert!(expected == actual);
    }
    #[test]
    fn test_outer_prod() {
        // (1 + 2e0 + 3e1 + 5e2 + 7e01 + 11e02 + 13e12 + 17e012)
        // ^(19 + 23e0 + 29e1 + 31e2 + 37e01 + 41e02 + 43e12 + 47e012)
        // = 19 + 61e0 + 86e1 + 159e01 + 126e2 + 197e02 + 238e12 + 715e012
        let a = 1.0 * e
            + 2.0 * e0
            + 3.0 * e1
            + 5.0 * e2
            + 7.0 * e01
            + 11.0 * e02
            + 13.0 * e12
            + 17.0 * e012;
        let b = 19.0 * e
            + 23.0 * e0
            + 29.0 * e1
            + 31.0 * e2
            + 37.0 * e01
            + 41.0 * e02
            + 43.0 * e12
            + 47.0 * e012;
        let expected = 19.0 * e
            + 61.0 * e0
            + 86.0 * e1
            + 159.0 * e01
            + 126.0 * e2
            + 197.0 * e02
            + 238.0 * e12
            + 715.0 * e012;
        let actual = a ^ b;

        println!("a = {}", a);
        println!("b = {}", b);
        println!("expected = {}", expected);
        println!("actual   = {}", actual);
        println!("diff     = {}", expected - actual);

        assert!(expected == actual);
    }
    #[test]
    fn test_inner_prod() {
        // (1 + 2e0 + 3e1 + 5e2 + 7e01 + 11e02 + 13e12 + 17e012)
        // |(19 + 23e0 + 29e1 + 31e2 + 37e01 + 41e02 + 43e12 + 47e012)
        let a = 1.0 * e
            + 2.0 * e0
            + 3.0 * e1
            + 5.0 * e2
            + 7.0 * e01
            + 11.0 * e02
            + 13.0 * e12
            + 17.0 * e012;
        let b = 19.0 * e
            + 23.0 * e0
            + 29.0 * e1
            + 31.0 * e2
            + 37.0 * e01
            + 41.0 * e02
            + 43.0 * e12
            + 47.0 * e012;
        // TODO: is this the correct definition of inner product?
        let expected = (a * b) - (a ^ b);
        let actual = a | b;

        println!("a = {}", a);
        println!("b = {}", b);
        println!("expected = {}", expected);
        println!("actual   = {}", actual);
        println!("diff     = {}", expected - actual);

        assert!(expected == actual);
    }
    #[test]
    fn test_commutator_prod() {
        // (1 + 2e0 + 3e1 + 5e2 + 7e01 + 11e02 + 13e12 + 17e012)
        // x(19 + 23e0 + 29e1 + 31e2 + 37e01 + 41e02 + 43e12 + 47e012)
        let a = 1.0 * e
            + 2.0 * e0
            + 3.0 * e1
            + 5.0 * e2
            + 7.0 * e01
            + 11.0 * e02
            + 13.0 * e12
            + 17.0 * e012;
        let b = 19.0 * e
            + 23.0 * e0
            + 29.0 * e1
            + 31.0 * e2
            + 37.0 * e01
            + 41.0 * e02
            + 43.0 * e12
            + 47.0 * e012;
        let expected = 0.5 * ((a * b) - (b * a));
        let actual = a.commutator(b);

        println!("a = {}", a);
        println!("b = {}", b);
        println!("expected = {}", expected);
        println!("actual   = {}", actual);
        println!("diff     = {}", expected - actual);

        assert!(expected == actual);
    }
    #[test]
    fn test_left_contration() {
        // (1 + 2e0 + 3e1 + 5e2 + 7e01 + 11e02 + 13e12 + 17e012)
        // << (19 + 23e0 + 29e1 + 31e2 + 37e01 + 41e02 + 43e12 + 47e012)
        // = -298 - 904e0 - 186e1 + 160e2 + 272e01 - 100e02 + 43e12 + 47e012
        let a = 1.0 * e
            + 2.0 * e0
            + 3.0 * e1
            + 5.0 * e2
            + 7.0 * e01
            + 11.0 * e02
            + 13.0 * e12
            + 17.0 * e012;
        let b = 19.0 * e
            + 23.0 * e0
            + 29.0 * e1
            + 31.0 * e2
            + 37.0 * e01
            + 41.0 * e02
            + 43.0 * e12
            + 47.0 * e012;
        let expected = -298.0 * e - 904.0 * e0 - 186.0 * e1 + 160.0 * e2 + 272.0 * e01
            - 100.0 * e02
            + 43.0 * e12
            + 47.0 * e012;
        let actual = a << b;

        println!("a = {}", a);
        println!("b = {}", b);
        println!("expected = {}", expected);
        println!("actual   = {}", actual);
        println!("diff     = {}", expected - actual);

        assert!(expected == actual);
    }
    #[test]
    fn test_right_contration() {
        // (1 + 2e0 + 3e1 + 5e2 + 7e01 + 11e02 + 13e12 + 17e012)
        // >> (19 + 23e0 + 29e1 + 31e2 + 37e01 + 41e02 + 43e12 + 47e012)
        let a = 1.0 * e
            + 2.0 * e0
            + 3.0 * e1
            + 5.0 * e2
            + 7.0 * e01
            + 11.0 * e02
            + 13.0 * e12
            + 17.0 * e012;
        let b = 19.0 * e
            + 23.0 * e0
            + 29.0 * e1
            + 31.0 * e2
            + 37.0 * e01
            + 41.0 * e02
            + 43.0 * e12
            + 47.0 * e012;
        let expected =
            -298.0 * e - 149.0 * e0 + 460.0 * e1 + 660.0 * e01 - 282.0 * e2 - 284.0 * e02
                + 247.0 * e12
                + 323.0 * e012;
        let actual = a >> b;

        println!("a = {}", a);
        println!("b = {}", b);
        println!("expected = {}", expected);
        println!("actual   = {}", actual);
        println!("diff     = {}", expected - actual);

        assert!(expected == actual);
    }
    #[test]
    fn test_scalar_product() {
        // (1 + 2e0 + 3e1 + 5e2 + 7e01 + 11e02 + 13e12 + 17e012)
        // .scalar_prod (19 + 23e0 + 29e1 + 31e2 + 37e01 + 41e02 + 43e12 + 47e012)
        let a = 1.0 * e
            + 2.0 * e0
            + 3.0 * e1
            + 5.0 * e2
            + 7.0 * e01
            + 11.0 * e02
            + 13.0 * e12
            + 17.0 * e012;
        let b = 19.0 * e
            + 23.0 * e0
            + 29.0 * e1
            + 31.0 * e2
            + 37.0 * e01
            + 41.0 * e02
            + 43.0 * e12
            + 47.0 * e012;
        let expected = -298.0 * e;
        let actual = a.scalar_prod(b);

        println!("a = {}", a);
        println!("b = {}", b);
        println!("expected = {}", expected);
        println!("actual   = {}", actual);
        println!("diff     = {}", expected - actual);

        assert!(expected == actual);
    }
    #[test]
    fn test_fat_dot() {
        // (1 + 2e0 + 3e1 + 5e2 + 7e01 + 11e02 + 13e12 + 17e012)
        // .fat_dot (19 + 23e0 + 29e1 + 31e2 + 37e01 + 41e02 + 43e12 + 47e012)
        // = -298 - 1053 * e0 + 274 * e1 + 932 * e01 - 122 * e2 - 384 * e02 + 290 * e12 + 370 * e012
        let a = 1.0 * e
            + 2.0 * e0
            + 3.0 * e1
            + 5.0 * e2
            + 7.0 * e01
            + 11.0 * e02
            + 13.0 * e12
            + 17.0 * e012;
        let b = 19.0 * e
            + 23.0 * e0
            + 29.0 * e1
            + 31.0 * e2
            + 37.0 * e01
            + 41.0 * e02
            + 43.0 * e12
            + 47.0 * e012;
        let expected = (a << b) + (a >> b) - a.scalar_prod(b);
        let actual = a.fat_dot(b);

        println!("a = {}", a);
        println!("b = {}", b);
        println!("expected = {}", expected);
        println!("actual   = {}", actual);
        println!("diff     = {}", expected - actual);

        assert!(expected == actual);
    }
}
