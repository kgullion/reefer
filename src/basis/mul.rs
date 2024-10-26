use super::{into::IntoBasis, Basis, ZeroVect};
use crate::{
    metric::{Metric, TritMul, TritXor},
    // traits::Inverse,
    utils::{
        parity::{SwapPar, SwapParity},
        Branch, If,
    },
};
use core::ops::{BitAnd, BitXor, Mul};
// use num_traits::Zero;
use typenum::{And, Bit, Eq, IsEqual, Prod, Unsigned, Xor, U0};

// -------------------------------------------------------------------------------------
// Division
// 0/0 = None
impl core::ops::Div<ZeroVect> for ZeroVect {
    type Output = Option<ZeroVect>;
    #[inline(always)]
    fn div(self, _: ZeroVect) -> Self::Output {
        None
    }
}
// B/0 = None
impl<U: Unsigned, M: Metric, S: Bit> core::ops::Div<ZeroVect> for Basis<U, M, S> {
    type Output = Option<ZeroVect>;
    #[inline(always)]
    fn div(self, _: ZeroVect) -> Self::Output {
        None
    }
}
// 0/B = 0
impl<U: Unsigned, M: Metric, S: Bit> core::ops::Div<Basis<U, M, S>> for ZeroVect {
    type Output = Option<ZeroVect>;
    #[inline(always)]
    fn div(self, _: Basis<U, M, S>) -> Self::Output {
        Some(self)
    }
}
// // L/R = L * R.inv
// pub trait MulOpt<Rhs> {
//     type Output;
//     fn mul_opt(self, rhs: Self::Output) -> Option<Self::Output>;
// }
// impl MulOpt<Option<ZeroVect>> for ZeroVect {
//     type Output = ZeroVect;
//     fn mul_opt(self, _: Option<ZeroVect>) -> Option<Self::Output> {
//         None
//     }
// }
// impl<LU: Unsigned, RU: Unsigned, M: Metric, LS: Bit, RS: Bit> core::ops::Div<Basis<RU, M, RS>>
//     for Basis<LU, M, LS>
// where
//     Self: MulOpt<<Basis<RU, M, RS> as Inverse>::Output>,
//     Basis<RU, M, RS>: Inverse,
// {
//     type Output = Option<<Self as MulOpt<<Basis<RU, M, RS> as Inverse>::Output>>::Output>;
//     #[inline(always)]
//     fn div(self, rhs: Basis<RU, M, RS>) -> Self::Output {
//         <Self as MulOpt<<Basis<RU, M, RS> as Inverse>::Output>>::mul_opt(self, rhs.inverse())
//     }
// }

// -------------------------------------------------------------------------------------
// Geometric Product
// 0 * 0 = 0
impl core::ops::Mul<ZeroVect> for ZeroVect {
    type Output = ZeroVect;
    #[inline(always)]
    fn mul(self, _: ZeroVect) -> Self::Output {
        Self::Output::default()
    }
}
// 0 * B = 0
impl<U: Unsigned, M: Metric, S: Bit> core::ops::Mul<Basis<U, M, S>> for ZeroVect {
    type Output = ZeroVect;
    #[inline(always)]
    fn mul(self, _: Basis<U, M, S>) -> Self::Output {
        Self::Output::default()
    }
}
// 0 * B = 0
impl<U: Unsigned, M: Metric, S: Bit> core::ops::Mul<ZeroVect> for Basis<U, M, S> {
    type Output = ZeroVect;
    #[inline(always)]
    fn mul(self, _: ZeroVect) -> Self::Output {
        Self::Output::default()
    }
}
// L * R accounts for parity from the metric, the swaps, the left basis parity, and the right basis parity.
impl<
        L: Unsigned + SwapPar<R, Parity: Bit> + BitXor<R, Output: Unsigned>,
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
        LS: Bit + BitXor<RS>,
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
impl core::ops::BitXor<ZeroVect> for ZeroVect {
    type Output = ZeroVect;
    #[inline(always)]
    fn bitxor(self, _: ZeroVect) -> Self::Output {
        Self::Output::default()
    }
}
// 0 ^ B = B
impl<U: Unsigned, M: Metric, S: Bit> core::ops::BitXor<Basis<U, M, S>> for ZeroVect {
    type Output = Basis<U, M, S>;
    #[inline(always)]
    fn bitxor(self, rhs: Basis<U, M, S>) -> Self::Output {
        rhs
    }
}
// B ^ 0 = B
impl<U: Unsigned, M: Metric, S: Bit> core::ops::BitXor<ZeroVect> for Basis<U, M, S> {
    type Output = Basis<U, M, S>;
    #[inline(always)]
    fn bitxor(self, _: ZeroVect) -> Self::Output {
        self
    }
}
// LB ^ RB = GP if no overlap, else Zero
impl<LU: Unsigned, RU: Unsigned, M: Metric, LS: Bit, RS: Bit> core::ops::BitXor<Basis<RU, M, RS>>
    for Basis<LU, M, LS>
