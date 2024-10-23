#![allow(unused_imports)]
use crate::{
    basis::{BNew, BProd, Basis, BasisCart, BasisInfo, IntoBasis, ZeroVector},
    collector::{CartXorCollectInto, CartXorCollector, CollectInto, Collector},
    field::Field,
    metric::{IntFromSwapParityWithOverlaps, Metric},
    utils::{
        typeset::{Intersect, IntersectMerge, Sort, Sorted, Union, UnionMerge},
        At, Branch, Contains, Count, CountOf, Flat, Flatten, Get, IdxOf, If, IndexOf, SwapPar,
    },
};
use core::marker::PhantomData;
use core::ops::{Add, BitAnd, BitOr, BitXor, Mul, Not, Sub};
use generic_array::{ArrayLength, GenericArray};
use typenum::{
    tarr, ATerm, Abs, Add1, And, Bit, Eq, IsEqual, IsNotEqual, Len, Length, NotEq, Or, Sum, TArr,
    TypeArray, Unsigned, Xor, B0, B1, U0, U1,
};

/// multivector
#[derive(Clone)]
pub struct Mvect<BS: BasisSet<M> + Len<Output: ArrayLength>, M: Metric, F: Field>(
    GenericArray<F, Length<BS>>,
    PhantomData<M>,
);

/// BasisSet stores the Bitmask of the Basis elements that are present in the multivector. Together with the metric, this is enough to recover each Basis.
pub trait BasisSet<M: Metric>: TypeArray {
    type Output;
}
impl<M: Metric> BasisSet<M> for ATerm {
    type Output = ATerm;
}
impl<BS: BasisSet<M> + Len<Output: Unsigned + ArrayLength + Add<B1>>, U: Unsigned, M: Metric>
    BasisSet<M> for TArr<U, BS>
where
    Basis<U, M, B0>: BasisInfo,
{
    type Output = TArr<U, BS>;
}

impl<BS: BasisSet<M> + Len<Output: Unsigned + ArrayLength>, M: Metric, F: Field> Mvect<BS, M, F> {
    /// Create a new multivector from a GenericArray of field elements.
    #[inline(always)]
    pub fn new(data: GenericArray<F, Length<BS>>) -> Self {
        Mvect(data, PhantomData)
    }
    /// Length of the multivector
    #[inline(always)]
    pub fn len(&self) -> usize {
        self.0.len()
    }
}

// Default - create a new multivector with all elements set to zero
impl<BS: BasisSet<M> + Len<Output: ArrayLength>, M: Metric, F: Field> core::default::Default
    for Mvect<BS, M, F>
// where
//     Self: MvectInfo,
{
    /// Create a new multivector from a GenericArray of field elements.
    #[inline(always)]
    fn default() -> Self {
        Mvect(GenericArray::<F, Length<BS>>::default(), PhantomData)
    }
}

// IntoMv - convert a Basis or ZeroVector type into a Mvect instance
pub trait IntoMv<F: Field> {
    type Output;
    fn into_mv() -> Self::Output;
}
impl<F: Field, U: Unsigned, M: Metric, S: Bit> IntoMv<F> for Basis<U, M, S>
where
    Basis<U, M, S>: BasisInfo,
    Basis<U, M, B0>: BasisInfo,
{
    type Output = Mvect<tarr![U], M, F>;
    fn into_mv() -> Self::Output {
        let mut out = Mvect::<tarr![U], M, F>::default();
        if S::BOOL {
            out.0[0] = -F::one();
        } else {
            out.0[0] = F::one();
        }
        out
    }
}

