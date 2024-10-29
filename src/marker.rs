use crate::parity::{SwapPar, SwapParity};
use typenum::{And, Bit, Eq, IsEqual, IsNotEqual, NotEq, Or, Unsigned, Xor, B1, U0};

// Helper trait for multiplication marker structs (the K type parameter in MvMul etc)
pub trait MvMulMarker<L: Unsigned, R: Unsigned>: Sized {
    type Output: Bit;
}

pub struct GeoProdMarker;
impl<L: Unsigned, R: Unsigned> MvMulMarker<L, R> for GeoProdMarker {
    // every element of the cartesian product of L and R is in the result
    type Output = B1;
}

pub struct OuterProdMarker;
impl<L: Unsigned + core::ops::BitAnd<R, Output: IsEqual<U0>>, R: Unsigned> MvMulMarker<L, R>
    for OuterProdMarker
{
    // C∧D = Σ〈〈C〉ₛ〈D〉ₜ〉ₛ+ₜ -> L&R==0 // no overlap
    type Output = Eq<And<L, R>, U0>;
}

pub struct CommutatorMarker;
impl<
        L: Unsigned + SwapPar<R, Parity: core::ops::BitXor<SwapParity<R, L>, Output: Bit>>,
        R: Unsigned + SwapPar<L>,
    > MvMulMarker<L, R> for CommutatorMarker
{
    // (a*b - b*a)/2 -> Not<IsReverse<Prod<L,R>>> // if an element is it's own reverse, it's not in the result
    type Output = Xor<SwapParity<L, R>, SwapParity<R, L>>;
}

pub struct InnerProdMarker;
impl<L: Unsigned + core::ops::BitAnd<R, Output: IsNotEqual<U0>>, R: Unsigned> MvMulMarker<L, R>
    for InnerProdMarker
{
    // Opposite selection to outer product
    type Output = NotEq<And<L, R>, U0>;
}

pub struct LeftContractionMarker;
impl<L: Unsigned + core::ops::BitAnd<R, Output: IsEqual<L>>, R: Unsigned> MvMulMarker<L, R>
    for LeftContractionMarker
{
    // C<<D = Σ〈〈C〉ₛ〈D〉ₜ〉ₜ-ₛ // L⊆R = L&R==L
    type Output = Eq<And<L, R>, L>;
}

pub struct RightContractionMarker;
impl<L: Unsigned + core::ops::BitAnd<R, Output: IsEqual<R>>, R: Unsigned> MvMulMarker<L, R>
    for RightContractionMarker
{
    // C>>D = Σ〈〈C〉ₛ〈D〉ₜ〉ₛ-ₜ // R⊆L = L&R==R
    type Output = Eq<And<L, R>, R>;
}

pub struct ScalarProdMarker;
impl<L: Unsigned + IsEqual<R>, R: Unsigned> MvMulMarker<L, R> for ScalarProdMarker {
    // C∧D = Σ〈〈C〉ₛ〈D〉ₜ〉₀ -> L==R // complete overlap
    type Output = Eq<L, R>;
}

pub struct FatDotMarker;
impl<L: Unsigned, R: Unsigned> MvMulMarker<L, R> for FatDotMarker
where
    L: core::ops::BitAnd<
        R,
        Output: IsEqual<L, Output: core::ops::BitOr<Eq<And<L, R>, R>, Output: Bit>> + IsEqual<R>,
    >,
{
    // C∧D = Σ〈〈C〉ₛ〈D〉ₜ〉|ₜ-ₛ| // L⊆R || R⊆L
    type Output = Or<Eq<And<L, R>, L>, Eq<And<L, R>, R>>;
}
