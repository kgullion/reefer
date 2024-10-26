use crate::ta;
/// --------------------------------------------
/// set operations for *sorted* TypeArrays
use typenum::{Bit, Cmp, Compare, Equal, Greater, Less, TypeArray, B0, B1};

/// Intersection of two sorted TypeArrays
pub type Intersect<A, B> = <A as IntersectMerge<B>>::Output;
pub trait IntersectMerge<Rhs> {
    type Output;
}
impl<R> IntersectMerge<R> for ta![] {
    // {} * X = {}
    type Output = ta![];
}
impl<L, A> IntersectMerge<ta![]> for ta![L | A] {
    // X * {} = {}
    type Output = ta![];
}
impl<L: Cmp<R>, A, R, B> IntersectMerge<ta![R | B]> for ta![L | A]
where
    Compare<L, R>: IntersectionConverge<ta![L | A], ta![R | B]>,
{
    type Output = <Compare<L, R> as IntersectionConverge<ta![L | A], ta![R | B]>>::Output;
}
pub trait IntersectionConverge<LA: TypeArray, RA: TypeArray> {
    type Output;
}
impl<L, A: IntersectMerge<B>, R, B> IntersectionConverge<ta![L | A], ta![R | B]> for Equal {
    // L == R -> [L|fAB]  // Keep L
    type Output = ta![L | Intersect<A, B>];
}
impl<L, A: IntersectMerge<Rhs>, Rhs: TypeArray> IntersectionConverge<ta![L | A], Rhs> for Less {
    // L < R -> fA[RB]  // Drop L
    type Output = Intersect<A, Rhs>;
}
impl<Lhs: TypeArray + IntersectMerge<B>, R, B> IntersectionConverge<Lhs, ta![R | B]> for Greater {
    // L > R -> f[LA]B  // Drop R
    type Output = Intersect<Lhs, B>;
}

/// Union of two sorted TypeArrays
pub type Union<A, B> = <A as UnionMerge<B>>::Output;
pub trait UnionMerge<Rhs> {
    type Output: TypeArray;
}
impl<R: TypeArray> UnionMerge<R> for ta![] {
    // {} + X = X
    type Output = R;
}
impl<L, A> UnionMerge<ta![]> for ta![L | A] {
    // X + {} = X
    type Output = ta![L | A];
}
impl<L: Cmp<R>, A, R, B> UnionMerge<ta![R | B]> for ta![L | A]
where
    Compare<L, R>: UnionizeConverge<ta![L | A], ta![R | B]>,
    <Compare<L, R> as UnionizeConverge<ta![L | A], ta![R | B]>>::Output: TypeArray,
{
    type Output = <Compare<L, R> as UnionizeConverge<ta![L | A], ta![R | B]>>::Output;
}
pub trait UnionizeConverge<LA: TypeArray, RA: TypeArray> {
    type Output;
}
impl<L, A: UnionMerge<B>, R, B> UnionizeConverge<ta![L | A], ta![R | B]> for Equal {
    // L == R -> [L|fAB] // Keep L
    type Output = ta![L | Union<A, B>];
}
impl<L, A: UnionMerge<Rhs>, Rhs: TypeArray> UnionizeConverge<ta![L | A], Rhs> for Less {
    // L < R -> [L|fA[RB]] // Keep L
    type Output = ta![L | Union<A, Rhs>];
}
impl<Lhs: TypeArray + UnionMerge<B>, R, B> UnionizeConverge<Lhs, ta![R | B]> for Greater {
    // L > R -> [R|f[LA]B] // Keep R
    type Output = ta![R | Union<Lhs, B>];
}