// PartialEq - compare two multivectors
struct MvPartialEq;
impl<'a, F: Field> Collector<F, bool> for MvPartialEq {
    fn collect_both(out: bool, left: &F, right: &F) -> bool {
        out && left == right
    }
    fn collect_just_left(out: bool, left: &F) -> bool {
        out && left == &F::zero()
    }
    fn collect_just_right(out: bool, right: &F) -> bool {
        out && &F::zero() == right
    }
}
impl<
        BS: BasisSet<M> + Len<Output: ArrayLength>,
        M: Metric,
        F: core::cmp::Eq + Field + CollectInto<F, MvPartialEq, bool, BS, BS>,
    > core::cmp::Eq for Mvect<BS, M, F>
{
}
impl<
        BS: BasisSet<M> + Len<Output: ArrayLength>,
        RBS: BasisSet<M> + Len<Output: ArrayLength>,
        M: Metric,
        F: Field + CollectInto<F, MvPartialEq, bool, BS, RBS>,
    > core::cmp::PartialEq<Mvect<RBS, M, F>> for Mvect<BS, M, F>
{
    fn eq(&self, other: &Mvect<RBS, M, F>) -> bool {
        MvPartialEq::do_collect::<BS, RBS>(true, &self.0, &other.0)
    }
}

// Add - add two multivectors
struct MvAdd;
impl<F: Field> Collector<F, &mut [F]> for MvAdd {
    fn collect_both<'a>(out: &'a mut [F], left: &F, right: &F) -> &'a mut [F] {
        out[0] += left.clone();
        out[0] += right.clone();
        &mut out[1..]
    }
    fn collect_just_left<'a>(out: &'a mut [F], left: &F) -> &'a mut [F] {
        out[0] += left.clone();
        &mut out[1..]
    }
    fn collect_just_right<'a>(out: &'a mut [F], right: &F) -> &'a mut [F] {
        out[0] += right.clone();
        &mut out[1..]
    }
}
impl<
        LBS: BasisSet<M>
            + Len<Output: ArrayLength>
            + UnionMerge<RBS, Output: BasisSet<M> + Len<Output: ArrayLength>>,
        RBS: BasisSet<M> + Len<Output: ArrayLength>,
        M: Metric,
        F: Field + for<'a> CollectInto<F, MvAdd, &'a mut [F], LBS, RBS>,
    > Add<&Mvect<RBS, M, F>> for &Mvect<LBS, M, F>
{
    type Output = Mvect<Union<LBS, RBS>, M, F>;
    fn add(self, rhs: &Mvect<RBS, M, F>) -> Self::Output {
        let mut out = Self::Output::default();
        MvAdd::do_collect::<LBS, RBS>(&mut out.0, &self.0, &rhs.0);
        out
    }
}
impl<
        LBS: BasisSet<M>
            + Len<Output: ArrayLength>
            + UnionMerge<RBS, Output: BasisSet<M> + Len<Output: ArrayLength>>,
        RBS: BasisSet<M> + Len<Output: ArrayLength>,
        M: Metric,
        F: Field + for<'a> CollectInto<F, MvAdd, &'a mut [F], LBS, RBS>,
    > Add<Mvect<RBS, M, F>> for Mvect<LBS, M, F>
{
    type Output = Mvect<Union<LBS, RBS>, M, F>;
    fn add(self, rhs: Mvect<RBS, M, F>) -> Self::Output {
        &self + &rhs
    }
}

