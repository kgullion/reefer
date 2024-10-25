use core::ops::{Add, BitOr, Sub};
use typenum::{ATerm, Bit, Eq, IsEqual, Or, Sub1, Sum, TArr, UInt, Unsigned, B0, B1, U0};

pub type IdxOf<A, V> = <A as IndexOf<V>>::Idx;
pub type Contains<A, V> = <A as IndexOf<V>>::Found;
pub type Get<A, I> = <A as At<I>>::Output;

pub trait IndexOf<V> {
    type Idx: Unsigned;
    type Found: Bit;
}
impl<V> IndexOf<V> for ATerm {
    type Idx = U0; // Always zero if not found
    type Found = B0;
}
impl<V, HD, TL: IndexOf<V>> IndexOf<V> for TArr<HD, TL>
where
    V: IsEqual<HD, Output: BitOr<TL::Found, Output: Bit>>,
    TL::Idx: Add<TL::Found, Output: Unsigned>,
{
    type Idx = Sum<TL::Idx, TL::Found>; // Add one if V in TL
    type Found = Or<Eq<V, HD>, TL::Found>;
}

pub trait At<Idx: Unsigned> {
    type Output;
}

impl<HD, TL> At<U0> for TArr<HD, TL> {
    type Output = HD;
}
impl<U: Unsigned, B: Bit, HD, TL: At<Sub1<UInt<U, B>>>> At<UInt<U, B>> for TArr<HD, TL>
where
    UInt<U, B>: Sub<B1>,
    Sub1<UInt<U, B>>: Unsigned,
{
    type Output = TL::Output;
}

impl<U: Unsigned> At<U> for U0 {
    type Output = B0; // every bit of 0 is 0
}
impl<HD, TL> At<U0> for UInt<TL, HD> {
    type Output = HD;
}
impl<U: Unsigned, B: Bit, HD, TL: At<Sub1<UInt<U, B>>>> At<UInt<U, B>> for UInt<TL, HD>
where
    UInt<U, B>: Sub<B1>,
    Sub1<UInt<U, B>>: Unsigned,
{
    type Output = TL::Output;
}

#[cfg(test)]
mod tests {
    use super::*;
    use typenum::{assert_type_eq, tarr, U0, U1, U10, U2, U3, U4, U5, U6, U7, U8, U9};

    type A = tarr![U9, U8, U7, U6, U5, U4, U3, U2, U1, U0];
    #[test]
    fn test_idx_of() {
        assert_eq!(IdxOf::<A, U0>::USIZE, 9);
        assert_eq!(IdxOf::<A, U1>::USIZE, 8);
        assert_eq!(IdxOf::<A, U2>::USIZE, 7);
        assert_eq!(IdxOf::<A, U3>::USIZE, 6);
        assert_eq!(IdxOf::<A, U4>::USIZE, 5);
        assert_eq!(IdxOf::<A, U5>::USIZE, 4);
        assert_eq!(IdxOf::<A, U6>::USIZE, 3);
        assert_eq!(IdxOf::<A, U7>::USIZE, 2);
        assert_eq!(IdxOf::<A, U8>::USIZE, 1);
        assert_eq!(IdxOf::<A, U9>::USIZE, 0);
        assert_eq!(IdxOf::<A, U10>::USIZE, 0); // not found still has index 0
    }
    #[test]
    fn test_contains() {
        assert!(Contains::<A, U0>::BOOL);
        assert!(Contains::<A, U1>::BOOL);
        assert!(Contains::<A, U2>::BOOL);
        assert!(Contains::<A, U3>::BOOL);
        assert!(Contains::<A, U4>::BOOL);
        assert!(Contains::<A, U5>::BOOL);
        assert!(Contains::<A, U6>::BOOL);
        assert!(Contains::<A, U7>::BOOL);
        assert!(Contains::<A, U8>::BOOL);
        assert!(Contains::<A, U9>::BOOL);
        assert!(!Contains::<A, U10>::BOOL); // make sure fail to find works
    }
    /// ```compile_fail
    /// Get::<A, U10>::USIZE; // U10 is not in A so this should fail to compile
    /// ```
    #[test]
    fn test_get() {
        assert_type_eq!(Get::<U0, U0>, B0);
        assert_type_eq!(Get::<U1, U0>, B1);
        assert_type_eq!(Get::<U1, U1>, B0);
        assert_eq!(Get::<A, U0>::USIZE, 9);
        assert_eq!(Get::<A, U1>::USIZE, 8);
        assert_eq!(Get::<A, U2>::USIZE, 7);
        assert_eq!(Get::<A, U3>::USIZE, 6);
        assert_eq!(Get::<A, U4>::USIZE, 5);
        assert_eq!(Get::<A, U5>::USIZE, 4);
        assert_eq!(Get::<A, U6>::USIZE, 3);
        assert_eq!(Get::<A, U7>::USIZE, 2);
        assert_eq!(Get::<A, U8>::USIZE, 1);
        assert_eq!(Get::<A, U9>::USIZE, 0);
    }
}
