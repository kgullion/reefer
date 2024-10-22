#![allow(unused)]
use super::{Branch, Flat, Flatten, If};
use core::ops::{Add, Shr, Sub};
use typenum::{
    tarr, ATerm, Add1, Bit, Cmp, Compare, Eq, Equal, Greater, IsEqual, Len, Length, Less, NonZero,
    Shright, Sub1, TArr, TypeArray, UInt, Unsigned, B0, B1, U0, U1, U2, U3, U4, U5, U6, U7, U8, U9,
};

// --------------------------------------------
// set operations for sorted TypeArrays

/// Intersection of two sorted TypeArrays
pub type Intersect<A, B> = <A as IntersectMerge<B>>::Output;
pub trait IntersectMerge<Rhs> {
    type Output;
}
impl<R> IntersectMerge<R> for ATerm {
    // {} * X = {}
    type Output = ATerm;
}
impl<L, A> IntersectMerge<ATerm> for TArr<L, A> {
    // X * {} = {}
    type Output = ATerm;
}
impl<L: Cmp<R>, A, R, B> IntersectMerge<TArr<R, B>> for TArr<L, A>
where
    Compare<L, R>: IntersectionConverge<TArr<L, A>, TArr<R, B>>,
{
    type Output = <Compare<L, R> as IntersectionConverge<TArr<L, A>, TArr<R, B>>>::Output;
}
pub trait IntersectionConverge<LA: TypeArray, RA: TypeArray> {
    type Output;
}
impl<L, A: IntersectMerge<B>, R, B> IntersectionConverge<TArr<L, A>, TArr<R, B>> for Equal {
    // L == R -> [L|fAB]  // Keep L
    type Output = TArr<L, Intersect<A, B>>;
}
impl<L, A: IntersectMerge<Rhs>, Rhs: TypeArray> IntersectionConverge<TArr<L, A>, Rhs> for Less {
    // L < R -> fA[RB]  // Drop L
    type Output = Intersect<A, Rhs>;
}
impl<Lhs: TypeArray + IntersectMerge<B>, R, B> IntersectionConverge<Lhs, TArr<R, B>> for Greater {
    // L > R -> f[LA]B  // Drop R
    type Output = Intersect<Lhs, B>;
}

/// Union of two sorted TypeArrays
pub type Union<A, B> = <A as UnionMerge<B>>::Output;
pub trait UnionMerge<Rhs> {
    type Output: TypeArray;
}
impl<R: TypeArray> UnionMerge<R> for ATerm {
    // {} + X = X
    type Output = R;
}
impl<L, A> UnionMerge<ATerm> for TArr<L, A> {
    // X + {} = X
    type Output = TArr<L, A>;
}
impl<L: Cmp<R>, A, R, B> UnionMerge<TArr<R, B>> for TArr<L, A>
where
    Compare<L, R>: UnionizeConverge<TArr<L, A>, TArr<R, B>>,
    <Compare<L, R> as UnionizeConverge<TArr<L, A>, TArr<R, B>>>::Output: TypeArray,
{
    type Output = <Compare<L, R> as UnionizeConverge<TArr<L, A>, TArr<R, B>>>::Output;
}
pub trait UnionizeConverge<LA: TypeArray, RA: TypeArray> {
    type Output;
}
impl<L, A: UnionMerge<B>, R, B> UnionizeConverge<TArr<L, A>, TArr<R, B>> for Equal {
    // L == R -> [L|fAB] // Keep L
    type Output = TArr<L, Union<A, B>>;
}
impl<L, A: UnionMerge<Rhs>, Rhs: TypeArray> UnionizeConverge<TArr<L, A>, Rhs> for Less {
    // L < R -> [L|fA[RB]] // Keep L
    type Output = TArr<L, Union<A, Rhs>>;
}
impl<Lhs: TypeArray + UnionMerge<B>, R, B> UnionizeConverge<Lhs, TArr<R, B>> for Greater {
    // L > R -> [R|f[LA]B] // Keep R
    type Output = TArr<R, Union<Lhs, B>>;
}