where
    Self: Mul<Basis<RU, M, RS>>,
    LU: BitAnd<
        RU,
        Output: IsEqual<
            U0,
            Output: Branch<Prod<Basis<LU, M, LS>, Basis<RU, M, RS>>, ZeroVect, Output: Default>,
        >,
    >,
{
    //   Output =       LU & RU == 0    ?            LB        *       RB         : Zero
    type Output = If<Eq<And<LU, RU>, U0>, Prod<Basis<LU, M, LS>, Basis<RU, M, RS>>, ZeroVect>;
    #[inline(always)]
    fn bitxor(self, _: Basis<RU, M, RS>) -> Self::Output {
        Self::Output::default()
    }
}

// -------------------------------------------------------------------------------------
// Inner Product
// 0 | 0 = 0
impl core::ops::BitOr<ZeroVect> for ZeroVect {
    type Output = ZeroVect;
    #[inline(always)]
    fn bitor(self, _: ZeroVect) -> Self::Output {
        Self::Output::default()
    }
}
// 0 | B = 0
impl<U: Unsigned, M: Metric, S: Bit> core::ops::BitOr<Basis<U, M, S>> for ZeroVect {
    type Output = ZeroVect;
    #[inline(always)]
    fn bitor(self, _: Basis<U, M, S>) -> Self::Output {
        Self::Output::default()
    }
}
// B | 0 = 0
impl<U: Unsigned, M: Metric, S: Bit> core::ops::BitOr<ZeroVect> for Basis<U, M, S> {
    type Output = ZeroVect;
    #[inline(always)]
    fn bitor(self, _: ZeroVect) -> Self::Output {
        Self::Output::default()
    }
}
// LB | RB = GP if any overlap, else Zero
impl<LU: Unsigned, RU: Unsigned, M: Metric, LS: Bit, RS: Bit> core::ops::BitOr<Basis<RU, M, RS>>
    for Basis<LU, M, LS>
where
    Self: Mul<Basis<RU, M, RS>>,
    LU: BitAnd<
        RU,
        Output: IsEqual<
            U0,
            Output: Branch<ZeroVect, Prod<Basis<LU, M, LS>, Basis<RU, M, RS>>, Output: Default>,
        >,
    >,
{
    //   Output =       LU & RU == 0    ? Zero                   LB        *       RB
    type Output = If<Eq<And<LU, RU>, U0>, ZeroVect, Prod<Basis<LU, M, LS>, Basis<RU, M, RS>>>;
    #[inline(always)]
    fn bitor(self, _: Basis<RU, M, RS>) -> Self::Output {
        Self::Output::default()
    }
}

// -------------------------------------------------------------------------------------
// Left Contraction
// 0 << 0 = 0
impl core::ops::Shl<ZeroVect> for ZeroVect {
    type Output = ZeroVect;
    #[inline(always)]
    fn shl(self, _: ZeroVect) -> Self::Output {
        Self::Output::default()
    }
}
// 0 << B = 0
impl<U: Unsigned, M: Metric, S: Bit> core::ops::Shl<Basis<U, M, S>> for ZeroVect {
    type Output = ZeroVect;
    #[inline(always)]
    fn shl(self, _: Basis<U, M, S>) -> Self::Output {
        Self::Output::default()
    }
}
// B << 0 = B
impl<U: Unsigned, M: Metric, S: Bit> core::ops::Shl<ZeroVect> for Basis<U, M, S> {
    type Output = Basis<U, M, S>;
    #[inline(always)]
    fn shl(self, _: ZeroVect) -> Self::Output {
        self
    }
}
// LB << RB = GP if R&L==L (R covers L), else Zero
impl<LU: Unsigned, RU: Unsigned, M: Metric, LS: Bit, RS: Bit> core::ops::Shl<Basis<RU, M, RS>>
    for Basis<LU, M, LS>
where
    Self: Mul<Basis<RU, M, RS>>,
    LU: BitAnd<
        RU,
        Output: IsEqual<
            LU,
            Output: Branch<Prod<Basis<LU, M, LS>, Basis<RU, M, RS>>, ZeroVect, Output: Default>,
        >,
    >,
{
    //   Output =       LU & RU == LU    ?            LB        *       RB         : Zero
    type Output = If<Eq<And<LU, RU>, LU>, Prod<Basis<LU, M, LS>, Basis<RU, M, RS>>, ZeroVect>;
    #[inline(always)]
    fn shl(self, _: Basis<RU, M, RS>) -> Self::Output {
        Self::Output::default()
    }
}

// -------------------------------------------------------------------------------------
// Right Contraction is the same as Left Contraction, but with the roles of L and R reversed.
// 0 >> 0 = 0
impl core::ops::Shr<ZeroVect> for ZeroVect {
    type Output = ZeroVect;
    #[inline(always)]
    fn shr(self, _: ZeroVect) -> Self::Output {
        Self::Output::default()
    }
}
// 0 >> B = 0
impl<U: Unsigned, M: Metric, S: Bit> core::ops::Shr<Basis<U, M, S>> for ZeroVect {
    type Output = ZeroVect;
    #[inline(always)]
    fn shr(self, _: Basis<U, M, S>) -> Self::Output {
        Self::Output::default()
    }
}
// B >> 0 = B
impl<U: Unsigned, M: Metric, S: Bit> core::ops::Shr<ZeroVect> for Basis<U, M, S> {
    type Output = Basis<U, M, S>;
    #[inline(always)]
    fn shr(self, _: ZeroVect) -> Self::Output {
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
