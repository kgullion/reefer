use crate::{
    field::Field,
    utils::{At, Contains, CountOf, Get, IdxOf, IndexOf, SwapPar, SwapParity},
};
use core::ops::BitXor;
use typenum::{
    tarr, Bit, Cmp, Compare, Equal, Greater, Less, TArr, TypeArray, Unsigned, Xor, B0, B1, U0,
};

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
impl<CO: Collector<F, OUT>, OUT, F: Field> CollectInto<F, CO, OUT, tarr![], tarr![]> for F {
    fn collect(out: OUT, _left: &[F], _right: &[F]) -> OUT {
        out
    }
}
// we've reached the end of the left array
impl<F: Field, CO: Collector<F, OUT>, OUT, R, B: TypeArray>
    CollectInto<F, CO, OUT, tarr![], TArr<R, B>> for F
where
    F: CollectInto<F, CO, OUT, tarr![], B>,
{
    fn collect(out: OUT, left: &[F], right: &[F]) -> OUT {
        // iterate right array
        <F as CollectInto<F, CO, OUT, tarr![], B>>::collect(
            // collect the right array at the current position
            CO::collect_just_right(out, &right[0]),
            left,
            &right[1..],
        )
    }
}
// we've reached the end of the right array
impl<F: Field, CO: Collector<F, OUT>, OUT, L, A: TypeArray>
    CollectInto<F, CO, OUT, TArr<L, A>, tarr![]> for F
where
    F: CollectInto<F, CO, OUT, A, tarr![]>,
{
    fn collect(out: OUT, left: &[F], right: &[F]) -> OUT {
        // iterate left array
        <F as CollectInto<F, CO, OUT, A, tarr![]>>::collect(
            // collect the left array at the current position
            CO::collect_just_left(out, &left[0]),
            &left[1..],
            right,
        )
    }
}
// we're in the middle of both arrays, need to check L<R, L>R, L=R and call the appropriate collect fn
impl<F: Field, CO: Collector<F, OUT>, OUT, L, R, A: TypeArray, B: TypeArray>
    CollectInto<F, CO, OUT, TArr<L, A>, TArr<R, B>> for F
where
    L: Cmp<R, Output: CollectInto<F, CO, OUT, TArr<L, A>, TArr<R, B>>>,
{
    fn collect(out: OUT, left: &[F], right: &[F]) -> OUT {
        <Compare<L, R> as CollectInto<F, CO, OUT, TArr<L, A>, TArr<R, B>>>::collect(
            out, left, right,
        )
    }
}
// L<R, collect the left array at the current position
impl<F: Field, CO: Collector<F, OUT>, OUT, L, R, A: TypeArray, B: TypeArray>
    CollectInto<F, CO, OUT, TArr<L, A>, TArr<R, B>> for Less
where
    F: CollectInto<F, CO, OUT, A, TArr<R, B>>,
{
    fn collect(out: OUT, left: &[F], right: &[F]) -> OUT {
        <F as CollectInto<F, CO, OUT, A, TArr<R, B>>>::collect(
            CO::collect_just_left(out, &left[0]),
            &left[1..],
            right,
        )
    }
}
// L>R, collect the right array at the current position
impl<F: Field, CO: Collector<F, OUT>, OUT, L, R, A: TypeArray, B: TypeArray>
    CollectInto<F, CO, OUT, TArr<L, A>, TArr<R, B>> for Greater
where
    F: CollectInto<F, CO, OUT, TArr<R, B>, A>,
{
    fn collect(out: OUT, left: &[F], right: &[F]) -> OUT {
        <F as CollectInto<F, CO, OUT, TArr<R, B>, A>>::collect(
            CO::collect_just_right(out, &right[0]),
            left,
            &right[1..],
        )
    }
}
// L=R, collect both arrays at the current position
impl<F: Field, CO: Collector<F, OUT>, OUT, L, R, A: TypeArray, B: TypeArray>
    CollectInto<F, CO, OUT, TArr<L, A>, TArr<R, B>> for Equal
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