// Sub - subtract two multivectors
struct MvSub;
impl<F: Field> Collector<F, &mut [F]> for MvSub {
    fn collect_both<'a>(out: &'a mut [F], left: &F, right: &F) -> &'a mut [F] {
        out[0] -= left.clone();
        out[0] -= right.clone();
        &mut out[1..]
    }
    fn collect_just_left<'a>(out: &'a mut [F], left: &F) -> &'a mut [F] {
        out[0] -= left.clone();
        &mut out[1..]
    }
    fn collect_just_right<'a>(out: &'a mut [F], right: &F) -> &'a mut [F] {
        out[0] -= right.clone();
        &mut out[1..]
    }
}
impl<
        LBS: BasisSet<M>
            + Len<Output: ArrayLength>
            + UnionMerge<RBS, Output: BasisSet<M> + Len<Output: ArrayLength>>,
        RBS: BasisSet<M> + Len<Output: ArrayLength>,
        M: Metric,
        F: Field + for<'a> CollectInto<F, MvSub, &'a mut [F], LBS, RBS>,
    > Sub<&Mvect<RBS, M, F>> for &Mvect<LBS, M, F>
{
    type Output = Mvect<Union<LBS, RBS>, M, F>;
    fn sub(self, rhs: &Mvect<RBS, M, F>) -> Self::Output {
        let mut out = Self::Output::default();
        MvSub::do_collect::<LBS, RBS>(&mut out.0, &self.0, &rhs.0);
        out
    }
}
impl<
        LBS: BasisSet<M>
            + Len<Output: ArrayLength>
            + UnionMerge<RBS, Output: BasisSet<M> + Len<Output: ArrayLength>>,
        RBS: BasisSet<M> + Len<Output: ArrayLength>,
        M: Metric,
        F: Field + for<'a> CollectInto<F, MvSub, &'a mut [F], LBS, RBS>,
    > Sub<Mvect<RBS, M, F>> for Mvect<LBS, M, F>
{
    type Output = Mvect<Union<LBS, RBS>, M, F>;
    fn sub(self, rhs: Mvect<RBS, M, F>) -> Self::Output {
        &self - &rhs
    }
}

// // --------------------------------------------
// // multivector multiplication
struct MvMul;
impl<F: Field> CartXorCollector<F, &mut [F]> for MvMul {
    fn collect_both<'a>(out: &'a mut F, parity: bool, left: &F, right: &F) {
        if parity {
            *out -= left.clone() * right.clone();
        } else {
            *out += left.clone() * right.clone();
        }
    }
}

// and finally...the Mul impl itself 🎉
impl<
        A: BasisSet<M> + Len<Output: ArrayLength>,
        B: BasisSet<M> + Len<Output: ArrayLength>,
        M: Metric,
        F: Field + for<'a> CartXorCollectInto<F, MvMul, &'a mut [F], A, B>,
    > Mul<&Mvect<B, M, F>> for &Mvect<A, M, F>
where
    Mvect<A, M, F>: SymMul<
            Mvect<B, M, F>,
            Output: UnionMerge<<Mvect<A, M, F> as ASymMul<Mvect<B, M, F>>>::Output>,
        > + ASymMul<Mvect<B, M, F>>,
    Union<
        <Mvect<A, M, F> as SymMul<Mvect<B, M, F>>>::Output,
        <Mvect<A, M, F> as ASymMul<Mvect<B, M, F>>>::Output,
    >: BasisSet<M> + Len<Output: ArrayLength>,
{
    type Output = Mvect<
        Union<
            <Mvect<A, M, F> as SymMul<Mvect<B, M, F>>>::Output,
            <Mvect<A, M, F> as ASymMul<Mvect<B, M, F>>>::Output,
        >,
        M,
        F,
    >;
    fn mul(self, rhs: &Mvect<B, M, F>) -> Self::Output {
        // TODO: need to handle negative metrics and zeros
        let mut out = Self::Output::default();
        MvMul::do_collect::<A, B>(&mut out.0, &self.0, &rhs.0);
        out
    }
}

pub trait SymMul<Rhs> {
    type Output;
}
// EMPTY LHS
impl<RBS: BasisSet<M> + Len<Output: ArrayLength>, M: Metric, F: Field> SymMul<Mvect<RBS, M, F>>
    for Mvect<ATerm, M, F>
{
    type Output = ATerm;
}
// SINGLE ELEMENT LHS
impl<
        L: Unsigned
            + CountOf<B1>
            + Add<B1>
            + BitAnd<And<R, M::ZeroMask>>
            + BitXor<R, Output: CountOf<B1>>
            + SymMul<B, Output: UnionMerge<tarr![tarr![Xor<L, R>, L, R]]>>,
        R: Unsigned + CountOf<B1> + Add<B1> + BitAnd<M::ZeroMask>,
        B: Len<Output: ArrayLength + Add<B1, Output: ArrayLength>>,
        M: Metric,
        F: Field,
    > SymMul<Mvect<TArr<R, B>, M, F>> for Mvect<tarr![L], M, F>
