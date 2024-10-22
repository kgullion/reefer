#![allow(unused_imports)]
use crate::{
    basis::{BNew, Basis, BasisCart, BasisInfo, IntoBasis, ZeroVector},
    field::Field,
    metric::Metric,
    utils::{
        typeset::{Intersect, IntersectMerge, Sort, Sorted, Union, UnionMerge},
        Contains, Flat, Flatten, IdxOf, If, IndexOf,
    },
};
use core::marker::PhantomData;
use core::ops::{Add, Mul, Sub};
use generic_array::{ArrayLength, GenericArray};
use typenum::{
    tarr, ATerm, Add1, Bit, Eq, IsEqual, Len, Length, Sum, TArr, TypeArray, Unsigned, B0, B1,
};

/// multivector
#[derive(Clone)]
pub struct Mvect<NS: Len<Output: ArrayLength>, F: Field>(GenericArray<F, Length<NS>>)
where
    Self: MvectInfo;

impl<NS: TypeArray + Len, F: Field> Default for Mvect<NS, F>
where
    Self: MvectInfo,
    Length<NS>: Unsigned + ArrayLength,
{
    /// Create a new multivector from a GenericArray of field elements.
    #[inline(always)]
    fn default() -> Self {
        Mvect(GenericArray::<F, Length<NS>>::default())
    }
}

impl<NS: TypeArray + Len, F: Field> Len for Mvect<NS, F>
where
    Self: MvectInfo,
    Length<NS>: Unsigned + ArrayLength,
{
    type Output = Length<NS>;
    fn len(&self) -> Self::Output {
        Self::Output::default()
    }
}

// -------------------------------------
// Mvect comptime info table (currently just used for trait bounds)
pub trait MvectInfo: Default + Len {}
impl<NS: TypeArray + Len<Output: Unsigned + ArrayLength>, F: Field> MvectInfo for Mvect<NS, F> {}

// -------------------------------------
// Placeholder for Mvect cartesian product info table (currently unneeded)
// pub trait MvectCart<Rhs> {
//     type Intersection: MvectInfo;
//     type Union: MvectInfo;
//     type Mul: MvectInfo;
// }

// impl<
//         L: TypeArray
//             + Len<Output: ArrayLength>
//             + IntersectMerge<R, Output: TypeArray + Len<Output: ArrayLength>>
//             + UnionMerge<R, Output: TypeArray + Len<Output: ArrayLength>>,
//         R: TypeArray + Len<Output: ArrayLength>,
//         F: Field,
//     > MvectCart<Mvect<R, F>> for Mvect<L, F>
// where
//     Self: MvectInfo,
//     Mvect<R, F>: MvectInfo,
// {
//     type Intersection = Mvect<Intersect<L, R>, F>;
//     type Union = Mvect<Union<L, R>, F>;
//     type Mul = Self;
// }

// --------------------------------------------
// multivector addition

/// MvAdd is used to recursively generate code at compile time from the BasisInfo TypeArray
///
/// all of the fn calls should be inlined by the compiler.
/// note that none of actual calls are self-recursing,
/// each is just calling the next fn in the chain
pub trait MvAdd<L: TypeArray, R: TypeArray>
where
    Self: Field,
{
    fn add(out: &mut [Self], lhs: &[Self], rhs: &[Self]);
}
// 0 + 0 = 0
impl<F: Field> MvAdd<ATerm, ATerm> for F {
    #[inline(always)]
    fn add(_out: &mut [F], _lhs: &[F], _rhs: &[F]) {
        #[cfg(feature = "assert-invariants")]
        {
            assert_eq!(_out.len(), 0);
            assert_eq!(_lhs.len(), 0);
            assert_eq!(_rhs.len(), 0);
        }
    }
}
// 0 + B = B
impl<F: Field + MvAdd<ATerm, B>, R: Unsigned, B: TypeArray> MvAdd<ATerm, TArr<R, B>> for F {
    #[inline(always)]
    fn add(out: &mut [F], lhs: &[F], rhs: &[F]) {
        #[cfg(feature = "assert-invariants")]
        {
            assert_eq!(lhs.len(), 0);
            assert_eq!(rhs.len(), Length::<B>::USIZE + 1);
        }
        out[0] += rhs[0].clone();
        <F as MvAdd<ATerm, B>>::add(&mut out[1..], lhs, &rhs[1..]);
    }
}
// A + 0 = A
impl<F: Field + MvAdd<A, ATerm>, L: Unsigned, A: TypeArray> MvAdd<TArr<L, A>, ATerm> for F {
    #[inline(always)]
    fn add(out: &mut [F], lhs: &[F], rhs: &[F]) {
        #[cfg(feature = "assert-invariants")]
        {
            assert_eq!(lhs.len(), Length::<A>::USIZE + 1);
            assert_eq!(rhs.len(), 0);
        }
        out[0] += lhs[0].clone();
        <F as MvAdd<A, ATerm>>::add(&mut out[1..], &lhs[1..], rhs);
    }
}
// A + B
impl<F: Field, L: Unsigned, R: Unsigned, A: TypeArray, B: TypeArray> MvAdd<TArr<L, A>, TArr<R, B>>
    for F