// // --------------------------------------------
// // multivector multiplication - this one is a bit more complex, but concept
// // of having Mul handle the Type-level setup and call the MvMul impl
// pub trait MvMul<C: TypeArray, L: TypeArray, R: TypeArray>
// where
//     Self: Field,
// {
//     fn mul(out: &mut [Self], lhs: &[Self], rhs: &[Self]);
// }
// pub trait MvMulInner<C: TypeArray, L: BasisInfo, B: TypeArray>
// where
//     Self: Field,
// {
//     fn mul_inner(out: &mut [Self], lhs: &Self, rhs: &[Self]);
// }
// // 0 * 0 = 0 - noop
// impl<F: Field, C: TypeArray> MvMul<C, tarr![], tarr![]> for F {
//     #[inline(always)]
//     fn mul(_out: &mut [F], _lhs: &[F], _rhs: &[F]) {
//         #[cfg(feature = "assert-invariants")]
//         {
//             assert_eq!(_out.len(), 0);
//             assert_eq!(_lhs.len(), 0);
//             assert_eq!(_rhs.len(), 0);
//         }
//     }
// }
// // 0 * B = 0 - noop
// impl<F: Field, C: TypeArray, R: Unsigned, B: TypeArray> MvMul<C, tarr![], TArr<R, B>> for F {
//     #[inline(always)]
//     fn mul(_out: &mut [F], _lhs: &[F], _rhs: &[F]) {
//         #[cfg(feature = "assert-invariants")]
//         {
//             assert_eq!(_lhs.len(), 0);
//             assert_eq!(_rhs.len(), Length::<B>::USIZE + 1);
//         }
//     }
// }
// // A * 0 = 0 - noop
// impl<F: Field, C: TypeArray, L: Unsigned, A: TypeArray> MvMul<C, TArr<L, A>, tarr![]> for F {
//     #[inline(always)]
//     fn mul(_out: &mut [F], _lhs: &[F], _rhs: &[F]) {
//         #[cfg(feature = "assert-invariants")]
//         {
//             assert_eq!(_lhs.len(), Length::<A>::USIZE + 1);
//             assert_eq!(_rhs.len(), 0);
//         }
//     }
// }
// // A * B = AB
// /// inner loop of product - multiple the single element of lhs by each element of rhs
// impl<
//         F: Field + MvMulInner<C, L, B>,
//         C: TypeArray + IndexOf<<BProd<L, R> as Abs>::Output>,
//         L: BasisInfo + BasisCart<R, Mul: Abs>,
//         R: BasisInfo,
//         B: TypeArray,
//     > MvMulInner<C, L, TArr<R, B>> for F
// {
//     #[inline(always)]
//     fn mul_inner(out: &mut [Self], lhs: &Self, rhs: &[Self]) {
//         if Contains::<C, <BProd<L, R> as Abs>::Output>::BOOL {
//             let i = IdxOf::<C, <BProd<L, R> as Abs>::Output>::USIZE;
//             if <BProd<L, R> as BasisInfo>::Sign::BOOL {
//                 out[i] -= lhs.clone() * rhs[0].clone()
//             } else {
//                 out[i] += lhs.clone() * rhs[0].clone()
//             }
//         }
//         <F as MvMulInner<C, L, B>>::mul_inner(out, lhs, &rhs[1..]);
//     }
// }
// // base case - no elements in rhs
// impl<F: Field, C: TypeArray, L: BasisInfo> MvMulInner<C, L, tarr![]> for F {
//     #[inline(always)]
//     fn mul_inner(_out: &mut [Self], _lhs: &Self, _rhs: &[Self]) {
//         #[cfg(feature = "assert-invariants")]
//         {
//             assert!(_rhs.len() == 0);
//         }
//     }
// }

// // outer loop of product - call the inner loop for each element of lhs
// impl<
//         F: Field + MvMulInner<C, L, TArr<R, B>>,
//         C: TypeArray,
//         L: BasisInfo,
//         A: TypeArray,
//         R: BasisInfo,
//         B: TypeArray,
//     > MvMul<C, TArr<L, A>, TArr<R, B>> for F
// where
//     for<'a> F: MvMul<C, A, B>,
//     for<'a> F: MvMul<C, TArr<L, A>, B>,
//     for<'a> F: MvMul<C, A, TArr<R, B>>,
// {
//     #[inline(always)]
//     fn mul(out: &mut [F], lhs: &[F], rhs: &[F]) {
//         #[cfg(feature = "assert-invariants")]
//         {
//             assert_eq!(lhs.len(), Length::<A>::USIZE + 1);
//             assert_eq!(rhs.len(), Length::<B>::USIZE + 1);
//         }
//         <F as MvMulInner<C, L, TArr<R, B>>>::mul_inner(out, &lhs[0], rhs);
//         <F as MvMul<C, A, TArr<R, B>>>::mul(out, &lhs[1..], rhs);
//     }
// }

