use crate::{
    basis::Basis,
    collector::CartCollector,
    field::Field,
    metric::Metric,
    mvect::{basis_set::BasisSet, into::IntoBasisSet, Mvect},
    ta,
    traits::{Commutator, FatDot, ScalarProduct},
    utils::{
        parity::{ReversePar, ReverseParity},
        typeset::{Union, UnionMerge},
        Branch, If,
    },
};
use core::marker::PhantomData;
use core::ops::{Add, BitAnd, BitOr, BitXor, Mul};
use generic_array::{ArrayLength, GenericArray};
use typenum::{
    And, Bit, Eq, IsEqual, IsNotEqual, Len, NotEq, Or, Prod, TypeArray, UInt, Unsigned, Xor, B0,
    B1, U0,
};

// --------------------------------------------
// multivector multiplication
// ----
// Helper trait for multiplication marker structs (the K type parameter in MvMul etc)
pub trait MvMulMarker<L: Unsigned, R: Unsigned>: Sized {
    type Output: Bit;
}
// MvMul - does the runtime work
pub trait MvMulRun<K, F, OUT, A: BasisSet<Self>, B: BasisSet<Self>>: Metric + Sized {
    fn mv_mul(out: &mut [F], left: &[F], right: &[F]);
}
impl<B: BasisSet<M>, OUT: BasisSet<M>, M: Metric, F: Field, K> MvMulRun<K, F, OUT, ta![], B> for M {
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
    Basis<L, M, B0>: Mul<Basis<R, M, B0>, Output: CartCollector<F, OUT>>,
{
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
    Basis<L, M, B0>: Mul<Basis<R, M, B0>, Output: IntoBasisSet<Output: BasisSet<M>>>,
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

/// internal helper for implementing multiplication operators
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
pub struct GeoProdMarker;
impl<L: Unsigned, R: Unsigned> MvMulMarker<L, R> for GeoProdMarker {
    // every element of the cartesian product of L and R is in the result
    type Output = B1;
}
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
    fn mul(self, rhs: Mvect<B, M, F>) -> Self::Output {
        let mut out = Self::Output::default();
        mv_mul_runner::<GeoProdMarker, A, B, M, F>(&mut out.0, &self.0, &rhs.0);
        out
    }
}
// ----
// Outer Product
pub struct OuterProdMarker;
impl<L: Unsigned + BitAnd<R, Output: IsEqual<U0>>, R: Unsigned> MvMulMarker<L, R>
    for OuterProdMarker
{
    // C∧D = Σ〈〈C〉ₛ〈D〉ₜ〉ₛ+ₜ -> L&R==0 // no overlap
    type Output = Eq<And<L, R>, U0>;
}
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
    fn bitxor(self, rhs: Mvect<B, M, F>) -> Self::Output {
        let mut out = Self::Output::default();
        mv_mul_runner::<OuterProdMarker, A, B, M, F>(&mut out.0, &self.0, &rhs.0);
        out
    }
}
// ----
// Commutator Product
pub struct CommutatorMarker;
impl<L: Unsigned + BitXor<R, Output: ReversePar>, R: Unsigned> MvMulMarker<L, R>
    for CommutatorMarker
{
    // (a*b - b*a)/2 -> Not<IsReverse<Prod<L,R>>> // if an element is it's own reverse, it's not in the result
    type Output = ReverseParity<Xor<L, R>>;
}
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
    fn commutator(self, rhs: Mvect<B, M, F>) -> Self::Output {
        let mut out = Self::Output::default();
        mv_mul_runner::<CommutatorMarker, A, B, M, F>(&mut out.0, &self.0, &rhs.0);
        out
    }
}
// ----
// Inner Product
pub struct InnerProdMarker;
impl<L: Unsigned + BitAnd<R, Output: IsNotEqual<U0>>, R: Unsigned> MvMulMarker<L, R>
    for InnerProdMarker
{
    // Opposite selection to outer product
    type Output = NotEq<And<L, R>, U0>;
}
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
    fn bitor(self, rhs: Mvect<B, M, F>) -> Self::Output {
        let mut out = Self::Output::default();
        mv_mul_runner::<InnerProdMarker, A, B, M, F>(&mut out.0, &self.0, &rhs.0);
        out
    }
}
// ----
// Left Contraction
pub struct LeftContractionMarker;
impl<L: Unsigned + BitAnd<R, Output: IsEqual<R>>, R: Unsigned> MvMulMarker<L, R>
    for LeftContractionMarker
{
    // C∧D = Σ〈〈C〉ₛ〈D〉ₜ〉ₛ-ₜ // R⊆L = L&R==R
    type Output = Eq<And<L, R>, R>;
}
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
    fn shl(self, rhs: Mvect<B, M, F>) -> Self::Output {
        let mut out = Self::Output::default();
        mv_mul_runner::<LeftContractionMarker, A, B, M, F>(&mut out.0, &self.0, &rhs.0);
        out
    }
}
// ----
// Right Contraction
pub struct RightContractionMarker;
impl<L: Unsigned + BitAnd<R, Output: IsEqual<L>>, R: Unsigned> MvMulMarker<L, R>
    for RightContractionMarker
{
    // C∧D = Σ〈〈C〉ₛ〈D〉ₜ〉ₜ-ₛ // L⊆R = L&R==L
    type Output = Eq<And<L, R>, L>;
}
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
    fn shr(self, rhs: Mvect<B, M, F>) -> Self::Output {
        let mut out = Self::Output::default();
        mv_mul_runner::<RightContractionMarker, A, B, M, F>(&mut out.0, &self.0, &rhs.0);
        out
    }
}

