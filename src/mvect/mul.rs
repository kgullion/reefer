use crate::{
    basis::Basis,
    collector::CartCollector,
    field::Field,
    metric::Metric,
    mvect::{basis_set::BasisSet, into::IntoBasisSet, Mvect},
    utils::{
        typeset::{Union, UnionMerge},
        Branch, If,
    },
};
use core::ops::{BitAnd, Mul};
use generic_array::ArrayLength;
use typenum::{tarr, And, Bit, Eq, IsEqual, Len, Prod, TArr, TypeArray, Unsigned, B0, B1, U0};

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
impl<B: BasisSet<M>, OUT: BasisSet<M>, M: Metric, F: Field, K> MvMulRun<K, F, OUT, tarr![], B>
    for M
{
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
    > MvMulRun<K, F, OUT, TArr<L, A>, B> for M
where
    TArr<L, A>: BasisSet<M>,
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
impl<L: Unsigned, OUT: BasisSet<M>, M: Metric, F: Field, K> MvMulRunInner<K, F, OUT, L, tarr![]>
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
    > MvMulRunInner<K, F, OUT, L, TArr<R, B>> for M
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
impl<B: BasisSet<M>, M: Metric, K> MvMulType<K, tarr![], B> for M {
    type Output = tarr![];
}
// [L|A] * B = A*B + L*B
impl<
        L: Unsigned,
        A: BasisSet<M> + Len<Output: ArrayLength>,
        B: BasisSet<M> + Len<Output: ArrayLength>,
        M: Metric + MvMulType<K, A, B> + MvMulTypeInner<K, L, B>,
        K,
    > MvMulType<K, TArr<L, A>, B> for M
where
    TArr<L, A>: BasisSet<M>,
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
impl<L: Unsigned, M: Metric, K> MvMulTypeInner<K, L, tarr![]> for M {
    type Output = tarr![];
}
// L*[R|B] = L*R + L*B
impl<L: Unsigned, R: Unsigned, B: BasisSet<M>, M: Metric + MvMulTypeInner<K, L, B>, K>
    MvMulTypeInner<K, L, TArr<R, B>> for M
where
    K: MvMulMarker<L, R>,
    Basis<L, M, B0>: Mul<Basis<R, M, B0>, Output: IntoBasisSet<Output: BasisSet<M>>>,
    <K as MvMulMarker<L, R>>::Output: Branch<
        <Prod<Basis<L, M, B0>, Basis<R, M, B0>> as IntoBasisSet>::Output,
        tarr![],
        Output: TypeArray,
    >,
    <M as MvMulTypeInner<K, L, B>>::Output: UnionMerge<
        If<
            <K as MvMulMarker<L, R>>::Output,
            <Prod<Basis<L, M, B0>, Basis<R, M, B0>> as IntoBasisSet>::Output,
            tarr![],
        >,
    >,
{
    type Output = Union<
        <M as MvMulTypeInner<K, L, B>>::Output,
        If<
            <K as MvMulMarker<L, R>>::Output,
            <Prod<Basis<L, M, B0>, Basis<R, M, B0>> as IntoBasisSet>::Output,
            tarr![],
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
// Mul - multiply two multivectors 🎉
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
// Commutator Product - TODO
pub struct CommutatorMarker;
// ----
// Inner Product - TODO
pub struct InnerProdMarker;
// ----
// Left Contraction - TODO
pub struct LeftContractionMarker;
// impl<L, R> Decider for LeftContractionMarker<L, R> {
//     // C∧D = Σ〈〈C〉ₛ〈D〉ₜ〉ₛ-ₜ // R⊆L
//     type Output = ;
// }
// ----
// Right Contraction - TODO
pub struct RightContractionMarker;
// impl<L, R> Decider for RightContractionMarker<L, R> {
//     // C∧D = Σ〈〈C〉ₛ〈D〉ₜ〉ₜ-ₛ // L⊆R
//     type Output = ;
// }

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
    > core::ops::BitOr<Mvect<B, M, F>> for Mvect<A, M, F>
{
    type Output = Mvect<<M as MvMulType<ScalarProdMarker, A, B>>::Output, M, F>;
    fn bitor(self, rhs: Mvect<B, M, F>) -> Self::Output {
        let mut out = Self::Output::default();
        mv_mul_runner::<ScalarProdMarker, A, B, M, F>(&mut out.0, &self.0, &rhs.0);
        out
    }
}
// ----
// FatDot Product
pub struct FatDotMarker;
// impl<L, R> Decider for RightContractionMarker<L, R> {
//     // C∧D = Σ〈〈C〉ₛ〈D〉ₜ〉|ₜ-ₛ| // L⊆R || R⊆L
//     type Output = B0;
// }
