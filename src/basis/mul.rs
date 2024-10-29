use crate::{
    basis::{into::IntoBasis, Basis, ZeroVect},
    metric::{Metric, TritMul, TritXor},
    parity::{SwapPar, SwapParity},
    utils::{Branch, If},
};
use typenum::{And, Bit, Eq, IsEqual, Prod, Unsigned, Xor, U0};

// -------------------------------------------------------------------------------------
// Division
// 0/0 = None
impl<M: Metric> core::ops::Div<ZeroVect<M>> for ZeroVect<M> {
    type Output = Option<ZeroVect<M>>;
    #[inline(always)]
    fn div(self, _: ZeroVect<M>) -> Self::Output {
        None
    }
}
// B/0 = None
impl<U: Unsigned, M: Metric, S: Bit> core::ops::Div<ZeroVect<M>> for Basis<U, M, S> {
    type Output = Option<ZeroVect<M>>;
    #[inline(always)]
    fn div(self, _: ZeroVect<M>) -> Self::Output {
        None
    }
}
// 0/B = 0
impl<U: Unsigned, M: Metric, S: Bit> core::ops::Div<Basis<U, M, S>> for ZeroVect<M> {
    type Output = Option<ZeroVect<M>>;
    #[inline(always)]
    fn div(self, _: Basis<U, M, S>) -> Self::Output {
        Some(self)
    }
}

// -------------------------------------------------------------------------------------
// Geometric Product
// 0 * 0 = 0
impl<M: Metric> core::ops::Mul<ZeroVect<M>> for ZeroVect<M> {
    type Output = ZeroVect<M>;
    #[inline(always)]
    fn mul(self, _: ZeroVect<M>) -> Self::Output {
        Self::Output::default()
    }
}
// 0 * B = 0
impl<U: Unsigned, M: Metric, S: Bit> core::ops::Mul<Basis<U, M, S>> for ZeroVect<M> {
    type Output = ZeroVect<M>;
    #[inline(always)]
    fn mul(self, _: Basis<U, M, S>) -> Self::Output {
        Self::Output::default()
    }
}
// 0 * B = 0
impl<U: Unsigned, M: Metric, S: Bit> core::ops::Mul<ZeroVect<M>> for Basis<U, M, S> {
    type Output = ZeroVect<M>;
    #[inline(always)]
    fn mul(self, _: ZeroVect<M>) -> Self::Output {
        Self::Output::default()
    }
}
// L * R accounts for parity from the metric, the swaps, the left basis parity, and the right basis parity.
impl<
        L: Unsigned + SwapPar<R, Parity: Bit> + core::ops::BitXor<R, Output: Unsigned>,
        R: Unsigned,
        M: Metric
            + TritMul<
                L,
                R,
                Output: TritXor<
                    LS,
                    Output: TritXor<
                        RS,
                        Output: TritXor<
                            SwapParity<L, R>,
                            Output: IntoBasis<Xor<L, R>, M, Output: Default>,
                        >,
                    >,
                >,
            >,
        LS: Bit + core::ops::BitXor<RS>,
        RS: Bit,
    > core::ops::Mul<Basis<R, M, RS>> for Basis<L, M, LS>
{
    //   Output =      M |> TritMul<L, R>          |> TritXor<LS>          |> TritXor<RS>          |> TritXor<SwapParity<L, R>>          |> IntoBasis<Xor<L, R>, M>;
    type Output = <<<<<M as TritMul<L, R>>::Output as TritXor<LS>>::Output as TritXor<RS>>::Output as TritXor<SwapParity<L, R>>>::Output as IntoBasis<Xor<L, R>, M>>::Output;
    #[inline(always)]
    fn mul(self, _: Basis<R, M, RS>) -> Self::Output {
        Self::Output::default()
    }
}