where
    TArr<R, B>: BasisSet<M>,
    tarr![L]: BasisSet<M>,
    U0: IsNotEqual<And<L, And<R, M::ZeroMask>>>,
    Count<Xor<L, R>, B1>: At<U1>,
    NotEq<U0, And<L, And<R, M::ZeroMask>>>: BitOr<Get<Count<Xor<L, R>, B1>, U1>>,
    Or<NotEq<U0, And<L, And<R, M::ZeroMask>>>, Get<Count<Xor<L, R>, B1>, U1>>: Branch<
        <L as SymMul<B>>::Output,
        Union<<L as SymMul<B>>::Output, tarr![tarr![Xor<L, R>, L, R]]>,
    >,
{
    // if !zero and isReverse, then include
    type Output = If<
        Or<NotEq<U0, And<L, And<R, M::ZeroMask>>>, Get<Count<Xor<L, R>, B1>, U1>>,
        <L as SymMul<B>>::Output,
        Union<<L as SymMul<B>>::Output, tarr![tarr![Xor<L, R>, L, R]]>,
    >;
}
// MULTI ELEMENT LHS
impl<
        L0: Unsigned,
        L1: Unsigned,
        A: BasisSet<M> + Len<Output: ArrayLength + Add<B1, Output: Add<B1, Output: ArrayLength>>>,
        B: BasisSet<M> + Len<Output: ArrayLength + Add<B1>>,
        M: Metric,
        F: Field,
    > SymMul<Mvect<B, M, F>> for Mvect<TArr<L0, TArr<L1, A>>, M, F>
where
    TArr<L0, TArr<L1, A>>: BasisSet<M> + Len<Output: ArrayLength + Add<B1>>,
    TArr<L1, A>: BasisSet<M> + Len<Output: ArrayLength + Add<B1>>,
    tarr![L0]: BasisSet<M> + Len<Output: ArrayLength + Add<B1>>,
    Mvect<tarr![L0], M, F>:
        SymMul<B, Output: UnionMerge<<Mvect<TArr<L1, A>, M, F> as SymMul<B>>::Output>>,
    Mvect<TArr<L1, A>, M, F>: SymMul<B>,
{
    type Output = Union<
        <Mvect<tarr![L0], M, F> as SymMul<B>>::Output,
        <Mvect<TArr<L1, A>, M, F> as SymMul<B>>::Output,
    >;
}

// --------------------------------------------
// asym multivector multiplication
pub trait ASymMul<Rhs> {
    type Output;
}
// EMPTY LHS
impl<RBS: BasisSet<M> + Len<Output: ArrayLength>, M: Metric, F: Field> ASymMul<Mvect<RBS, M, F>>
    for Mvect<ATerm, M, F>
{
    type Output = ATerm;
}
// SINGLE ELEMENT LHS
impl<
        L: Unsigned
            + CountOf<B1>
            + Add<B1>
            + BitAnd<And<R, M::ZeroMask>>
            + BitXor<R, Output: CountOf<B1>>
            + ASymMul<B, Output: UnionMerge<tarr![tarr![Xor<L, R>, L, R]]>>,
        R: Unsigned + CountOf<B1> + Add<B1> + BitAnd<M::ZeroMask>,
        B: Len<Output: ArrayLength + Add<B1, Output: ArrayLength>>,
        M: Metric,
        F: Field,
    > ASymMul<Mvect<TArr<R, B>, M, F>> for Mvect<tarr![L], M, F>
