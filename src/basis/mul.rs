use crate::{
    basis::{into::IntoBasis, Basis, ZeroVect},
    marker::{
        CommutatorMarker, FatDotMarker, InnerProdMarker, LeftContractionMarker, MarkedProd,
        MulMarker, OuterProdMarker, RightContractionMarker, ScalarProdMarker,
    },
    metric::{Metric, TritMul, TritXor},
    parity::{SwapPar, SwapParity},
    traits::{Commutator, Dual, FatDot, Inverse, Sandwich, ScalarProduct, Undual},
    utils::{Branch, If},
};
use core::ops::{BitAnd, BitOr, BitXor, Div, Mul, Shl, Shr};
use typenum::{Bit, Prod, Unsigned, Xor};

// -------------------------------------------------------------------------------------
// Geometric Product
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
    > Mul<Basis<R, M, RS>> for Basis<L, M, LS>
{
    //   Output =      M |> TritMul<L, R>          |> TritXor<LS>          |> TritXor<RS>          |> TritXor<SwapParity<L, R>>          |> IntoBasis<Xor<L, R>, M>;
    type Output = <<<<<M as TritMul<L, R>>::Output as TritXor<LS>>::Output as TritXor<RS>>::Output as TritXor<SwapParity<L, R>>>::Output as IntoBasis<Xor<L, R>, M>>::Output;
    #[inline(always)]
    fn mul(self, _: Basis<R, M, RS>) -> Self::Output {
        Self::Output::default()
    }
}

// -------------------------------------------------------------------------------------
// macro to generate 0*0=0, 0*B=0, and B*0=0
macro_rules! build_zero_prod {
    ($trait:ident, $fn:ident) => {
        impl<M: Metric> $trait<ZeroVect<M>> for ZeroVect<M> {
            type Output = ZeroVect<M>;
            #[inline(always)]
            fn $fn(self, _: ZeroVect<M>) -> Self::Output {
                Self::Output::default()
            }
        }
        impl<U: Unsigned, M: Metric, S: Bit> $trait<Basis<U, M, S>> for ZeroVect<M> {
            type Output = ZeroVect<M>;
            #[inline(always)]
            fn $fn(self, _: Basis<U, M, S>) -> Self::Output {
                Self::Output::default()
            }
        }
        impl<U: Unsigned, M: Metric, S: Bit> $trait<ZeroVect<M>> for Basis<U, M, S> {
            type Output = ZeroVect<M>;
            #[inline(always)]
            fn $fn(self, _: ZeroVect<M>) -> Self::Output {
                Self::Output::default()
            }
        }
    };
}
// macro to generate marker based geometric products
macro_rules! build_marker_prod {
    ($marker:ident, $trait:ident, $fn:ident) => {
        build_zero_prod!($trait, $fn);
        impl<RU: Unsigned, LU: Unsigned, M: Metric, RS: Bit, LS: Bit> $trait<Basis<RU, M, RS>>
            for Basis<LU, M, LS>
        where
            Self: Mul<Basis<RU, M, RS>>,
            $marker: MulMarker<
                LU,
                RU,
                Output: Branch<
                    Prod<Basis<LU, M, LS>, Basis<RU, M, RS>>,
                    ZeroVect<M>,
                    Output: Default,
                >,
            >,
        {
            type Output = If<
                MarkedProd<$marker, LU, RU>,
                Prod<Basis<LU, M, LS>, Basis<RU, M, RS>>,
                ZeroVect<M>,
            >;
            #[inline(always)]
            fn $fn(self, _: Basis<RU, M, RS>) -> Self::Output {
                Self::Output::default()
            }
        }
    };
}

build_zero_prod!(Mul, mul);
build_marker_prod!(OuterProdMarker, BitXor, bitxor);
build_marker_prod!(InnerProdMarker, BitOr, bitor);
build_marker_prod!(LeftContractionMarker, Shl, shl);
build_marker_prod!(RightContractionMarker, Shr, shr);
build_marker_prod!(CommutatorMarker, Commutator, commutator);
build_marker_prod!(ScalarProdMarker, ScalarProduct, scalar_prod);
build_marker_prod!(FatDotMarker, FatDot, fat_dot);

// -------------------------------------------------------------------------------------
// Regressive Product
build_zero_prod!(BitAnd, bitand);
impl<L: Unsigned, R: Unsigned, M: Metric, LS: Bit + BitXor<RS>, RS: Bit> BitAnd<Basis<R, M, RS>>
    for Basis<L, M, LS>
where
    Basis<L, M, LS>: Dual<Output: BitXor<<Basis<R, M, RS> as Dual>::Output, Output: Undual>>,
    Basis<R, M, RS>: Dual,
{
    type Output =
        <Xor<<Basis<L, M, LS> as Dual>::Output, <Basis<R, M, RS> as Dual>::Output> as Undual>::Output;
    fn bitand(self, rhs: Basis<R, M, RS>) -> Self::Output {
        (self.dual() ^ rhs.dual()).undual()
    }
}

// -------------------------------------------------------------------------------------
// Sandwich Product
// 0 >>> 0 = None
impl<M: Metric> Sandwich<ZeroVect<M>> for ZeroVect<M> {
    type Output = ZeroVect<M>;
    #[inline(always)]
    fn sandwich(self, _: ZeroVect<M>) -> Option<Self::Output> {
        None
    }
}
// 0 >>> B = None
impl<R: Unsigned, M: Metric, RS: Bit> Sandwich<Basis<R, M, RS>> for ZeroVect<M> {
    type Output = ZeroVect<M>;
    #[inline(always)]
    fn sandwich(self, _: Basis<R, M, RS>) -> Option<Self::Output> {
        None
    }
}
// B >>> 0 = 0
impl<L: Unsigned, M: Metric, LS: Bit> Sandwich<ZeroVect<M>> for Basis<L, M, LS> {
    type Output = ZeroVect<M>;
    #[inline(always)]
    fn sandwich(self, rhs: ZeroVect<M>) -> Option<Self::Output> {
        Some(rhs)
    }
}
// LB >>> RB = LB * RB * LB⁻¹
impl<L: Unsigned, R: Unsigned, M: Metric, LS: Bit + BitXor<RS>, RS: Bit> Sandwich<Basis<R, M, RS>>
    for Basis<L, M, LS>
where
    Self: Mul<Basis<R, M, RS>, Output: Mul<<Self as Inverse>::Output>> + Inverse,
{
    type Output = Prod<Prod<Self, Basis<R, M, RS>>, <Self as Inverse>::Output>;
    fn sandwich(self, rhs: Basis<R, M, RS>) -> Option<Self::Output> {
        Some(self.clone() * rhs * self.inverse()?)
    }
}

// -------------------------------------------------------------------------------------
// Division
// 0/0 = None
impl<M: Metric> Div<ZeroVect<M>> for ZeroVect<M> {
    type Output = Option<ZeroVect<M>>;
    #[inline(always)]
    fn div(self, _: ZeroVect<M>) -> Self::Output {
        None
    }
}
// B/0 = None
impl<U: Unsigned, M: Metric, S: Bit> Div<ZeroVect<M>> for Basis<U, M, S> {
    type Output = Option<ZeroVect<M>>;
    #[inline(always)]
    fn div(self, _: ZeroVect<M>) -> Self::Output {
        None
    }
}
// 0/B = 0
impl<U: Unsigned, M: Metric, S: Bit> Div<Basis<U, M, S>> for ZeroVect<M> {
    type Output = Option<ZeroVect<M>>;
    #[inline(always)]
    fn div(self, _: Basis<U, M, S>) -> Self::Output {
        Some(self)
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