/// Difference of two sorted TypeArrays
pub type Diff<A, B> = <A as DiffMerge<B>>::Output;
pub trait DiffMerge<Rhs> {
    type Output;
}
impl<Rhs> DiffMerge<Rhs> for ta![] {
    // {} - X = {}
    type Output = ta![];
}
impl<L, A> DiffMerge<ta![]> for ta![L | A] {
    // X - {} = X
    type Output = ta![L | A];
}
impl<L: Cmp<R>, A, R, B> DiffMerge<ta![R | B]> for ta![L | A]
where
    Compare<L, R>: DifferenceConverge<ta![L | A], ta![R | B]>,
{
    type Output = <Compare<L, R> as DifferenceConverge<ta![L | A], ta![R | B]>>::Output;
}
pub trait DifferenceConverge<Lhs: TypeArray, Rhs: TypeArray> {
    type Output;
}
impl<L, A: DiffMerge<B>, R, B> DifferenceConverge<ta![L | A], ta![R | B]> for Equal {
    // L == R -> fAB  // Drop L
    type Output = Diff<A, B>;
}
impl<L, A: DiffMerge<Rhs>, Rhs: TypeArray> DifferenceConverge<ta![L | A], Rhs> for Less {
    // L < R -> fA[RB]  // Keep L
    type Output = ta![L | Diff<A, Rhs>];
}
impl<Lhs: TypeArray + DiffMerge<B>, R, B> DifferenceConverge<Lhs, ta![R | B]> for Greater {
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
impl<Rhs> DisjointMerge<Rhs> for ta![] {
    // {} is disjoint with everything
    type Output = B1;
}
impl<L, A> DisjointMerge<ta![]> for ta![L | A] {
    // everything is disjoint with {}
    type Output = B1;
}
impl<L: Cmp<R>, A, R, B> DisjointMerge<ta![R | B]> for ta![L | A]
where
    Compare<L, R>: DisjointConverge<ta![L | A], ta![R | B]>,
{
    type Output = <Compare<L, R> as DisjointConverge<ta![L | A], ta![R | B]>>::Output;
}
pub trait DisjointConverge<Lhs: TypeArray, Rhs: TypeArray> {
    type Output: Bit;
}
impl<L, A: DisjointMerge<B>, R, B> DisjointConverge<ta![L | A], ta![R | B]> for Equal {
    // L == R -> fAB  // Not disjoint
    type Output = B0;
}
impl<L, A: DisjointMerge<Rhs>, Rhs: TypeArray> DisjointConverge<ta![L | A], Rhs> for Less {
    // L < R -> fA[RB]  // Check A and Rhs
    type Output = IsDisjoint<A, Rhs>;
}
impl<Lhs: TypeArray + DisjointMerge<B>, R, B> DisjointConverge<Lhs, ta![R | B]> for Greater {
    // L > R -> f[LA]B  // Check Lhs and B
    type Output = IsDisjoint<Lhs, B>;
}

/// A <= B
pub type IsSubset<A, B> = <A as SubsetMerge<B>>::Output;
pub trait SubsetMerge<Rhs> {
    type Output: Bit;
}
impl<Rhs> SubsetMerge<Rhs> for ta![] {
    // {} is a subset of everything
    type Output = B1;
}
impl<L, A> SubsetMerge<ta![]> for ta![L | A] {
    // nothing is a subset of {}
    type Output = B0;
}
impl<L: Cmp<R>, A, R, B> SubsetMerge<ta![R | B]> for ta![L | A]
where
    Compare<L, R>: SubsetConverge<ta![L | A], ta![R | B]>,
{
    type Output = <Compare<L, R> as SubsetConverge<ta![L | A], ta![R | B]>>::Output;
}
pub trait SubsetConverge<Lhs: TypeArray, Rhs: TypeArray> {
    type Output: Bit;
}
impl<L, A: SubsetMerge<B>, R, B> SubsetConverge<ta![L | A], ta![R | B]> for Equal {
    // L == R -> fAB  // Check A and B
    type Output = IsSubset<A, B>;
}
impl<L, A: SubsetMerge<Rhs>, Rhs: TypeArray> SubsetConverge<ta![L | A], Rhs> for Less {
    // L < R -> False  // not a subset
    type Output = B0;
}
impl<Lhs: TypeArray + SubsetMerge<B>, R, B> SubsetConverge<Lhs, ta![R | B]> for Greater {
    // L > R -> f[LA]B  // Check Lhs and B
    type Output = IsSubset<Lhs, B>;
}

// IsSuperset<A, B> = IsSubset<B, A>;
#[allow(unused)]
pub type IsSuperset<A, B> = IsSubset<B, A>;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ta;
    use typenum::{assert_type_eq, U0, U1, U2, U3};
    #[test]
    fn test_union() {
        assert_type_eq!(Union::<ta![], ta![]>, ta![]); // {} | {} = {}
        assert_type_eq!(Union::<ta![], ta![U0]>, ta![U0]); // {} | {0} = {0}
        assert_type_eq!(Union::<ta![U0], ta![]>, ta![U0]); // {0} | {} = {0}
        assert_type_eq!(Union::<ta![U0], ta![U0]>, ta![U0]); // {0} | {0} = {0}
        assert_type_eq!(Union::<ta![U0], ta![U1]>, ta![U0, U1]); // {0} | {1} = {0, 1}
        assert_type_eq!(Union::<ta![U1], ta![U0]>, ta![U0, U1]); // {1} | {0} = {0, 1}
        assert_type_eq!(Union::<ta![U0, U2], ta![U1, U2]>, ta![U0, U1, U2]);
        assert_type_eq!(Union::<ta![U0, U3], ta![U1, U2]>, ta![U0, U1, U2, U3]);
    }
    #[test]
    fn test_intersect() {
        assert_type_eq!(Intersect::<ta![], ta![]>, ta![]); // {} & {} = {}
        assert_type_eq!(Intersect::<ta![], ta![U0]>, ta![]); // {} & {0} = {}
        assert_type_eq!(Intersect::<ta![U0], ta![]>, ta![]); // {0} & {} = {}
        assert_type_eq!(Intersect::<ta![U0], ta![U0]>, ta![U0]); // {0} & {0} = {0}
        assert_type_eq!(Intersect::<ta![U0], ta![U1]>, ta![]); // {0} & {1} = {}
        assert_type_eq!(Intersect::<ta![U0, U2], ta![U1, U2]>, ta![U2]);
        assert_type_eq!(Intersect::<ta![U0, U3], ta![U1, U2]>, ta![]);
    }
    #[test]
    fn test_diff() {
        assert_type_eq!(Diff::<ta![], ta![]>, ta![]); // {} - {} = {}
        assert_type_eq!(Diff::<ta![], ta![U0]>, ta![]); // {} - {0} = {}
        assert_type_eq!(Diff::<ta![U0], ta![]>, ta![U0]); // {0} - {} = {0}
        assert_type_eq!(Diff::<ta![U0], ta![U0]>, ta![]); // {0} - {0} = {}
        assert_type_eq!(Diff::<ta![U0], ta![U1]>, ta![U0]); // {0} - {1} = {0}
        assert_type_eq!(Diff::<ta![U0, U2], ta![U1, U2]>, ta![U0]);
        assert_type_eq!(Diff::<ta![U0, U3], ta![U1, U2]>, ta![U0, U3]);
    }
    #[test]
    fn test_sym_diff() {
        assert_type_eq!(SymDiff::<ta![], ta![]>, ta![]); // {} ^ {} = {}
        assert_type_eq!(SymDiff::<ta![], ta![U0]>, ta![U0]); // {} ^ {0} = {0}
        assert_type_eq!(SymDiff::<ta![U0], ta![]>, ta![U0]); // {0} ^ {} = {0}
        assert_type_eq!(SymDiff::<ta![U0], ta![U0]>, ta![]); // {0} ^ {0} = {}
        assert_type_eq!(SymDiff::<ta![U0], ta![U1]>, ta![U0, U1]); // {0} ^ {1} = {0, 1}
        assert_type_eq!(SymDiff::<ta![U0, U2], ta![U1, U2]>, ta![U0, U1]);
        assert_type_eq!(SymDiff::<ta![U0, U3], ta![U1, U2]>, ta![U0, U1, U2, U3]);
    }
    #[test]
    fn test_is_subset() {
        assert_eq!(IsSubset::<ta![], ta![]>::BOOL, true); // {} <= {} = true
        assert_eq!(IsSubset::<ta![], ta![U0]>::BOOL, true); // {} <= {0} = true
        assert_eq!(IsSubset::<ta![U0], ta![]>::BOOL, false); // {0} <= {} = false
        assert_eq!(IsSubset::<ta![U0], ta![U0]>::BOOL, true); // {0} <= {0} = true
        assert_eq!(IsSubset::<ta![U0], ta![U1]>::BOOL, false); // {0} <= {1} = false
        assert_eq!(IsSubset::<ta![U0, U2], ta![U1, U2]>::BOOL, false);
        assert_eq!(IsSubset::<ta![U0, U3], ta![U1, U2]>::BOOL, false);
        assert_eq!(IsSubset::<ta![U0, U2], ta![U0, U1, U2]>::BOOL, true);
        assert_eq!(IsSubset::<ta![U0, U1, U2], ta![U1, U2]>::BOOL, false);
        assert_eq!(IsSubset::<ta![U0, U1, U3], ta![U1, U2]>::BOOL, false);
    }
}
