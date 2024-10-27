#![allow(unused)]
use crate::{
    basis::into::IntoBasis,
    basis::{Basis, ZeroVect},
    metric::Metric,
    traits::{Dual, Undual},
    utils::parity::{ReversePar, ReverseParity, SwapPar, SwapParity},
};
use core::ops::BitXor;
use typenum::{Bit, Unsigned, Xor};

// // PsuedoScalar
// pub trait PseudoScalar {
//     type Output: BasisInfo;
// }
// impl PseudoScalar for ZeroVector {
//     type Output = ZeroVector;
// }
// impl<U: Unsigned, M: Metric> PseudoScalar for Basis<U, M, B0>
// where
//     Self: BasisInfo,
//     M::Psuedoscalar: Unsigned,
//     Basis<M::Psuedoscalar, M, B0>: BasisInfo,
// {
//     type Output = Basis<M::Psuedoscalar, M, B0>;
// }
// // TODO: hook this back in to allow for easy access to subspaces (or maybe another mechanism?)

// ------------------------
// X * X.dual = I -> X.dual = X.rev * I (assuming pos metric)
// X=U, I=M::Psuedoscalar
impl<
        U: Unsigned
            + SwapPar<M::Psuedoscalar>
            + BitXor<M::Psuedoscalar, Output: Unsigned>
            + ReversePar<
                Parity: BitXor<
                    SwapParity<U, M::Psuedoscalar>,
                    Output: BitXor<
                        S,
                        Output: IntoBasis<Xor<U, M::Psuedoscalar>, M, Output: Default>,
                    >,
                >,
            >,
        M: Metric,
        S: Bit,
    > Dual for Basis<U, M, S>
{
    //   Output =    ReverseParity<U> |> BitXor<SwapParity<U, M::Psuedoscalar>>          |> BitXor<S>          |> IntoBasis<Xor<U, M::Psuedoscalar>, M>
    type Output = <<<ReverseParity<U> as BitXor<SwapParity<U, M::Psuedoscalar>>>::Output as BitXor<S>>::Output as IntoBasis<Xor<U, M::Psuedoscalar>, M>>::Output;
    #[inline(always)]
    fn dual(self) -> Self::Output {
        Self::Output::default()
    }
}
impl Dual for ZeroVect {
    type Output = ZeroVect;
    #[inline(always)]
    fn dual(self) -> Self::Output {
        Self::Output::default()
    }
}

// ------------------------
// X * X.dual = I -> X = I * X.dual.rev (assuming pos metric)
// X.dual=U, I=M::Psuedoscalar
impl<
        U: Unsigned
            + BitXor<M::Psuedoscalar, Output: Unsigned>
            + ReversePar<
                Parity: BitXor<
                    SwapParity<M::Psuedoscalar, U>,
                    Output: BitXor<
                        S,
                        Output: IntoBasis<Xor<U, M::Psuedoscalar>, M, Output: Default>,
                    >,
                >,
            >,
        M: Metric,
        S: Bit,
    > Undual for Basis<U, M, S>
where
    <M as Metric>::Psuedoscalar: SwapPar<U>,
{
    //   Output =    ReverseParity<U> |> BitXor<SwapParity<M::Psuedoscalar, U>>          |> BitXor<S>          |> IntoBasis<Xor<U, M::Psuedoscalar>, M>
    type Output = <<<ReverseParity<U> as BitXor<SwapParity<M::Psuedoscalar, U>>>::Output as BitXor<S>>::Output as IntoBasis<Xor<U, M::Psuedoscalar>, M>>::Output;
    #[inline(always)]
    fn undual(self) -> Self::Output {
        Self::Output::default()
    }
}
impl Undual for ZeroVect {
    type Output = ZeroVect;
    #[inline(always)]
    fn undual(self) -> Self::Output {
        Self::Output::default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vga3d::{scalar as e, x, xy, xyz, xz, y, yz, z};

    #[test]
    fn dual() {
        assert!(e * e.dual() == xyz);
        assert!(x * x.dual() == xyz);
        assert!(y * y.dual() == xyz);
        assert!(z * z.dual() == xyz);
        assert!(xy * xy.dual() == xyz);
        assert!(xz * xz.dual() == xyz);
        assert!(yz * yz.dual() == xyz);
        assert!(xyz * xyz.dual() == xyz);
    }

    #[test]
    fn undual() {
        assert!(e.undual() * e == xyz);
        assert!(x.undual() * x == xyz);
        assert!(y.undual() * y == xyz);
        assert!(z.undual() * z == xyz);
        assert!(xy.undual() * xy == xyz);
        assert!(xz.undual() * xz == xyz);
        assert!(yz.undual() * yz == xyz);
        assert!(xyz.undual() * xyz == xyz);
    }
}