// // and finally...the Mul impl itself 🎉
// impl<
//         A: TypeArray
//             + Len<Output: ArrayLength>
//             + MulBs<B, Output: PositiveBs<Output: TypeArray + Len<Output: ArrayLength>>>,
//         B: TypeArray + Len<Output: ArrayLength>,
//         F: Field + MvMul<<<A as MulBs<B>>::Output as PositiveBs>::Output, A, B>,
//     > Mul<&Mvect<B, F>> for &Mvect<A, F>
// where
//     Mvect<A, F>: MvectInfo,
//     Mvect<B, F>: MvectInfo,
// {
//     type Output = Mvect<<<A as MulBs<B>>::Output as PositiveBs>::Output, F>;
//     fn mul(self, rhs: &Mvect<B, F>) -> Self::Output {
//         let mut out = Self::Output::default();
//         <F as MvMul<<<A as MulBs<B>>::Output as PositiveBs>::Output, A, B>>::mul(
//             &mut out.0, &self.0, &rhs.0,
//         );
//         out
//     }
// }

// ----------------------------------------------------------------------------
// CartXorCollector
// this one is even more complex, and specific to a &mut[] output, as it needs to collect the
// cartesian product of the two arrays rather than just zip them together.
// it is a generalized version of MvMul
pub trait CartXorCollectInto<T, CO: CartXorCollector<T, OUT>, OUT, A: TypeArray, B: TypeArray> {
    fn collect<'a>(out: &'a mut [T], left: &'a [T], right: &'a [T]);
}
pub trait CartXorCollector<T, OUT> {
    fn do_collect<'a, A: TypeArray, B: TypeArray>(out: &'a mut [T], left: &'a [T], right: &'a [T])
    where
        T: CartXorCollectInto<T, Self, OUT, A, B>,
        Self: CartXorCollector<T, OUT> + Sized,
    {
        <T as CartXorCollectInto<T, Self, OUT, A, B>>::collect(out, left, right);
    }
    fn collect_both<'a>(out: &'a mut T, parity: bool, left: &T, right: &T);
}
// EMPTY LEFT ARRAY CASE
impl<F: Field, CO: CartXorCollector<F, OUT>, OUT, B: TypeArray>
    CartXorCollectInto<F, CO, OUT, tarr![], B> for F
{
    fn collect<'a>(_out: &'a mut [F], _left: &'a [F], _right: &'a [F]) {}
}
// SINGLE LEFT ARRAY CASES
// we only have one element on the left-side, time to do some work
impl<F: Field, CO: CartXorCollector<F, OUT>, OUT, L, R, B: TypeArray>
    CartXorCollectInto<F, CO, OUT, tarr![L], TArr<R, B>> for F
where
    F: CartXorCollector<F, OUT> + CartXorCollectInto<F, CO, OUT, tarr![L], B>,
    L: BitXor<R, Output: Unsigned>
        + CountOf<B1, Count: At<U0>>
        + SwapPar<R, Get<L::Count, U0>, B0, Parity: Bit>,
    OUT: IndexOf<Xor<L, R>, Found: Bit>,
{
    fn collect<'a>(out: &'a mut [F], left: &'a [F], right: &'a [F]) {
        // compiler should optimize this completely out if BOOL is false
        if Contains::<OUT, Xor<L, R>>::BOOL {
            <F as CartXorCollector<F, OUT>>::collect_both(
                &mut out[IdxOf::<OUT, Xor<L, R>>::USIZE],
                SwapParity::<L, R>::BOOL,
                &left[0],
                &right[0],
            );
        };
        // iterate right array
        <F as CartXorCollectInto<F, CO, OUT, tarr![L], B>>::collect(out, left, &right[1..]);
    }
}
// we've reached the end of the right array, done with this left element
impl<CO: CartXorCollector<F, OUT>, OUT, F: Field, L>
    CartXorCollectInto<F, CO, OUT, tarr![L], tarr![]> for F
{
    fn collect<'a>(_out: &'a mut [F], _left: &'a [F], _right: &'a [F]) {}
}

// MULTIPLE ELEMENT LEFT ARRAY CASES
// Now we need to match [L0, L1|A] and make 2 recursive calls
// one with tarr![L0] and one with TArr<L1, A>
impl<F: Field, CO: CartXorCollector<F, OUT>, OUT, L0, L1, A: TypeArray, B: TypeArray>
    CartXorCollectInto<F, CO, OUT, TArr<L0, TArr<L1, A>>, B> for F
where
    L0: CartXorCollectInto<F, CO, OUT, tarr![L0], B>,
    L1: CartXorCollectInto<F, CO, OUT, TArr<L1, A>, B>,
{
    fn collect<'a>(out: &'a mut [F], left: &'a [F], right: &'a [F]) {
        // L0 x B
        <L0 as CartXorCollectInto<F, CO, OUT, tarr![L0], B>>::collect(out, &left[..1], right);
        // [L1|A] x B
        <L1 as CartXorCollectInto<F, CO, OUT, TArr<L1, A>, B>>::collect(out, &left[1..], right)
    }
}
