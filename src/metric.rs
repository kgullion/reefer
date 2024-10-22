use crate::utils::{At, Branch, CountOf, Get, If};
use core::ops::BitXor;
use typenum::{ATerm, Bit, Integer, TArr, UInt, Unsigned, Xor, B0, B1, N1, P1, U0, Z0};

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

pub trait IntFromSwapParityWithOverlaps<DegenOverlap, NegOverlap> {
    type Output: Integer;
}
impl<P: Bit, U, B, NM> IntFromSwapParityWithOverlaps<UInt<U, B>, NM> for P {
    // DegenOverlap is non-zero, so Output = 0
    type Output = Z0;
}
impl<P: Bit, NM: CountOf<B1>> IntFromSwapParityWithOverlaps<U0, NM> for P
where
    NM::Count: At<U0>,
    P: BitXor<Get<NM::Count, U0>>,
    Xor<P, Get<NM::Count, U0>>: Branch<N1, P1>,
    If<Xor<P, Get<NM::Count, U0>>, N1, P1>: Integer,
{
    // zero DegenOverlap, so count neg overlaps. Output = -1 if odd else +1
    type Output = If<Xor<P, Get<NM::Count, U0>>, N1, P1>;
}
