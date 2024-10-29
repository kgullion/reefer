use crate::parity::{SwapPar, SwapParity};
use core::ops::{BitAnd, BitOr, BitXor};
use typenum::{And, Bit, Eq, IsEqual, IsNotEqual, NotEq, Or, Unsigned, Xor, B1, U0};

// Helper trait for multiplication marker structs (the K type parameter in MvMul etc)
pub trait MulMarker<L: Unsigned, R: Unsigned>: Sized {
    type Output: Bit;
}
pub type MarkedProd<K, L, R> = <K as MulMarker<L, R>>::Output;

pub struct GeoProdMarker;
impl<L: Unsigned, R: Unsigned> MulMarker<L, R> for GeoProdMarker {
    // every element of the cartesian product of L and R is in the result
    type Output = B1;
}

pub struct OuterProdMarker;
impl<L: Unsigned + BitAnd<R, Output: IsEqual<U0>>, R: Unsigned> MulMarker<L, R>
    for OuterProdMarker
{
    // C∧D = Σ〈〈C〉ₛ〈D〉ₜ〉ₛ+ₜ -> L&R==0 // no overlap
    type Output = Eq<And<L, R>, U0>;
}

pub struct CommutatorMarker;
impl<
        L: Unsigned + SwapPar<R, Parity: BitXor<SwapParity<R, L>, Output: Bit>>,
        R: Unsigned + SwapPar<L>,
    > MulMarker<L, R> for CommutatorMarker
{
    // (a*b - b*a)/2 -> must be antisymmetric
    type Output = Xor<SwapParity<L, R>, SwapParity<R, L>>;
}

pub struct InnerProdMarker;
impl<L: Unsigned + BitAnd<R, Output: IsNotEqual<U0>>, R: Unsigned> MulMarker<L, R>
    for InnerProdMarker
{
    // Opposite selection to outer product
    type Output = NotEq<And<L, R>, U0>;
}

pub struct LeftContractionMarker;
impl<L: Unsigned + BitAnd<R, Output: IsEqual<L>>, R: Unsigned> MulMarker<L, R>
    for LeftContractionMarker
{
    // C<<D = Σ〈〈C〉ₛ〈D〉ₜ〉ₜ-ₛ // L⊆R = L&R==L
    type Output = Eq<And<L, R>, L>;
}

pub struct RightContractionMarker;
impl<L: Unsigned + BitAnd<R, Output: IsEqual<R>>, R: Unsigned> MulMarker<L, R>
    for RightContractionMarker
{
    // C>>D = Σ〈〈C〉ₛ〈D〉ₜ〉ₛ-ₜ // R⊆L = L&R==R
    type Output = Eq<And<L, R>, R>;
}

pub struct ScalarProdMarker;
impl<L: Unsigned + IsEqual<R>, R: Unsigned> MulMarker<L, R> for ScalarProdMarker {
    // C∧D = Σ〈〈C〉ₛ〈D〉ₜ〉₀ -> L==R // complete overlap
    type Output = Eq<L, R>;
}

pub struct FatDotMarker;
impl<L: Unsigned, R: Unsigned> MulMarker<L, R> for FatDotMarker
where
    L: BitAnd<R, Output: IsEqual<L, Output: BitOr<Eq<And<L, R>, R>, Output: Bit>> + IsEqual<R>>,
{
    // C∧D = Σ〈〈C〉ₛ〈D〉ₜ〉|ₜ-ₛ| // L⊆R || R⊆L
    type Output = Or<Eq<And<L, R>, L>, Eq<And<L, R>, R>>;
}