where
    for<'a> F: MvAdd<A, B>,
    for<'a> F: MvAdd<TArr<L, A>, B>,
    for<'a> F: MvAdd<A, TArr<R, B>>,
{
    #[inline(always)]
    fn add(out: &mut [F], lhs: &[F], rhs: &[F]) {
        #[cfg(feature = "assert-invariants")]
        {
            assert_eq!(lhs.len(), Length::<A>::USIZE + 1);
            assert_eq!(rhs.len(), Length::<B>::USIZE + 1);
        }
        if L::USIZE < R::USIZE {
            out[0] += lhs[0].clone();
            <F as MvAdd<A, TArr<R, B>>>::add(&mut out[1..], &lhs[1..], rhs);
        } else if L::USIZE > R::USIZE {
            out[0] += rhs[0].clone();
            <F as MvAdd<TArr<L, A>, B>>::add(&mut out[1..], lhs, &rhs[1..]);
        } else {
            out[0] += lhs[0].clone();
            out[0] += rhs[0].clone();
            <F as MvAdd<A, B>>::add(&mut out[1..], &lhs[1..], &rhs[1..]);
        }
    }
}
// Add impl does the Type-level setup and calls the MvAdd impl
impl<
        Lhs: TypeArray + Len<Output: ArrayLength> + UnionMerge<Rhs, Output: Len<Output: ArrayLength>>,
        Rhs: TypeArray + Len<Output: ArrayLength>,
        F: Field + MvAdd<Lhs, Rhs>,
    > Add<&Mvect<Rhs, F>> for &Mvect<Lhs, F>
where
    Mvect<Lhs, F>: MvectInfo,
    Mvect<Rhs, F>: MvectInfo,
{
    type Output = Mvect<Union<Lhs, Rhs>, F>;
    #[inline(always)]
    fn add(self, rhs: &Mvect<Rhs, F>) -> Self::Output {
        let mut out = GenericArray::<F, Length<Union<Lhs, Rhs>>>::default();
        <F as MvAdd<Lhs, Rhs>>::add(&mut out, &self.0, &rhs.0);
        Mvect(out)
    }
}
// Basically the same as the above impl, but without the &s. Gives "expected" syntax for Field types that derive Copy
impl<
        Lhs: TypeArray + Len<Output: ArrayLength> + UnionMerge<Rhs, Output: Len<Output: ArrayLength>>,
        Rhs: TypeArray + Len<Output: ArrayLength>,
        F: Field + MvAdd<Lhs, Rhs>,
    > Add<Mvect<Rhs, F>> for Mvect<Lhs, F>
where
    Mvect<Lhs, F>: MvectInfo,
    Mvect<Rhs, F>: MvectInfo,
{
    type Output = Mvect<Union<Lhs, Rhs>, F>;
    #[inline(always)]
    fn add(self, rhs: Mvect<Rhs, F>) -> Self::Output {
        &self + &rhs
    }
}

