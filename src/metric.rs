use crate::utils::{At, Count, CountOf, Get};
use core::ops::BitAnd;
use typenum::{
    tarr, ATerm, And, Bit, Integer, IsNotEqual, NotEq, TArr, UInt, Unsigned, B0, B1, N1, P1, U0, Z0,
};

pub trait Metric {
    type Psuedoscalar: Unsigned;
    type PosMask: Unsigned;
    type NegMask: Unsigned;
    type ZeroMask: Unsigned;
}
impl Metric for ATerm {
    type Psuedoscalar = U0;
    type PosMask = U0;
    type NegMask = U0;
    type ZeroMask = U0;
}
impl<TL: Metric> Metric for TArr<P1, TL> {
    type Psuedoscalar = UInt<TL::Psuedoscalar, B1>;
    type PosMask = UInt<TL::PosMask, B1>;
    type NegMask = UInt<TL::NegMask, B0>;
    type ZeroMask = UInt<TL::ZeroMask, B0>;
}
impl<TL: Metric> Metric for TArr<N1, TL> {
    type Psuedoscalar = UInt<TL::Psuedoscalar, B1>;
    type PosMask = UInt<TL::PosMask, B0>;
    type NegMask = UInt<TL::NegMask, B1>;
    type ZeroMask = UInt<TL::ZeroMask, B0>;
}
impl<TL: Metric> Metric for TArr<Z0, TL> {
    type Psuedoscalar = UInt<TL::Psuedoscalar, B1>;
    type PosMask = UInt<TL::PosMask, B0>;
    type NegMask = UInt<TL::NegMask, B0>;
    type ZeroMask = UInt<TL::ZeroMask, B1>;
}

/// Does mask square to zero under metric?
pub type IsDegen<M, U> = <U as DegenCheck<M>>::Output;
pub trait DegenCheck<M: Metric>: Unsigned {
    type Output: Bit;
}
impl<U: Unsigned, M: Metric> DegenCheck<M> for U
where
    U: BitAnd<M::ZeroMask, Output: IsNotEqual<U0, Output: Bit>>,
{
    type Output = NotEq<And<U, M::ZeroMask>, U0>;
}

/// Does mask square to -1 under metric?
pub type MaskParity<M, U> = <U as MaskPar<M>>::Output;
pub trait MaskPar<M: Metric>: Unsigned {
    type Output: Bit;
}
impl<U: Unsigned, M: Metric> MaskPar<M> for U
where
    U: BitAnd<M::NegMask, Output: CountOf<B1, Count: At<U0, Output: Bit>>>,
{
    // U & M.neg & 1
    type Output = Get<Count<And<U, M::NegMask>, B1>, U0>;
}

/// Calculate the orientation of the masks under the metric, +1, 0, or -1
pub trait TritMul<L, R>: Metric {
    type Output: Trit;
}
pub trait Trit: Integer {}
impl Trit for N1 {}
impl Trit for Z0 {}
impl Trit for P1 {}

pub type TritProd<M, L, R> = <M as TritMul<L, R>>::Output;
impl<L: Unsigned, R: Unsigned, M: Metric> TritMul<L, R> for M
where
    L: BitAnd<R, Output: DegenCheck<M> + MaskPar<M>>,
    tarr![IsDegen<M, And<L, R>>, MaskParity<M, And<L, R>>]: TritMulInner,
{
    // Output = 0 if degenerate, -1 if odd parity, +1 if even parity
    type Output = <tarr![IsDegen<M, And<L, R>>, MaskParity<M, And<L, R>>] as TritMulInner>::Output;
}

pub trait TritMulInner {
    type Output: Trit;
}
impl TritMulInner for tarr![B0, B0] {
    type Output = P1;
}
impl TritMulInner for tarr![B0, B1] {
    type Output = N1;
}
impl TritMulInner for tarr![B1, B0] {
    type Output = Z0;
}
impl TritMulInner for tarr![B1, B1] {
    type Output = Z0;
}

pub type TXor<L, R> = <L as TritXor<R>>::Output;
pub trait TritXor<R: Bit> {
    type Output: Trit;
}

impl<S: Bit> TritXor<S> for Z0 {
    type Output = Z0;
}
impl TritXor<B0> for N1 {
    type Output = N1;
}
impl TritXor<B0> for P1 {
    type Output = P1;
}
impl TritXor<B1> for N1 {
    type Output = P1;
}
impl TritXor<B1> for P1 {
    type Output = N1;
}
