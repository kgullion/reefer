/// --------------------------------------------
/// set operations for *sorted* TypeArrays
use typenum::{ATerm, Bit, Cmp, Compare, Equal, Greater, Less, TArr, TypeArray, B0, B1};

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
#[allow(unused)]
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
#[allow(unused)]
pub type IsSuperset<A, B> = IsSubset<B, A>;

#[cfg(test)]
mod tests {
    use super::*;
    use typenum::{assert_type_eq, tarr, U0, U1, U2, U3};
    #[test]
    fn test_union() {
        assert_type_eq!(Union::<tarr![], tarr![]>, tarr![]); // {} | {} = {}
        assert_type_eq!(Union::<tarr![], tarr![U0]>, tarr![U0]); // {} | {0} = {0}
        assert_type_eq!(Union::<tarr![U0], tarr![]>, tarr![U0]); // {0} | {} = {0}
        assert_type_eq!(Union::<tarr![U0], tarr![U0]>, tarr![U0]); // {0} | {0} = {0}
        assert_type_eq!(Union::<tarr![U0], tarr![U1]>, tarr![U0, U1]); // {0} | {1} = {0, 1}
        assert_type_eq!(Union::<tarr![U1], tarr![U0]>, tarr![U0, U1]); // {1} | {0} = {0, 1}
        assert_type_eq!(Union::<tarr![U0, U2], tarr![U1, U2]>, tarr![U0, U1, U2]);
        assert_type_eq!(Union::<tarr![U0, U3], tarr![U1, U2]>, tarr![U0, U1, U2, U3]);
    }
    #[test]
    fn test_intersect() {
        assert_type_eq!(Intersect::<tarr![], tarr![]>, tarr![]); // {} & {} = {}
        assert_type_eq!(Intersect::<tarr![], tarr![U0]>, tarr![]); // {} & {0} = {}
        assert_type_eq!(Intersect::<tarr![U0], tarr![]>, tarr![]); // {0} & {} = {}
        assert_type_eq!(Intersect::<tarr![U0], tarr![U0]>, tarr![U0]); // {0} & {0} = {0}
        assert_type_eq!(Intersect::<tarr![U0], tarr![U1]>, tarr![]); // {0} & {1} = {}
        assert_type_eq!(Intersect::<tarr![U0, U2], tarr![U1, U2]>, tarr![U2]);
        assert_type_eq!(Intersect::<tarr![U0, U3], tarr![U1, U2]>, tarr![]);
    }
    #[test]
    fn test_diff() {
        assert_type_eq!(Diff::<tarr![], tarr![]>, tarr![]); // {} - {} = {}
        assert_type_eq!(Diff::<tarr![], tarr![U0]>, tarr![]); // {} - {0} = {}
        assert_type_eq!(Diff::<tarr![U0], tarr![]>, tarr![U0]); // {0} - {} = {0}
        assert_type_eq!(Diff::<tarr![U0], tarr![U0]>, tarr![]); // {0} - {0} = {}
        assert_type_eq!(Diff::<tarr![U0], tarr![U1]>, tarr![U0]); // {0} - {1} = {0}
        assert_type_eq!(Diff::<tarr![U0, U2], tarr![U1, U2]>, tarr![U0]);
        assert_type_eq!(Diff::<tarr![U0, U3], tarr![U1, U2]>, tarr![U0, U3]);
    }
    #[test]
    fn test_sym_diff() {
        assert_type_eq!(SymDiff::<tarr![], tarr![]>, tarr![]); // {} ^ {} = {}
        assert_type_eq!(SymDiff::<tarr![], tarr![U0]>, tarr![U0]); // {} ^ {0} = {0}
        assert_type_eq!(SymDiff::<tarr![U0], tarr![]>, tarr![U0]); // {0} ^ {} = {0}
        assert_type_eq!(SymDiff::<tarr![U0], tarr![U0]>, tarr![]); // {0} ^ {0} = {}
        assert_type_eq!(SymDiff::<tarr![U0], tarr![U1]>, tarr![U0, U1]); // {0} ^ {1} = {0, 1}
        assert_type_eq!(SymDiff::<tarr![U0, U2], tarr![U1, U2]>, tarr![U0, U1]);
        assert_type_eq!(
            SymDiff::<tarr![U0, U3], tarr![U1, U2]>,
            tarr![U0, U1, U2, U3]
        );
    }
    #[test]
    fn test_is_subset() {
        assert_eq!(IsSubset::<tarr![], tarr![]>::BOOL, true); // {} <= {} = true
        assert_eq!(IsSubset::<tarr![], tarr![U0]>::BOOL, true); // {} <= {0} = true
        assert_eq!(IsSubset::<tarr![U0], tarr![]>::BOOL, false); // {0} <= {} = false
        assert_eq!(IsSubset::<tarr![U0], tarr![U0]>::BOOL, true); // {0} <= {0} = true
        assert_eq!(IsSubset::<tarr![U0], tarr![U1]>::BOOL, false); // {0} <= {1} = false
        assert_eq!(IsSubset::<tarr![U0, U2], tarr![U1, U2]>::BOOL, false);
        assert_eq!(IsSubset::<tarr![U0, U3], tarr![U1, U2]>::BOOL, false);
        assert_eq!(IsSubset::<tarr![U0, U2], tarr![U0, U1, U2]>::BOOL, true);
        assert_eq!(IsSubset::<tarr![U0, U1, U2], tarr![U1, U2]>::BOOL, false);
        assert_eq!(IsSubset::<tarr![U0, U1, U3], tarr![U1, U2]>::BOOL, false);
    }
}