// -------------------------------------------------------------------------------------
// Outer Product
// 0 ^ 0 = 0
impl<M: Metric> core::ops::BitXor<ZeroVect<M>> for ZeroVect<M> {
    type Output = ZeroVect<M>;
    #[inline(always)]
    fn bitxor(self, _: ZeroVect<M>) -> Self::Output {
        Self::Output::default()
    }
}
// 0 ^ B = B
impl<U: Unsigned, M: Metric, S: Bit> core::ops::BitXor<Basis<U, M, S>> for ZeroVect<M> {
    type Output = Basis<U, M, S>;
    #[inline(always)]
    fn bitxor(self, rhs: Basis<U, M, S>) -> Self::Output {
        rhs
    }
}
// B ^ 0 = B
impl<U: Unsigned, M: Metric, S: Bit> core::ops::BitXor<ZeroVect<M>> for Basis<U, M, S> {
    type Output = Basis<U, M, S>;
    #[inline(always)]
    fn bitxor(self, _: ZeroVect<M>) -> Self::Output {
        self
    }
}
// LB ^ RB = GP if no overlap, else Zero
impl<LU: Unsigned, RU: Unsigned, M: Metric, LS: Bit, RS: Bit> core::ops::BitXor<Basis<RU, M, RS>>
    for Basis<LU, M, LS>
where
    Self: core::ops::Mul<Basis<RU, M, RS>>,
    LU: core::ops::BitAnd<
        RU,
        Output: IsEqual<
            U0,
            Output: Branch<Prod<Basis<LU, M, LS>, Basis<RU, M, RS>>, ZeroVect<M>, Output: Default>,
        >,
    >,
{
    //   Output =       LU & RU == 0    ?            LB        *       RB         : Zero
    type Output = If<Eq<And<LU, RU>, U0>, Prod<Basis<LU, M, LS>, Basis<RU, M, RS>>, ZeroVect<M>>;
    #[inline(always)]
    fn bitxor(self, _: Basis<RU, M, RS>) -> Self::Output {
        Self::Output::default()
    }
}

// -------------------------------------------------------------------------------------
// Inner Product
// 0 | 0 = 0
impl<M: Metric> core::ops::BitOr<ZeroVect<M>> for ZeroVect<M> {
    type Output = ZeroVect<M>;
    #[inline(always)]
    fn bitor(self, _: ZeroVect<M>) -> Self::Output {
        Self::Output::default()
    }
}
// 0 | B = 0
impl<U: Unsigned, M: Metric, S: Bit> core::ops::BitOr<Basis<U, M, S>> for ZeroVect<M> {
    type Output = ZeroVect<M>;
    #[inline(always)]
    fn bitor(self, _: Basis<U, M, S>) -> Self::Output {
        Self::Output::default()
    }
}
// B | 0 = 0
impl<U: Unsigned, M: Metric, S: Bit> core::ops::BitOr<ZeroVect<M>> for Basis<U, M, S> {
    type Output = ZeroVect<M>;
    #[inline(always)]
    fn bitor(self, _: ZeroVect<M>) -> Self::Output {
        Self::Output::default()
    }
}
// LB | RB = GP if any overlap, else Zero
impl<LU: Unsigned, RU: Unsigned, M: Metric, LS: Bit, RS: Bit> core::ops::BitOr<Basis<RU, M, RS>>
    for Basis<LU, M, LS>
where
    Self: core::ops::Mul<Basis<RU, M, RS>>,
    LU: core::ops::BitAnd<
        RU,
        Output: IsEqual<
            U0,
            Output: Branch<ZeroVect<M>, Prod<Basis<LU, M, LS>, Basis<RU, M, RS>>, Output: Default>,
        >,
    >,
{
    //   Output =       LU & RU == 0    ? Zero                   LB        *       RB
    type Output = If<Eq<And<LU, RU>, U0>, ZeroVect<M>, Prod<Basis<LU, M, LS>, Basis<RU, M, RS>>>;
    #[inline(always)]
    fn bitor(self, _: Basis<RU, M, RS>) -> Self::Output {
        Self::Output::default()
    }
}

