#![allow(unused)]
use crate::{
    basis::{into::IntoBasis, Basis, ZeroVect},
    metric::Metric,
    parity::{DualPar, DualParity, ReversePar, ReverseParity, UndualPar, UndualParity},
    traits::{Dual, Undual},
    utils::count::CountOf,
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
impl<
        U: Unsigned
            + BitXor<M::Psuedoscalar, Output: Unsigned>
            + DualPar<M, Parity: BitXor<S, Output: Bit>>,
        M: Metric,
        S: Bit,
    > Dual for Basis<U, M, S>
{
    type Output = Basis<Xor<U, M::Psuedoscalar>, M, Xor<DualParity<U, M>, S>>;
    #[inline(always)]
    fn dual(self) -> Self::Output {
        Self::Output::default()
    }
}
impl<M: Metric> Dual for ZeroVect<M> {
    type Output = ZeroVect<M>;
    #[inline(always)]
    fn dual(self) -> Self::Output {
        Self::Output::default()
    }
}

impl<
        U: Unsigned
            + BitXor<M::Psuedoscalar, Output: Unsigned>
            + UndualPar<M, Parity: BitXor<S, Output: Bit>>,
        M: Metric,
        S: Bit,
    > Undual for Basis<U, M, S>
{
    type Output = Basis<Xor<U, M::Psuedoscalar>, M, Xor<UndualParity<U, M>, S>>;
    #[inline(always)]
    fn undual(self) -> Self::Output {
        Self::Output::default()
    }
}
impl<M: Metric> Undual for ZeroVect<M> {
    type Output = ZeroVect<M>;
    #[inline(always)]
    fn undual(self) -> Self::Output {
        Self::Output::default()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn dual_vga3d() {
        use crate::{
            traits::Dual,
            vga3d::{scalar as e, x, xy, xyz, xz, y, yz, z},
        };

        assert!(e.dual() == xyz);
        assert!(x.dual() == yz);
        assert!(y.dual() == -xz);
        assert!(z.dual() == xy);
        assert!(xy.dual() == -z);
        assert!(xz.dual() == y);
        assert!(yz.dual() == -x);
        assert!(xyz.dual() == -e);
    }

    #[test]
    fn undual_vga3d() {
        use crate::{
            traits::{Dual, Undual},
            vga3d::{scalar as e, x, xy, xyz, xz, y, yz, z},
        };
        assert!(e.dual().undual() == e);
        assert!(x.dual().undual() == x);
        assert!(y.dual().undual() == y);
        assert!(z.dual().undual() == z);
        assert!(xy.dual().undual() == xy);
        assert!(xz.dual().undual() == xz);
        assert!(yz.dual().undual() == yz);
        assert!(xyz.dual().undual() == xyz);
    }

    #[test]
    fn dual_vga6d() {
        use crate::{
            traits::{Dual, Undual},
            vga6d::*,
        };
        assert!(e.dual() == e123456);
        assert!(e1.dual() == e23456);
        assert!(e2.dual() == -e13456);
        assert!(e3.dual() == e12456);
        assert!(e4.dual() == -e12356);
        assert!(e5.dual() == e12346);
        assert!(e6.dual() == -e12345);
        assert!(e12.dual() == -e3456);
        assert!(e13.dual() == e2456);
        assert!(e14.dual() == -e2356);
        assert!(e15.dual() == e2346);
        assert!(e16.dual() == -e2345);
        assert!(e23.dual() == -e1456);
        assert!(e24.dual() == e1356);
        assert!(e25.dual() == -e1346);
        assert!(e26.dual() == e1345);
        assert!(e34.dual() == -e1256);
        assert!(e35.dual() == e1246);
        assert!(e36.dual() == -e1245);
        assert!(e45.dual() == -e1236);
        assert!(e46.dual() == e1235);
        assert!(e56.dual() == -e1234);
        assert!(e123.dual() == -e456);
        assert!(e124.dual() == e356);
        assert!(e125.dual() == -e346);
        assert!(e126.dual() == e345);
        assert!(e134.dual() == -e256);
        assert!(e135.dual() == e246);
        assert!(e136.dual() == -e245);
        assert!(e145.dual() == -e236);
        assert!(e146.dual() == e235);
        assert!(e156.dual() == -e234);
        assert!(e234.dual() == e156);
        assert!(e235.dual() == -e146);
        assert!(e236.dual() == e145);
        assert!(e245.dual() == e136);
        assert!(e246.dual() == -e135);
        assert!(e256.dual() == e134);
        assert!(e345.dual() == -e126);
        assert!(e346.dual() == e125);
        assert!(e356.dual() == -e124);
        assert!(e456.dual() == e123);
        assert!(e1234.dual() == e56);
        assert!(e1235.dual() == -e46);
        assert!(e1236.dual() == e45);
        assert!(e1245.dual() == e36);
        assert!(e1246.dual() == -e35);
        assert!(e1256.dual() == e34);
        assert!(e1345.dual() == -e26);
        assert!(e1346.dual() == e25);
        assert!(e1356.dual() == -e24);
        assert!(e1456.dual() == e23);
        assert!(e2345.dual() == e16);
        assert!(e2346.dual() == -e15);
        assert!(e2356.dual() == e14);
        assert!(e2456.dual() == -e13);
        assert!(e3456.dual() == e12);
        assert!(e12345.dual() == e6);
        assert!(e12346.dual() == -e5);
        assert!(e12356.dual() == e4);
        assert!(e12456.dual() == -e3);
        assert!(e13456.dual() == e2);
        assert!(e23456.dual() == -e1);
        assert!(e123456.dual() == -e);
    }
}
