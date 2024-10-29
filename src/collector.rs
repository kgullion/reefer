use crate::{
    basis::{Basis, ZeroVect},
    field::Field,
    metric::Metric,
    ta,
    utils::contains::{Contains, IdxOf, IndexOf},
};
use typenum::{Bit, Cmp, Compare, Equal, Greater, Less, TypeArray, Unsigned, B0, B1};

/// Iterates the Left and Right sorted TypeArrays, and calls the appropriate Collector fn
/// based on if the current type exists in both arrays, just the left array, or just the right array.
pub trait CollectInto<T, CO: Collector<T, OUT>, OUT, A: TypeArray, B: TypeArray> {
    fn collect(out: OUT, left: &[T], right: &[T]) -> OUT;
}
/// The Collector trait is MUCH easier to implement than repeating the same logic in CollectInto for each type
pub trait Collector<T, OUT> {
    fn do_collect<A: TypeArray, B: TypeArray>(out: OUT, left: &[T], right: &[T]) -> OUT
    where
        Self: Collector<T, OUT> + Sized,
        T: CollectInto<T, Self, OUT, A, B>,
    {
        <T as CollectInto<T, Self, OUT, A, B>>::collect(out, left, right)
    }
    fn collect_both(out: OUT, left: &T, right: &T) -> OUT {
        let _ = left;
        let _ = right;
        out
    }
    fn collect_just_left(out: OUT, left: &T) -> OUT {
        let _ = left;
        out
    }
    fn collect_just_right(out: OUT, right: &T) -> OUT {
        let _ = right;
        out
    }
}
// we've reached the end of both arrays
impl<CO: Collector<F, OUT>, OUT, F: Field> CollectInto<F, CO, OUT, ta![], ta![]> for F {
    fn collect(out: OUT, _left: &[F], _right: &[F]) -> OUT {
        out
    }
}
// we've reached the end of the left array
impl<F: Field, CO: Collector<F, OUT>, OUT, R, B: TypeArray>
    CollectInto<F, CO, OUT, ta![], ta![R | B]> for F
where
    F: CollectInto<F, CO, OUT, ta![], B>,
{
    fn collect(out: OUT, left: &[F], right: &[F]) -> OUT {
        // iterate right array
        <F as CollectInto<F, CO, OUT, ta![], B>>::collect(
            // collect the right array at the current position
            CO::collect_just_right(out, &right[0]),
            left,
            &right[1..],
        )
    }
}
// we've reached the end of the right array
impl<F: Field, CO: Collector<F, OUT>, OUT, L, A: TypeArray>
    CollectInto<F, CO, OUT, ta![L | A], ta![]> for F
where
    F: CollectInto<F, CO, OUT, A, ta![]>,
{
    fn collect(out: OUT, left: &[F], right: &[F]) -> OUT {
        // iterate left array
        <F as CollectInto<F, CO, OUT, A, ta![]>>::collect(
            // collect the left array at the current position
            CO::collect_just_left(out, &left[0]),
            &left[1..],
            right,
        )
    }
}
// we're in the middle of both arrays, need to check L<R, L>R, L=R and call the appropriate collect fn
impl<F: Field, CO: Collector<F, OUT>, OUT, L, R, A: TypeArray, B: TypeArray>
    CollectInto<F, CO, OUT, ta![L | A], ta![R | B]> for F
where
    L: Cmp<R, Output: CollectInto<F, CO, OUT, ta![L | A], ta![R | B]>>,
{
    fn collect(out: OUT, left: &[F], right: &[F]) -> OUT {
        <Compare<L, R> as CollectInto<F, CO, OUT, ta![L | A], ta![R | B]>>::collect(
            out, left, right,
        )
    }
}
// L<R, collect the left array at the current position
impl<F: Field, CO: Collector<F, OUT>, OUT, L, R, A: TypeArray, B: TypeArray>
    CollectInto<F, CO, OUT, ta![L | A], ta![R | B]> for Less
where
    F: CollectInto<F, CO, OUT, A, ta![R | B]>,
{
    fn collect(out: OUT, left: &[F], right: &[F]) -> OUT {
        <F as CollectInto<F, CO, OUT, A, ta![R | B]>>::collect(
            CO::collect_just_left(out, &left[0]),
            &left[1..],
            right,
        )
    }
}
// L>R, collect the right array at the current position
impl<F: Field, CO: Collector<F, OUT>, OUT, L, R, A: TypeArray, B: TypeArray>
    CollectInto<F, CO, OUT, ta![L | A], ta![R | B]> for Greater
where
    F: CollectInto<F, CO, OUT, ta![R | B], A>,
{
    fn collect(out: OUT, left: &[F], right: &[F]) -> OUT {
        <F as CollectInto<F, CO, OUT, ta![R | B], A>>::collect(
            CO::collect_just_right(out, &right[0]),
            left,
            &right[1..],
        )
    }
}
// L=R, collect both arrays at the current position
impl<F: Field, CO: Collector<F, OUT>, OUT, L, R, A: TypeArray, B: TypeArray>
    CollectInto<F, CO, OUT, ta![L | A], ta![R | B]> for Equal
where
    F: CollectInto<F, CO, OUT, A, B>,
{
    fn collect(out: OUT, left: &[F], right: &[F]) -> OUT {
        <F as CollectInto<F, CO, OUT, A, B>>::collect(
            CO::collect_both(out, &left[0], &right[0]),
            &left[1..],
            &right[1..],
        )
    }
}

// ----------------------------------------------------------------------------
// CartCollector
// this one is specific to a &mut[F] output, as it needs to collect the
// cartesian product of the two arrays rather than just zip them together.
pub trait CartCollector<F: Field, OUT: TypeArray> {
    fn collect(out: &mut [F], left: &F, right: &F);
}
impl<F: Field, OUT: TypeArray, M: Metric> CartCollector<F, OUT> for ZeroVect<M> {
    fn collect(_out: &mut [F], _left: &F, _right: &F) {}
}
impl<U: Unsigned, M: Metric, F: Field, OUT: TypeArray + IndexOf<U>> CartCollector<F, OUT>
    for Basis<U, M, B0>
{
    fn collect(out: &mut [F], left: &F, right: &F) {
        // branch should get compiled away since this is a comptime const 🤞
        if Contains::<OUT, U>::BOOL {
            let idx = IdxOf::<OUT, U>::USIZE;
            out[idx] += left.clone() * right.clone();
        }
    }
}
impl<U: Unsigned, M: Metric, F: Field, OUT: TypeArray + IndexOf<U>> CartCollector<F, OUT>
    for Basis<U, M, B1>
{
    fn collect(out: &mut [F], left: &F, right: &F) {
        // branch should get compiled away since this is a comptime const 🤞
        if Contains::<OUT, U>::BOOL {
            let idx = IdxOf::<OUT, U>::USIZE;
            out[idx] -= left.clone() * right.clone();
        }
    }
}