// --------------------------------------------
// multivector subtraction - see addition above
pub trait MvSub<L: TypeArray, R: TypeArray>
where
    Self: Field,
{
    fn sub(out: &mut [Self], lhs: &[Self], rhs: &[Self]);
}
impl<F: Field> MvSub<ATerm, ATerm> for F {
    #[inline(always)]
    fn sub(_out: &mut [F], _lhs: &[F], _rhs: &[F]) {
        #[cfg(feature = "assert-invariants")]
        {
            assert_eq!(_out.len(), 0);
            assert_eq!(_lhs.len(), 0);
            assert_eq!(_rhs.len(), 0);
        }
    }
}
impl<F: Field + MvSub<ATerm, B>, R: Unsigned, B: TypeArray> MvSub<ATerm, TArr<R, B>> for F {
    #[inline(always)]
    fn sub(out: &mut [F], lhs: &[F], rhs: &[F]) {
        #[cfg(feature = "assert-invariants")]
        {
            assert_eq!(lhs.len(), 0);
            assert_eq!(rhs.len(), Length::<B>::USIZE + 1);
        }
        out[0] -= rhs[0].clone();
        <F as MvSub<ATerm, B>>::sub(&mut out[1..], lhs, &rhs[1..]);
    }
}
impl<F: Field + MvSub<A, ATerm>, L: Unsigned, A: TypeArray> MvSub<TArr<L, A>, ATerm> for F {
    #[inline(always)]
    fn sub(out: &mut [F], lhs: &[F], rhs: &[F]) {
        #[cfg(feature = "assert-invariants")]
        {
            assert_eq!(lhs.len(), Length::<A>::USIZE + 1);
            assert_eq!(rhs.len(), 0);
        }
        out[0] -= lhs[0].clone();
        <F as MvSub<A, ATerm>>::sub(&mut out[1..], &lhs[1..], rhs);
    }
}
impl<F: Field, L: Unsigned, R: Unsigned, A: TypeArray, B: TypeArray> MvSub<TArr<L, A>, TArr<R, B>>
    for F
where
    for<'a> F: MvSub<A, B>,
    for<'a> F: MvSub<TArr<L, A>, B>,
    for<'a> F: MvSub<A, TArr<R, B>>,
{
    #[inline(always)]
    fn sub(out: &mut [F], lhs: &[F], rhs: &[F]) {
        #[cfg(feature = "assert-invariants")]
        {
            assert_eq!(lhs.len(), Length::<A>::USIZE + 1);
            assert_eq!(rhs.len(), Length::<B>::USIZE + 1);
        }
        if L::USIZE < R::USIZE {
            out[0] -= lhs[0].clone();
            <F as MvSub<A, TArr<R, B>>>::sub(&mut out[1..], &lhs[1..], rhs);
        } else if L::USIZE > R::USIZE {
            out[0] -= rhs[0].clone();
            <F as MvSub<TArr<L, A>, B>>::sub(&mut out[1..], lhs, &rhs[1..]);
        } else {
            out[0] -= lhs[0].clone();
            out[0] -= rhs[0].clone();
            <F as MvSub<A, B>>::sub(&mut out[1..], &lhs[1..], &rhs[1..]);
        }
    }
}
impl<
        Lhs: TypeArray + Len<Output: ArrayLength> + UnionMerge<Rhs, Output: Len<Output: ArrayLength>>,
        Rhs: TypeArray + Len<Output: ArrayLength>,
        F: Field + MvSub<Lhs, Rhs>,
    > Sub<&Mvect<Rhs, F>> for &Mvect<Lhs, F>
where
    Mvect<Lhs, F>: MvectInfo,
    Mvect<Rhs, F>: MvectInfo,
{
    type Output = Mvect<Union<Lhs, Rhs>, F>;
    #[inline(always)]
    fn sub(self, rhs: &Mvect<Rhs, F>) -> Self::Output {
        let mut out = GenericArray::<F, Length<Union<Lhs, Rhs>>>::default();
        <F as MvSub<Lhs, Rhs>>::sub(&mut out, &self.0, &rhs.0);
        Mvect(out)
    }
}
impl<
        Lhs: TypeArray + Len<Output: ArrayLength> + UnionMerge<Rhs, Output: Len<Output: ArrayLength>>,
        Rhs: TypeArray + Len<Output: ArrayLength>,
        F: Field + MvSub<Lhs, Rhs>,
    > Sub<Mvect<Rhs, F>> for Mvect<Lhs, F>
where
    Mvect<Lhs, F>: MvectInfo,
    Mvect<Rhs, F>: MvectInfo,
{
    type Output = Mvect<Union<Lhs, Rhs>, F>;
    #[inline(always)]
    fn sub(self, rhs: Mvect<Rhs, F>) -> Self::Output {
        &self - &rhs
    }
}