// ----
// Scalar Product
pub struct ScalarProdMarker;
impl<L: Unsigned + IsEqual<R>, R: Unsigned> MvMulMarker<L, R> for ScalarProdMarker {
    // C∧D = Σ〈〈C〉ₛ〈D〉ₜ〉₀ -> L==R // complete overlap
    type Output = Eq<L, R>;
}
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
    fn scalar_prod(self, rhs: Mvect<B, M, F>) -> Self::Output {
        let mut out = Self::Output::default();
        mv_mul_runner::<ScalarProdMarker, A, B, M, F>(&mut out.0, &self.0, &rhs.0);
        out
    }
}
// ----
// FatDot Product
pub struct FatDotMarker;
impl<L: Unsigned, R: Unsigned> MvMulMarker<L, R> for FatDotMarker
where
    L: BitAnd<R, Output: IsEqual<L, Output: BitOr<Eq<And<L, R>, R>, Output: Bit>> + IsEqual<R>>,
{
    // C∧D = Σ〈〈C〉ₛ〈D〉ₜ〉|ₜ-ₛ| // L⊆R || R⊆L
    type Output = Or<Eq<And<L, R>, L>, Eq<And<L, R>, R>>;
}
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
    fn fat_dot(self, rhs: Mvect<B, M, F>) -> Self::Output {
        let mut out = Self::Output::default();
        mv_mul_runner::<FatDotMarker, A, B, M, F>(&mut out.0, &self.0, &rhs.0);
        out
    }
}

// --------------------------------------------
impl<U: Unsigned, M: Metric, F: Field> core::ops::Mul<F> for Basis<U, M, B0> {
    type Output = Mvect<ta![U], M, F>;
    fn mul(self, rhs: F) -> Self::Output {
        let mut out = GenericArray::default();
        out[0] = rhs;
        Mvect(out, PhantomData)
    }
}
impl<U: Unsigned, M: Metric, F: Field> core::ops::Mul<F> for Basis<U, M, B1> {
    type Output = Mvect<ta![U], M, F>;
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
    fn mul(self, rhs: F) -> Self::Output {
        let mut out = self;
        for i in 0..out.0.len() {
            out.0[i] *= rhs.clone();
        }
        out
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use typenum::{B0, P1, U0, U1, U2, U3, U4, U5, U6, U7, Z0};

    type Metric = ta![Z0, P1, P1];
    type Pga2d<U> = Basis<U, Metric, B0>;

    const E: Pga2d<U0> = Pga2d::<U0>::new();
    const E0: Pga2d<U1> = Pga2d::<U1>::new();
    const E1: Pga2d<U2> = Pga2d::<U2>::new();
    const E01: Pga2d<U3> = Pga2d::<U3>::new();
    const E2: Pga2d<U4> = Pga2d::<U4>::new();
    const E02: Pga2d<U5> = Pga2d::<U5>::new();
    const E12: Pga2d<U6> = Pga2d::<U6>::new();
    const E012: Pga2d<U7> = Pga2d::<U7>::new();

    #[test]
    fn test_geo_prod() {
        // (1 + 2e0 + 3e1 + 5e2 + 7e01 + 11e02 + 13e12 + 17e012)
        // *(19 + 23e0 + 29e1 + 31e2 + 37e01 + 41e02 + 43e12 + 47e012)
        // =−298 − 1053e0 + 274e1 + 122e2 + 981e01 − 617e02 + 238e12 + 715e012
        let a = 1.0 * E
            + 2.0 * E0
            + 3.0 * E1
            + 5.0 * E2
            + 7.0 * E01
            + 11.0 * E02
            + 13.0 * E12
            + 17.0 * E012;
        let b = 19.0 * E
            + 23.0 * E0
            + 29.0 * E1
            + 31.0 * E2
            + 37.0 * E01
            + 41.0 * E02
            + 43.0 * E12
            + 47.0 * E012;
        let expected = -298.0 * E - 1053.0 * E0 + 274.0 * E1 - 122.0 * E2 + 981.0 * E01
            - 617.0 * E02
            + 238.0 * E12
            + 715.0 * E012;
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
        let a = 1.0 * E
            + 2.0 * E0
            + 3.0 * E1
            + 5.0 * E2
            + 7.0 * E01
            + 11.0 * E02
            + 13.0 * E12
            + 17.0 * E012;
        let b = 19.0 * E
            + 23.0 * E0
            + 29.0 * E1
            + 31.0 * E2
            + 37.0 * E01
            + 41.0 * E02
            + 43.0 * E12
            + 47.0 * E012;
        let expected = 19.0 * E
            + 61.0 * E0
            + 86.0 * E1
            + 159.0 * E01
            + 126.0 * E2
            + 197.0 * E02
            + 238.0 * E12
            + 715.0 * E012;
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
        let a = 1.0 * E
            + 2.0 * E0
            + 3.0 * E1
            + 5.0 * E2
            + 7.0 * E01
            + 11.0 * E02
            + 13.0 * E12
            + 17.0 * E012;
        let b = 19.0 * E
            + 23.0 * E0
            + 29.0 * E1
            + 31.0 * E2
            + 37.0 * E01
            + 41.0 * E02
            + 43.0 * E12
            + 47.0 * E012;
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
}