// -------------------------------------------------------------------------------------
// Left Contraction
// 0 << 0 = 0
impl<M: Metric> core::ops::Shl<ZeroVect<M>> for ZeroVect<M> {
    type Output = ZeroVect<M>;
    #[inline(always)]
    fn shl(self, _: ZeroVect<M>) -> Self::Output {
        Self::Output::default()
    }
}
// 0 << B = 0
impl<U: Unsigned, M: Metric, S: Bit> core::ops::Shl<Basis<U, M, S>> for ZeroVect<M> {
    type Output = ZeroVect<M>;
    #[inline(always)]
    fn shl(self, _: Basis<U, M, S>) -> Self::Output {
        Self::Output::default()
    }
}
// B << 0 = B
impl<U: Unsigned, M: Metric, S: Bit> core::ops::Shl<ZeroVect<M>> for Basis<U, M, S> {
    type Output = Basis<U, M, S>;
    #[inline(always)]
    fn shl(self, _: ZeroVect<M>) -> Self::Output {
        self
    }
}
// LB << RB = GP if R&L==L (R covers L), else Zero
impl<LU: Unsigned, RU: Unsigned, M: Metric, LS: Bit, RS: Bit> core::ops::Shl<Basis<RU, M, RS>>
    for Basis<LU, M, LS>
where
    Self: core::ops::Mul<Basis<RU, M, RS>>,
    LU: core::ops::BitAnd<
        RU,
        Output: IsEqual<
            LU,
            Output: Branch<Prod<Basis<LU, M, LS>, Basis<RU, M, RS>>, ZeroVect<M>, Output: Default>,
        >,
    >,
{
    //   Output = if    LU & RU == LU  then                 LB * RB        else     Zero
    type Output = If<Eq<And<LU, RU>, LU>, Prod<Basis<LU, M, LS>, Basis<RU, M, RS>>, ZeroVect<M>>;
    #[inline(always)]
    fn shl(self, _: Basis<RU, M, RS>) -> Self::Output {
        Self::Output::default()
    }
}

// -------------------------------------------------------------------------------------
// Right Contraction is the same as Left Contraction, but with the roles of L and R reversed.
// 0 >> 0 = 0
impl<M: Metric> core::ops::Shr<ZeroVect<M>> for ZeroVect<M> {
    type Output = ZeroVect<M>;
    #[inline(always)]
    fn shr(self, _: ZeroVect<M>) -> Self::Output {
        Self::Output::default()
    }
}
// 0 >> B = 0
impl<U: Unsigned, M: Metric, S: Bit> core::ops::Shr<Basis<U, M, S>> for ZeroVect<M> {
    type Output = ZeroVect<M>;
    #[inline(always)]
    fn shr(self, _: Basis<U, M, S>) -> Self::Output {
        Self::Output::default()
    }
}
// B >> 0 = B
impl<U: Unsigned, M: Metric, S: Bit> core::ops::Shr<ZeroVect<M>> for Basis<U, M, S> {
    type Output = Basis<U, M, S>;
    #[inline(always)]
    fn shr(self, _: ZeroVect<M>) -> Self::Output {
        self
    }
}
impl<RU: Unsigned, LU: Unsigned, M: Metric, RS: Bit, LS: Bit> core::ops::Shr<Basis<LU, M, LS>>
    for Basis<RU, M, RS>
where
    Basis<LU, M, LS>: core::ops::Shl<Basis<RU, M, RS>>,
{
    type Output = <Basis<LU, M, LS> as core::ops::Shl<Basis<RU, M, RS>>>::Output;
    #[inline(always)]
    fn shr(self, rhs: Basis<LU, M, LS>) -> Self::Output {
        rhs << self
    }
}

#[cfg(test)]
mod tests {
    use crate::vga6d::*;

    #[test]
    fn test_geo_prod() {
        assert!(e12 * e34 == e1234);
        assert!(e34 * e12 == e1234);
        assert!(e12 * e3 == e123);
        assert!(e3 * e12 == e123);
        assert!(e12 * e4 == e124);
        assert!(e4 * e12 == e124);
    }
}
