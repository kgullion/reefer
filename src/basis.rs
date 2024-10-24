#![allow(unused_imports)]
use crate::{
    field::Field,
    metric::{IntFromSwapParityWithOverlaps, Metric},
    mvect::Mvect,
    utils::{At, Count, CountOf, Get, IndexOf, SwapPar, SwapParity},
};
use core::marker::PhantomData;
use core::ops::{Add, BitAnd, BitXor, Mul, Neg, Not, Sub};
use generic_array::ArrayLength;
use typenum::{
    tarr, ATerm, Abs, Add1, And, Bit, Eq, Integer, IsEqual, Len, Sub1, Sum, TArr, TypeArray, UInt,
    UTerm, Unsigned, Xor, B0, B1, N1, P1, U0, U1, U2, Z0,
};

// -------------------------------------
/// A Basis is a signed product of unit length basis vectors in a geometric algebra.
pub struct Basis<U: Unsigned, M: Metric, S: Bit>(PhantomData<(U, M, S)>)
where
    Self: BasisInfo;

// the zero vector is a special case, every other basis is length 1
/// Represents the zero vector. You probably don't need to use this directly and got it by multiplying
/// two degenerate Basis elements. Mostly here for the compiler.
pub struct ZeroVector
where
    Self: BasisInfo;

// -------------------------------------
impl Abs for ZeroVector {
    type Output = ZeroVector;
}
impl<U: Unsigned, M: Metric, S: Bit> Abs for Basis<U, M, S>
where
    Basis<U, M, S>: BasisInfo,
    Basis<U, M, B0>: BasisInfo,
{
    type Output = Basis<U, M, B0>;
}
pub type BProd<L, R> = <L as BasisCart<R>>::Mul;
/// IsDegenerate is a type functional alias to check if a basis is degenerate =B*B==0
///
/// It just multiplies a basis with itself and checks if the result is the zero vector.
pub type IsDegenerate<B> = <<B as BasisCart<B>>::Mul as BasisCart<ZeroVector>>::Equal;
/// simple alias for Not, to make the code a bit more readable
pub type Flip<B> = <B as Not>::Output;
/// Basis type alias to construct a new basis type
pub type BNew<U, M, I> = <I as IntoBasis<U, M>>::Output;
pub trait IntoBasis<U: Unsigned, M: Metric> {
    type Output: BasisInfo;
}
impl<U: Unsigned + Len, M: Metric> IntoBasis<U, M> for B0
where
    Basis<U, M, B0>: BasisInfo,
{
    type Output = Basis<U, M, B0>;
}
impl<U: Unsigned + Len, M: Metric> IntoBasis<U, M> for B1
where
    Basis<U, M, B1>: BasisInfo,
{
    type Output = Basis<U, M, B1>;
}
impl<U: Unsigned, M: Metric> IntoBasis<U, M> for Z0 {
    type Output = ZeroVector;
}
impl<U: Unsigned + Len, M: Metric> IntoBasis<U, M> for P1
where
    Basis<U, M, B1>: BasisInfo,
{
    type Output = Basis<U, M, B1>;
}
impl<U: Unsigned + Len, M: Metric> IntoBasis<U, M> for N1
where
    Basis<U, M, B1>: BasisInfo,
{
    type Output = Basis<U, M, B1>;
}

// -------------------------------------
/// Basis comptime info table
pub trait BasisInfo {
    type Grade: Unsigned;
    type Mask: Unsigned;
    type Metric: Metric;
    type Sign: Bit;
    type IsInvolute: Bit;
    type IsReverse: Bit;
    type IsConjugate: Bit;
    type IsZero: Bit;
}
impl BasisInfo for ZeroVector {
    type Grade = U0;
    type Metric = tarr![];
    type Mask = U0;
    type Sign = B0;
    type IsInvolute = B1;
    type IsReverse = B1;
    type IsConjugate = B1;
    type IsZero = B1;
}
impl<U: Unsigned + CountOf<B1> + Add<B1>, M: Metric, S: Bit> BasisInfo for Basis<U, M, S>
where
    Add1<U>: Unsigned,
    U::Count: Unsigned + At<U0> + At<U1> + Add<B1>,
    Add1<U::Count>: At<U1>,
    Get<U::Count, U0>: Not,
    Get<U::Count, U1>: Not,
    Get<Add1<U::Count>, U1>: Not,
    Flip<Get<U::Count, U0>>: Bit,
    Flip<Get<U::Count, U1>>: Bit,
    Flip<Get<Add1<U::Count>, U1>>: Bit,
{
    type Grade = U::Count;
    type Mask = U;
    type Metric = M;
    type Sign = S;
    type IsInvolute = Flip<Get<U::Count, U0>>;
    type IsReverse = Flip<Get<Self::Grade, U1>>;
    type IsConjugate = Flip<Get<Add1<Self::Grade>, U1>>;
    type IsZero = B0;
}