where
    TArr<R, B>: BasisSet<M>,
    tarr![L]: BasisSet<M>,
    U0: IsEqual<And<L, And<R, M::ZeroMask>>>,
    Count<Xor<L, R>, B1>: At<U1>,
    Eq<U0, And<L, And<R, M::ZeroMask>>>: BitOr<Get<Count<Xor<L, R>, B1>, U1>>,
    Or<Eq<U0, And<L, And<R, M::ZeroMask>>>, Get<Count<Xor<L, R>, B1>, U1>>: Branch<
        Union<<L as ASymMul<B>>::Output, tarr![tarr![Xor<L, R>, L, R]]>,
        <L as ASymMul<B>>::Output,
    >,
{
    // if !zero and !isReverse, then include
    type Output = If<
        Or<Eq<U0, And<L, And<R, M::ZeroMask>>>, Get<Count<Xor<L, R>, B1>, U1>>,
        Union<<L as ASymMul<B>>::Output, tarr![tarr![Xor<L, R>, L, R]]>,
        <L as ASymMul<B>>::Output,
    >;
}
// MULTI ELEMENT LHS
impl<
        L0: Unsigned,
        L1: Unsigned,
        A: BasisSet<M> + Len<Output: ArrayLength + Add<B1, Output: Add<B1, Output: ArrayLength>>>,
        B: BasisSet<M> + Len<Output: ArrayLength + Add<B1>>,
        M: Metric,
        F: Field,
    > ASymMul<Mvect<B, M, F>> for Mvect<TArr<L0, TArr<L1, A>>, M, F>
where
    TArr<L0, TArr<L1, A>>: BasisSet<M> + Len<Output: ArrayLength + Add<B1>>,
    TArr<L1, A>: BasisSet<M> + Len<Output: ArrayLength + Add<B1>>,
    tarr![L0]: BasisSet<M> + Len<Output: ArrayLength + Add<B1>>,
    Mvect<tarr![L0], M, F>:
        ASymMul<B, Output: UnionMerge<<Mvect<TArr<L1, A>, M, F> as ASymMul<B>>::Output>>,
    Mvect<TArr<L1, A>, M, F>: ASymMul<B>,
{
    type Output = Union<
        <Mvect<tarr![L0], M, F> as ASymMul<B>>::Output,
        <Mvect<TArr<L1, A>, M, F> as ASymMul<B>>::Output,
    >;
}

// tests
#[cfg(test)]
mod tests {
    use super::*;
    use typenum::{tarr, B0, B1, P1, U0, U1, U2, U3, U4, U5, U6, U7, Z0};

    type Metric = tarr![Z0, P1, P1];
    type Pga2d<U> = Basis<U, Metric, B0>;

    type Scalar = Pga2d<U0>;
    type E0 = Pga2d<U1>;
    type E1 = Pga2d<U2>;
    type E01 = Pga2d<U3>;
    type E2 = Pga2d<U4>;
    type E02 = Pga2d<U5>;
    type E12 = Pga2d<U6>;
    type E012 = Pga2d<U7>;

    #[test]
    fn test_default() {
        type M = tarr![Z0, P1, P1];
        type BS = <tarr![U0, U1, U2, U4] as BasisSet<M>>::Output;
        let mv = Mvect::<BS, M, f32>::default();
        assert_eq!(mv.len(), 4);
        for &elem in mv.0.iter() {
            assert_eq!(elem, 0.0);
        }
    }

    #[test]
    fn test_into_mv() {
        let expected = Mvect::<tarr![U3], Metric, f32>::new(GenericArray::<f32, U1>::from([1.0]));
        let actual = <E01 as IntoMv<f32>>::into_mv();
        assert!(expected == actual);
    }

    #[test]
    fn test_eq() {
        let mv0 = <E01 as IntoMv<f32>>::into_mv();
        let mv1 = <E01 as IntoMv<f32>>::into_mv();
        assert!(mv0 == mv1);
        // TODO: test zeros vs not stored vals
    }

    #[test]
    fn test_add() {
        let e = <Scalar as IntoMv<f32>>::into_mv();
        let e0 = <E0 as IntoMv<f32>>::into_mv();

        let c = e + e0;
    }

    #[test]
    fn test_mul() {
        let e = <Scalar as IntoMv<f32>>::into_mv();
        let e0 = <E0 as IntoMv<f32>>::into_mv();

        // let c = &e * &e0;
    }
}