/// Difference of two sorted TypeArrays
pub type Diff<A, B> = <A as DiffMerge<B>>::Output;
pub trait DiffMerge<Rhs> {
    type Output;
}
impl<Rhs> DiffMerge<Rhs> for ATerm {
    // {} - X = {}
    type Output = ATerm;
}
impl<L, A> DiffMerge<ATerm> for TArr<L, A> {
    // X - {} = X
    type Output = TArr<L, A>;
}
impl<L: Cmp<R>, A, R, B> DiffMerge<TArr<R, B>> for TArr<L, A>
where
    Compare<L, R>: DifferenceConverge<TArr<L, A>, TArr<R, B>>,
{
    type Output = <Compare<L, R> as DifferenceConverge<TArr<L, A>, TArr<R, B>>>::Output;
}
pub trait DifferenceConverge<Lhs: TypeArray, Rhs: TypeArray> {
    type Output;
}
impl<L, A: DiffMerge<B>, R, B> DifferenceConverge<TArr<L, A>, TArr<R, B>> for Equal {
    // L == R -> fAB  // Drop L
    type Output = Diff<A, B>;
}
impl<L, A: DiffMerge<Rhs>, Rhs: TypeArray> DifferenceConverge<TArr<L, A>, Rhs> for Less {
    // L < R -> fA[RB]  // Keep L
    type Output = TArr<L, Diff<A, Rhs>>;
}
impl<Lhs: TypeArray + DiffMerge<B>, R, B> DifferenceConverge<Lhs, TArr<R, B>> for Greater {
    // L > R -> f[LA]B  // Drop R
    type Output = Diff<Lhs, B>;
}

/// Symmetric Difference of two sorted TypeArrays
pub type SymDiff<A, B> = Union<Diff<A, B>, Diff<B, A>>;

/// Is Disjoint
pub type IsDisjoint<A, B> = <A as DisjointMerge<B>>::Output;
pub trait DisjointMerge<Rhs> {
    type Output: Bit;
}
impl<Rhs> DisjointMerge<Rhs> for ATerm {
    // {} is disjoint with everything
    type Output = B1;
}
impl<L, A> DisjointMerge<ATerm> for TArr<L, A> {
    // everything is disjoint with {}
    type Output = B1;
}
impl<L: Cmp<R>, A, R, B> DisjointMerge<TArr<R, B>> for TArr<L, A>
where
    Compare<L, R>: DisjointConverge<TArr<L, A>, TArr<R, B>>,
{
    type Output = <Compare<L, R> as DisjointConverge<TArr<L, A>, TArr<R, B>>>::Output;
}
pub trait DisjointConverge<Lhs: TypeArray, Rhs: TypeArray> {
    type Output: Bit;
}
impl<L, A: DisjointMerge<B>, R, B> DisjointConverge<TArr<L, A>, TArr<R, B>> for Equal {
    // L == R -> fAB  // Not disjoint
    type Output = B0;
}
impl<L, A: DisjointMerge<Rhs>, Rhs: TypeArray> DisjointConverge<TArr<L, A>, Rhs> for Less {
    // L < R -> fA[RB]  // Check A and Rhs
    type Output = IsDisjoint<A, Rhs>;
}
impl<Lhs: TypeArray + DisjointMerge<B>, R, B> DisjointConverge<Lhs, TArr<R, B>> for Greater {
    // L > R -> f[LA]B  // Check Lhs and B
    type Output = IsDisjoint<Lhs, B>;
}

/// A <= B
pub type IsSubset<A, B> = <A as SubsetMerge<B>>::Output;
pub trait SubsetMerge<Rhs> {
    type Output: Bit;
}
impl<Rhs> SubsetMerge<Rhs> for ATerm {
    // {} is a subset of everything
    type Output = B1;
}
impl<L, A> SubsetMerge<ATerm> for TArr<L, A> {
    // nothing is a subset of {}
    type Output = B0;
}
impl<L: Cmp<R>, A, R, B> SubsetMerge<TArr<R, B>> for TArr<L, A>
where
    Compare<L, R>: SubsetConverge<TArr<L, A>, TArr<R, B>>,
{
    type Output = <Compare<L, R> as SubsetConverge<TArr<L, A>, TArr<R, B>>>::Output;
}
pub trait SubsetConverge<Lhs: TypeArray, Rhs: TypeArray> {
    type Output: Bit;
}
impl<L, A: SubsetMerge<B>, R, B> SubsetConverge<TArr<L, A>, TArr<R, B>> for Equal {
    // L == R -> fAB  // Check A and B
    type Output = IsSubset<A, B>;
}
impl<L, A: SubsetMerge<Rhs>, Rhs: TypeArray> SubsetConverge<TArr<L, A>, Rhs> for Less {
    // L < R -> False  // not a subset
    type Output = B0;
}
impl<Lhs: TypeArray + SubsetMerge<B>, R, B> SubsetConverge<Lhs, TArr<R, B>> for Greater {
    // L > R -> f[LA]B  // Check Lhs and B
    type Output = IsSubset<Lhs, B>;
}

