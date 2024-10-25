use super::{into::IntoBasis, Basis, ZeroVector};
use crate::{
    metric::{Metric, TritMul, TritXor},
    traits::GeometricProduct,
    utils::{parity::SwapPar, SwapParity},
};
use core::ops::BitXor;
use typenum::{Bit, Unsigned, Xor};

// 0 * 0 = 0
impl GeometricProduct<ZeroVector> for ZeroVector {
    type Output = ZeroVector;
    #[inline(always)]
    fn geo_prod(self, _: ZeroVector) -> Self::Output {
        Self::Output::default()
    }
}
// 0 * B = 0
impl<U: Unsigned, M: Metric, S: Bit> GeometricProduct<Basis<U, M, S>> for ZeroVector {
    type Output = ZeroVector;
    #[inline(always)]
    fn geo_prod(self, _: Basis<U, M, S>) -> Self::Output {
        Self::Output::default()
    }
}
// 0 * B = 0
impl<U: Unsigned, M: Metric, S: Bit> GeometricProduct<ZeroVector> for Basis<U, M, S> {
    type Output = ZeroVector;
    #[inline(always)]
    fn geo_prod(self, _: ZeroVector) -> Self::Output {
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
    > GeometricProduct<Basis<R, M, RS>> for Basis<L, M, LS>
{
    //   Output =      M |> TritMul<L, R>          |> TritXor<LS>          |> TritXor<RS>          |> TritXor<SwapParity<L, R>>          |> IntoBasis<Xor<L, R>, M>;
    type Output = <<<<<M as TritMul<L, R>>::Output as TritXor<LS>>::Output as TritXor<RS>>::Output as TritXor<SwapParity<L, R>>>::Output as IntoBasis<Xor<L, R>, M>>::Output;
    #[inline(always)]
    fn geo_prod(self, _: Basis<R, M, RS>) -> Self::Output {
        Self::Output::default()
    }
}

// TODO: test this