// -------------------------------------
/// Basis Cartesian product comptime info table
pub trait BasisCart<Rhs: BasisInfo> {
    type Left: BasisInfo;
    type Right: BasisInfo;
    type Equal: Bit;
    type Mul: BasisInfo;
    type Dual: BasisInfo;
    type Undual: BasisInfo;
}
// B x 0
impl<U: Unsigned, M: Metric, S: Bit> BasisCart<ZeroVector> for Basis<U, M, S>
where
    Basis<U, M, S>: BasisInfo,
    ZeroVector: BasisInfo,
{
    type Left = Self;
    type Right = ZeroVector;
    type Equal = B0;
    type Mul = ZeroVector;
    type Dual = Self;
    type Undual = Self;
}
// 0 x B
impl<U: Unsigned, M: Metric, S: Bit> BasisCart<Basis<U, M, S>> for ZeroVector
where
    Basis<U, M, S>: BasisInfo,
    ZeroVector: BasisInfo,
{
    type Left = ZeroVector;
    type Right = Basis<U, M, S>;
    type Equal = B0;
    type Mul = ZeroVector;
    type Dual = ZeroVector;
    type Undual = ZeroVector;
}
// 0 x 0
impl BasisCart<ZeroVector> for ZeroVector
where
    ZeroVector: BasisInfo,
{
    type Left = ZeroVector;
    type Right = ZeroVector;
    type Equal = B1;
    type Mul = ZeroVector;
    type Dual = ZeroVector;
    type Undual = ZeroVector;
}
// B x B
// type BSwap<L, R> = <L as SwapPar<R, Get<<L as BasisInfo>::Grade, U0>, B0>>::Parity;
impl<U: Unsigned, RU: Unsigned, M: Metric, S: Bit, RS: Bit> BasisCart<Basis<RU, M, RS>>
    for Basis<U, M, S>
where
    Self: BasisInfo,
    Basis<RU, M, RS>: BasisInfo,
    // Equal
    U: IsEqual<RU, Output: BitAnd<Eq<S, RS>, Output: Bit>>,
    S: IsEqual<RS>,
    // Dual
    U: BitXor<RU, Output: Unsigned>
        + CountOf<B1, Count: At<U0>>
        + SwapPar<RU, Get<U::Count, U0>, B0, Parity: IntoBasis<Xor<U, RU>, M>>,
    // Undual
    RU: CountOf<B1, Count: At<U0>>
        + SwapPar<U, Get<RU::Count, U0>, B0, Parity: IntoBasis<Xor<U, RU>, M>>,
    // Mul
    U: BitAnd<RU, Output: Unsigned>,
    M::ZeroMask: BitAnd<And<U, RU>, Output: Unsigned>,
    M::NegMask: BitAnd<And<U, RU>, Output: Unsigned>,
    SwapParity<U, RU>: IntFromSwapParityWithOverlaps<
        And<M::ZeroMask, And<U, RU>>,
        And<M::NegMask, And<U, RU>>,
        Output: IntoBasis<Xor<U, RU>, M>,
    >,
{
    type Left = Self;
    type Right = Basis<RU, M, RS>;
    type Equal = And<Eq<U, RU>, Eq<S, RS>>;
    type Mul = BNew<
        Xor<U, RU>,
        M,
        <SwapParity<U, RU> as IntFromSwapParityWithOverlaps<
            And<M::ZeroMask, And<U, RU>>,
            And<M::NegMask, And<U, RU>>,
        >>::Output,
    >;
    type Dual = BNew<Xor<U, RU>, M, SwapParity<U, RU>>;
    type Undual = BNew<Xor<U, RU>, M, SwapParity<RU, U>>;
}

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

// Default
impl<U: Unsigned, M: Metric, S: Bit> Default for Basis<U, M, S>
where
    Self: BasisInfo,
{
    #[inline(always)]
    /// Create a new basis element.
    fn default() -> Self {
        Self(PhantomData)
    }
}
impl Default for ZeroVector {
    #[inline(always)]
    fn default() -> Self {
        Self
    }
}

// // Graded
// pub trait Graded {
//     /// Number of basis vectors in the basis.
//     fn grade(self) -> usize;
// }
// impl<B: BasisInfo> Graded for B {
//     #[inline(always)]
//     fn grade(self) -> usize {
//         B::Grade::to_usize()
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

