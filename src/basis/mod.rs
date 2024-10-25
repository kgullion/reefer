pub mod default;
pub mod dual;
pub mod equality;
pub mod geo_prod;
pub mod grade;
pub mod into;

use crate::metric::Metric;
use core::marker::PhantomData;
use typenum::{Bit, Unsigned};

// pub trait TBasis: Default {}
// impl TBasis for ZeroVector {}
// impl<U: Unsigned, M: Metric, S: Bit> TBasis for Basis<U, M, S> {}
// -------------------------------------
/// A Basis is a signed product of unit length basis vectors in a geometric algebra.
pub struct Basis<U: Unsigned, M: Metric, S: Bit>(PhantomData<(U, M, S)>);
// where
//     Self: TBasis;

// the zero vector is a special case, every other basis is length 1
/// Represents the zero vector. You probably don't need to use this directly and got it by multiplying
/// two degenerate Basis elements. Mostly here for the compiler.
pub struct ZeroVector;
// where
//     Self: TBasis;

// // -------------------------------------
// // basis operations
// // const new
// impl<U: Unsigned, M: Metric, S: Bit> Basis<U, M, S>
// where
//     Self: BasisInfo,
// {
//     #[inline(always)]
//     pub const fn new() -> Self {
//         Self(PhantomData)
//     }
// }

// // Neg
// impl<U: Unsigned, M: Metric, S: Not<Output: Bit> + Bit> Neg for Basis<U, M, S>
// where
//     Self: BasisInfo,
//     Basis<U, M, Flip<S>>: BasisInfo,
// {
//     type Output = Basis<U, M, Flip<S>>;
//     /// Negate the basis.
//     #[inline(always)]
//     fn neg(self) -> Self::Output {
//         Basis(PhantomData)
//     }
// }
// impl Neg for ZeroVector {
//     type Output = ZeroVector;
//     #[inline(always)]
//     fn neg(self) -> Self::Output {
//         self // 😌
//     }
// }

// // Involute
// pub trait Involute {
//     type Output;
//     fn involute(self) -> Self::Output;
// }
// impl<U: Unsigned, M: Metric, S: Bit> Involute for Basis<U, M, S>
// where
//     Self: BasisInfo<IsInvolute: BitXor<S, Output: Bit>>,
//     Basis<U, M, Xor<<Self as BasisInfo>::IsInvolute, S>>: BasisInfo,
// {
//     type Output = Basis<U, M, Xor<<Self as BasisInfo>::IsInvolute, S>>;
//     /// Calculate the involute of the basis.
//     #[inline(always)]
//     fn involute(self) -> Self::Output {
//         Basis(PhantomData)
//     }
// }
// impl Involute for ZeroVector {
//     type Output = ZeroVector;
//     #[inline(always)]
//     fn involute(self) -> Self::Output {
//         self // 😌
//     }
// }

// // Reverse
// pub trait Reverse {
//     type Output;
//     fn reverse(self) -> Self::Output;
// }
// impl<U: Unsigned, M: Metric, S: Bit> Reverse for Basis<U, M, S>
// where
//     Self: BasisInfo<IsReverse: BitXor<S, Output: Bit>>,
//     Basis<U, M, Xor<<Self as BasisInfo>::IsReverse, S>>: BasisInfo,
// {
//     type Output = Basis<U, M, Xor<<Self as BasisInfo>::IsReverse, S>>;
//     /// Calculate the reverse of the basis.
//     #[inline(always)]
//     fn reverse(self) -> Self::Output {
//         Basis(PhantomData)
//     }
// }
// impl Reverse for ZeroVector {
//     type Output = ZeroVector;
//     #[inline(always)]
//     fn reverse(self) -> Self::Output {
//         self // 😌
//     }
// }

// // Conjugate
// pub trait Conjugate {
//     type Output;
//     fn conjugate(self) -> Self::Output;
// }
// impl<U: Unsigned, M: Metric, S: Bit> Conjugate for Basis<U, M, S>
// where
//     Self: BasisInfo<IsConjugate: BitXor<S, Output: Bit>>,
//     Basis<U, M, Xor<<Self as BasisInfo>::IsConjugate, S>>: BasisInfo,
// {
//     type Output = Basis<U, M, Xor<<Self as BasisInfo>::IsConjugate, S>>;
//     /// Calculate the conjugate of the basis.
//     #[inline(always)]
//     fn conjugate(self) -> Self::Output {
//         Basis(PhantomData)
//     }
// }
// impl Conjugate for ZeroVector {
//     type Output = ZeroVector;
//     #[inline(always)]
//     fn conjugate(self) -> Self::Output {
//         self // 😌
//     }
// }

// // Inverse
// pub trait Inverse {
//     type Output;
//     /// Calculate the inverse of the basis.
//     fn inverse(self) -> Option<Self::Output>;
// }
// impl<U: Unsigned, M: Metric, S: Bit> Inverse for Basis<U, M, S>
// where
//     Self:
//         BasisInfo<IsReverse: BitXor<S, Output: Bit>> + BasisCart<Self, Mul: BasisCart<ZeroVector>>,
//     Basis<U, M, Xor<<Self as BasisInfo>::IsReverse, S>>: BasisInfo,
// {
//     type Output = Basis<U, M, Xor<<Self as BasisInfo>::IsReverse, S>>;
//     /// The inverse is the element that takes the basis to the identity element.
//     #[inline(always)]
//     fn inverse(self) -> Option<Self::Output> {
//         if IsDegenerate::<Self>::BOOL {
//             None
//         } else {
//             Some(Self::Output::default())
//         }
//     }
// }