// --------------------------------------------
// multivector multiplication - this one is a bit more complex, but concept
// of having Mul handle the Type-level setup and call the MvMul impl
pub trait MvMul<C: TypeArray, L: TypeArray, R: TypeArray>
where
    Self: Field,
{
    fn mul(out: &mut [Self], lhs: &[Self], rhs: &[Self]);
}
// 0 * 0 = 0 - noop
impl<F: Field, C: TypeArray> MvMul<C, ATerm, ATerm> for F {
    #[inline(always)]
    fn mul(_out: &mut [F], _lhs: &[F], _rhs: &[F]) {
        #[cfg(feature = "assert-invariants")]
        {
            assert_eq!(_out.len(), 0);
            assert_eq!(_lhs.len(), 0);
            assert_eq!(_rhs.len(), 0);
        }
    }
}
// 0 * B = 0 - noop
impl<F: Field, C: TypeArray, R: Unsigned, B: TypeArray> MvMul<C, ATerm, TArr<R, B>> for F {
    #[inline(always)]
    fn mul(_out: &mut [F], _lhs: &[F], _rhs: &[F]) {
        #[cfg(feature = "assert-invariants")]
        {
            assert_eq!(_lhs.len(), 0);
            assert_eq!(_rhs.len(), Length::<B>::USIZE + 1);
        }
    }
}
// A * 0 = 0 - noop
impl<F: Field, C: TypeArray, L: Unsigned, A: TypeArray> MvMul<C, TArr<L, A>, ATerm> for F {
    #[inline(always)]
    fn mul(_out: &mut [F], _lhs: &[F], _rhs: &[F]) {
        #[cfg(feature = "assert-invariants")]
        {
            assert_eq!(_lhs.len(), Length::<A>::USIZE + 1);
            assert_eq!(_rhs.len(), 0);
        }
    }
}
// A * B = AB
/// inner loop of product - multiple the single element of lhs by each element of rhs
pub trait MvMulInner<C: TypeArray, L: BasisInfo, B: TypeArray>
where
    Self: Field,
{
    fn mul_inner(out: &mut [Self], lhs: &Self, rhs: &[Self]);
}
impl<F: Field, C: TypeArray, L: BasisInfo> MvMulInner<C, L, ATerm> for F {
    #[inline(always)]
    fn mul_inner(_out: &mut [Self], _lhs: &Self, _rhs: &[Self]) {
        #[cfg(feature = "assert-invariants")]
        {
            assert!(_rhs.len() == 0);
        }
    }
}
type BProd<L, R> = <L as BasisCart<R>>::Mul;
impl<
        F: Field + MvMulInner<C, L, B>,
        C: TypeArray + IndexOf<BProd<L, R>>,
        L: BasisInfo + BasisCart<R>,
        R: BasisInfo,
        B: TypeArray,
    > MvMulInner<C, L, TArr<R, B>> for F
where
    BProd<L, R>: BasisInfo + IsEqual<ZeroVector>,
{
    #[inline(always)]
    fn mul_inner(out: &mut [Self], lhs: &Self, rhs: &[Self]) {
        if !Eq::<BProd<L, R>, ZeroVector>::BOOL {
            // TODO: Need to search for unsigned version
            #[cfg(feature = "assert-invariants")]
            {
                assert!(Contains::<C, BProd<L, R>>::BOOL);
            }
            let i = IdxOf::<C, BProd<L, R>>::USIZE;
            if <BProd<L, R> as BasisInfo>::Sign::BOOL {
                out[i] -= lhs.clone() * rhs[0].clone()
            } else {
                out[i] += lhs.clone() * rhs[0].clone()
            }
        }
        <F as MvMulInner<C, L, B>>::mul_inner(out, lhs, &rhs[1..]);
    }
}
// outer loop of product - call the inner loop for each element of lhs
impl<
        F: Field + MvMulInner<C, L, TArr<R, B>>,
        C: TypeArray,
        L: BasisInfo,
        A: TypeArray,
        R: BasisInfo,
        B: TypeArray,
    > MvMul<C, TArr<L, A>, TArr<R, B>> for F