// // Eq & PartialEq
// impl core::cmp::Eq for ZeroVector where Self: BasisInfo + BasisCart<Self> {}
// impl<U: Unsigned, M: Metric, S: Bit> core::cmp::Eq for Basis<U, M, S> where
//     Self: BasisInfo + BasisCart<Self>
// {
// }
// impl<R: BasisInfo> PartialEq<R> for ZeroVector
// where
//     ZeroVector: BasisInfo + BasisCart<R>,
// {
//     #[inline(always)]
//     fn eq(&self, _: &R) -> bool {
//         <Self as BasisCart<R>>::Equal::BOOL
//     }
// }
// impl<U: Unsigned, M: Metric, S: Bit, R: BasisInfo> PartialEq<R> for Basis<U, M, S>
// where
//     Self: BasisInfo + BasisCart<R>,
// {
//     #[inline(always)]
//     fn eq(&self, _: &R) -> bool {
//         <Self as BasisCart<R>>::Equal::BOOL
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
// // TODO: implement PseudoScalar for B1

// // Dual
// pub trait Dual<I: BasisInfo = <Self as PseudoScalar>::Output> {
//     type Output;
//     /// Calculate the dual of the basis. You can also choose a different dual subspace
//     /// by passing a different pseudoscalar as the type parameter or using `dual_via`.
//     fn dual(self) -> Self::Output;
//     /// Calculate the dual of the basis using a different pseudoscalar.
//     fn dual_via(self, pseudoscalar: I) -> Self::Output;
// }
// impl<B: BasisInfo + BasisCart<I>, I: BasisInfo> Dual<I> for B {
//     type Output = B::Dual;
//     #[inline(always)]
//     fn dual(self) -> Self::Output {
//         Self::Output::default()
//     }
//     #[inline(always)]
//     fn dual_via(self, _: I) -> Self::Output {
//         Self::Output::default()
//     }
// }

// // Undual
// pub trait Undual<I: BasisInfo = <Self as PseudoScalar>::Output> {
//     type Output;
//     /// Calculate the undual of the basis. You can also choose a different dual subspace
//     /// with `undual_via` or by passing a different pseudoscalar as the type parameter.
//     fn undual(self) -> Self::Output;
//     /// Calculate the undual of the basis using a different pseudoscalar.
//     fn undual_via(self, pseudoscalar: I) -> Self::Output;
// }
// impl<B: BasisInfo + BasisCart<I>, I: BasisInfo> Undual<I> for B {
//     type Output = B::Undual;
//     #[inline(always)]
//     fn undual(self) -> Self::Output {
//         Self::Output::default()
//     }
//     #[inline(always)]
//     fn undual_via(self, _: I) -> Self::Output {
//         Self::Output::default()
//     }
// }

// Mul
impl<R: BasisInfo> Mul<R> for ZeroVector {
    type Output = ZeroVector;
    #[inline(always)]
    fn mul(self, _: R) -> Self::Output {
        Self::Output::default()
    }
}
impl<U: Unsigned, M: Metric, S: Bit, R: BasisInfo> Mul<R> for Basis<U, M, S>
where
    Self: BasisInfo + BasisCart<R, Mul: Default>,
{
    type Output = <Self as BasisCart<R>>::Mul;
    #[inline(always)]
    fn mul(self, _: R) -> Self::Output {
        Self::Output::default()
    }
}
// impl<U: Unsigned, M: Metric, S: Bit, R: BasisInfo> Mul<R> for Basis<U, M, S>
// where
//     Self: BasisInfo + BasisCart<R>,
// {
//     type Output = <Self as BasisCart<R>>::Mul;
//     #[inline(always)]
//     fn mul(self, _: R) -> Self::Output {
//         Self::Output::default()
//     }
// }
// impl<F: Field> Mul<F> for ZeroVector {
//     type Output = Mvect<tarr![], F>;
//     #[inline(always)]
//     fn mul(self, _scalar: F) -> Self::Output {
//         Self::Output::default()
//     }
// }
// impl<F: Field, U: Unsigned, M: Metric, S: Bit> Mul<F> for Basis<U, M, S>
// where
//     Self: BasisInfo,
//     Basis<U, M, B0>: BasisInfo,
//     tarr![Basis<U, M, B0>]: IndexOf<Basis<U, M, B0>>,
// {
//     type Output = Mvect<tarr![Basis<U, M, B0>], F>;
//     #[inline(always)]
//     fn mul(self, scalar: F) -> Self::Output {
//         let mut out = Self::Output::default();
//         out[Basis::<U, M, B0>::new()] = scalar;
//         out
//     }
// }