// IsSuperset<A, B> = IsSubset<B, A>;
pub type IsSuperset<A, B> = IsSubset<B, A>;

// /// Cartesian Product of two sorted TypeArrays
// /// A × B = {a*b | a in A, b in B}
// pub type CartProd<A, B> = <A as CartProdMerge<B>>::Output;
// pub trait CartProdMerge<Rhs: TypeArray> {
//     type Output: TypeArray;
// }
// impl<Rhs: TypeArray> CartProdMerge<Rhs> for ATerm {
//     // {} × X = {}
//     type Output = ATerm;
// }
// impl<L: CartProdConverge<Rhs>, A: CartProdMerge<Rhs>, Rhs: TypeArray> CartProdMerge<Rhs>
//     for TArr<L, A>
// {
//     // L × R = {l, r | l in A, r in A}
//     // TODO: fix this
//     type Output = Flat<TArr<<L as CartProdConverge<Rhs>>::Output, CartProd<A, Rhs>>>;
// }

// pub type Pair<A, B> = tarr![A, B];
// pub trait CartProdConverge<Rhs: TypeArray> {
//     type Output: TypeArray;
// }
// impl<L: CartProdConverge<B>, R, B: TypeArray> CartProdConverge<TArr<R, B>> for L {
//     // l*r for r in B
//     type Output = TArr<Pair<L, R>, <L as CartProdConverge<B>>::Output>;
// }
// impl<L> CartProdConverge<ATerm> for L {
//     type Output = ATerm;
// }

// --------------------------------------------
// split TypeArray into two TypeArrays
type SplitLeft<A, I> = <I as Split<A>>::Left;
type SplitRight<A, I> = <I as Split<A>>::Right;
pub trait Split<A: TypeArray> {
    type Left: TypeArray;
    type Right: TypeArray;
}
impl<I: Unsigned> Split<ATerm> for I {
    type Left = ATerm;
    type Right = ATerm;
}
impl<V, A: TypeArray, I: Unsigned + Split<A> + Sub<B1> + IsEqual<U0>> Split<TArr<V, A>> for I
where
    Sub1<I>: Split<A>,
    Eq<I, U0>: Branch<ATerm, TArr<V, SplitLeft<A, Sub1<I>>>>,
    If<Eq<I, U0>, ATerm, TArr<V, SplitLeft<A, Sub1<I>>>>: TypeArray,
    Eq<I, U0>: Branch<TArr<V, A>, SplitRight<A, Sub1<I>>>,
    If<Eq<I, U0>, TArr<V, A>, SplitRight<A, Sub1<I>>>: TypeArray,
{
    type Left = If<Eq<I, U0>, ATerm, TArr<V, SplitLeft<A, Sub1<I>>>>;
    type Right = If<Eq<I, U0>, TArr<V, A>, SplitRight<A, Sub1<I>>>;
}

// --------------------------------------------
// sort TypeArray - merge sort
// TODO: does this actually end up better than something more naive? def not O(nlogn) since Len and Idx are O(n) not O(1)
pub type Sorted<A> = <A as Sort>::Sorted;
pub trait Sort {
    type Sorted: TypeArray;
}
impl Sort for ATerm {
    type Sorted = ATerm;
}
impl<V, A: Sort + Len + TypeArray> Sort for TArr<V, A>
where
    Length<A>: Add<B1>,
    Add1<Length<A>>: Unsigned + Shr<UInt<U0, B1>>,
    Shright<Add1<Length<A>>, U1>: Unsigned + Split<TArr<V, A>>,
    SplitLeft<TArr<V, A>, Shright<Add1<Length<A>>, U1>>: Sort + TypeArray,
    SplitRight<TArr<V, A>, Shright<Add1<Length<A>>, U1>>: Sort + TypeArray,
    Sorted<SplitLeft<TArr<V, A>, Shright<Add1<Length<A>>, U1>>>:
        UnionMerge<Sorted<SplitRight<TArr<V, A>, Shright<Add1<Length<A>>, U1>>>>,
{
    type Sorted = Union<
        Sorted<SplitLeft<TArr<V, A>, Shright<Add1<Length<A>>, U1>>>,
        Sorted<SplitRight<TArr<V, A>, Shright<Add1<Length<A>>, U1>>>,
    >;
}