where
    for<'a> F: MvMul<C, A, B>,
    for<'a> F: MvMul<C, TArr<L, A>, B>,
    for<'a> F: MvMul<C, A, TArr<R, B>>,
{
    #[inline(always)]
    fn mul(out: &mut [F], lhs: &[F], rhs: &[F]) {
        #[cfg(feature = "assert-invariants")]
        {
            assert_eq!(lhs.len(), Length::<A>::USIZE + 1);
            assert_eq!(rhs.len(), Length::<B>::USIZE + 1);
        }
        <F as MvMulInner<C, L, TArr<R, B>>>::mul_inner(out, &lhs[0], rhs);
        <F as MvMul<C, A, TArr<R, B>>>::mul(out, &lhs[1..], rhs);
    }
}
// and finally...the Mul impl itself 🎉
impl<
        A: TypeArray
            + Len<Output: ArrayLength>
            + MulBs<B, Output: PositiveBs<Output: TypeArray + Len<Output: ArrayLength>>>,
        B: TypeArray + Len<Output: ArrayLength>,
        F: Field + MvMul<<<A as MulBs<B>>::Output as PositiveBs>::Output, A, B>,
    > Mul<&Mvect<B, F>> for &Mvect<A, F>
where
    Mvect<A, F>: MvectInfo,
    Mvect<B, F>: MvectInfo,
{
    type Output = Mvect<<<A as MulBs<B>>::Output as PositiveBs>::Output, F>;
    fn mul(self, rhs: &Mvect<B, F>) -> Self::Output {
        let mut out = GenericArray::<F, Length<Self::Output>>::default();
        <F as MvMul<<<A as MulBs<B>>::Output as PositiveBs>::Output, A, B>>::mul(
            &mut out, &self.0, &rhs.0,
        );
        Mvect(out)
    }
}

// --------------------------------------------
// these 3 traits do the work of setting up the Type-level info for Mul
// MulBs = [l*r for r in B for l in A].sort().uniq()
// MulBsInner is the inner loop of MulBs
// PositiveBs = abs(bs).remove(ZeroVector) -- used to create the Type-level info for the output Mvect in Mul
pub trait MulBs<Rhs> {
    type Output;
}
pub trait MulBsInner<Rhs> {
    type Output;
}
pub trait PositiveBs {
    type Output;
}

// PositiveBs - make all bs positive, flip sign on negs and drop zeros
impl<U: Unsigned + Len, M: Metric, Bs: PositiveBs> PositiveBs for TArr<Basis<U, M, B0>, Bs>
where
    Basis<U, M, B0>: BasisInfo,
{
    // lookin' good!
    type Output = TArr<Basis<U, M, B0>, Bs>;
}
impl<Bs: Unsigned + Len, M: Metric, A: PositiveBs> PositiveBs for TArr<Basis<Bs, M, B1>, A>
where
    Basis<Bs, M, B0>: BasisInfo,
    Basis<Bs, M, B1>: BasisInfo,
{
    // stay positive!
    type Output = TArr<Basis<Bs, M, B0>, A>;
}
impl<Bs: PositiveBs> PositiveBs for TArr<ZeroVector, Bs> {
    // and don't take no bs!
    type Output = Bs;
}

// MulBs - [[l*r for r in B] for l in A]
impl<L: BasisInfo, R: BasisInfo, A: TypeArray, B: TypeArray + Len<Output: ArrayLength>>
    MulBs<TArr<R, B>> for TArr<L, A>
where
    TArr<L, A>: MulBsInner<R, Output: UnionMerge<<TArr<L, A> as MulBs<B>>::Output>> + MulBs<B>,
    TArr<R, B>: TypeArray,
{
    type Output = Union<
        // inner(l, B) -> l * r for l in A
        <TArr<L, A> as MulBsInner<R>>::Output,
        // for r in B
        <TArr<L, A> as MulBs<B>>::Output,
    >;
}
impl<B: TypeArray> MulBs<B> for ATerm {
    // []
    type Output = ATerm;
}
impl<L: BasisInfo, A: TypeArray> MulBs<ATerm> for TArr<L, A> {
    // []
    type Output = ATerm;
}

// MulBsInner - [l*r for l in A]
impl<
        L: BasisInfo + BasisCart<R>,
        R: BasisInfo,
        A: TypeArray
            + Len<Output: ArrayLength>
            + MulBsInner<R, Output: UnionMerge<tarr![BProd<L, R>]>>,
    > MulBsInner<R> for TArr<L, A>
{
    // [l*r for l in A]
    type Output = Union<
        // for l in A
        <A as MulBsInner<R>>::Output,
        // l * r
        tarr![BProd<L, R>],
    >;
}
impl<B: BasisInfo> MulBsInner<B> for ATerm {
    // []
    type Output = ATerm;
}