// // Sub - only defined for operations that result in a basis currently TODO: add others? should the math be saturating or should it jump to Mvect? How to deal with the Field if jumping to Mvect? (probably ends up as a trait bound/ phantomdata on Basis :/ )
// impl Sub<ZeroVector> for ZeroVector {
//     type Output = ZeroVector;
//     #[inline(always)]
//     fn sub(self, _: ZeroVector) -> Self::Output {
//         Self::Output::default()
//     }
// }
// impl<U: Unsigned, M: Metric, S: Bit> Sub<ZeroVector> for Basis<U, M, S>
// where
//     Self: BasisInfo,
// {
//     type Output = Self;
//     #[inline(always)]
//     fn sub(self, _: ZeroVector) -> Self::Output {
//         Self::Output::default()
//     }
// }
// impl<U: Unsigned, M: Metric, S: Bit + Not<Output: Bit>> Sub<Basis<U, M, S>> for ZeroVector
// where
//     Self: BasisInfo,
//     Basis<U, M, S>: BasisInfo,
//     Basis<U, M, Flip<S>>: BasisInfo,
// {
//     type Output = Basis<U, M, Flip<S>>;
//     #[inline(always)]
//     fn sub(self, _: Basis<U, M, S>) -> Self::Output {
//         Self::Output::default()
//     }
// }
// impl<U: Unsigned, M: Metric, S: Bit> Sub<Basis<U, M, S>> for Basis<U, M, S>
// where
//     Self: BasisInfo,
// {
//     type Output = ZeroVector;
//     #[inline(always)]
//     fn sub(self, _: Self) -> Self::Output {
//         Self::Output::default()
//     }
// }
// // Add - see Sub
// impl<R: BasisInfo> Add<R> for ZeroVector {
//     type Output = R;
//     #[inline(always)]
//     fn add(self, _: R) -> Self::Output {
//         Self::Output::default()
//     }
// }
// impl<U: Unsigned, M: Metric, S: Bit> Add<ZeroVector> for Basis<U, M, S>
// where
//     Self: BasisInfo,
// {
//     type Output = Self;
//     #[inline(always)]
//     fn add(self, _: ZeroVector) -> Self::Output {
//         Self::Output::default()
//     }
// }
// impl<U: Unsigned, M: Metric> Add<Basis<U, M, B0>> for Basis<U, M, B1>
// where
//     Basis<U, M, B0>: BasisInfo,
//     Basis<U, M, B1>: BasisInfo,
// {
//     type Output = ZeroVector;
//     #[inline(always)]
//     fn add(self, _: Basis<U, M, B0>) -> Self::Output {
//         Self::Output::default()
//     }
// }
// impl<U: Unsigned, M: Metric> Add<Basis<U, M, B1>> for Basis<U, M, B0>
// where
//     Basis<U, M, B0>: BasisInfo,
//     Basis<U, M, B1>: BasisInfo,
// {
//     type Output = ZeroVector;
//     #[inline(always)]
//     fn add(self, _: Basis<U, M, B1>) -> Self::Output {
//         Self::Output::default()
//     }
// }

// #[cfg(test)]
// mod tests {
//     // use super::BasisBound;
//     use super::*;
//     use crate::mvect::IntoMv;
//     #[allow(unused_imports)]
//     use typenum::{B0, B1, U0, U1, U2, U3, U4, U5, U6, U7};

//     type Metric = tarr![Z0, P1, P1];
//     type PGA2D<U> = Basis<U, Metric, B0>;
//     type E = PGA2D<U0>;
//     type E0 = PGA2D<U1>;
//     type E1 = PGA2D<U2>;
//     type E01 = PGA2D<U3>;
//     type E2 = PGA2D<U4>;
//     type E02 = PGA2D<U5>;
//     type E12 = PGA2D<U6>;
//     type E012 = PGA2D<U7>;

//     #[test]
//     fn test_basis_grade() {
//         assert_eq!(<E as BasisInfo>::Grade::USIZE, 0);
//         // assert_eq!(E0.grade(), 1);
//         // assert_eq!(E1.grade(), 1);
//         // assert_eq!(E01.grade(), 2);
//         // assert_eq!(E2.grade(), 1);
//         // assert_eq!(E02.grade(), 2);
//         // assert_eq!(E12.grade(), 2);
//         // assert_eq!(E012.grade(), 3);
//         assert_eq!(<E0 as BasisInfo>::Grade::USIZE, 1);
//         assert_eq!(<E1 as BasisInfo>::Grade::USIZE, 1);
//         assert_eq!(<E01 as BasisInfo>::Grade::USIZE, 2);
//         assert_eq!(<E2 as BasisInfo>::Grade::USIZE, 1);
//         assert_eq!(<E02 as BasisInfo>::Grade::USIZE, 2);
//         assert_eq!(<E12 as BasisInfo>::Grade::USIZE, 2);
//         assert_eq!(<E012 as BasisInfo>::Grade::USIZE, 3);

//         // <E0 as IntoMv<f32>>::into_mv(E0);
//     }
// }
