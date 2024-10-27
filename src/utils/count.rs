use crate::ta;
use core::ops::Add;
use typenum::{Add1, Eq, IsEqual, Sum, UInt, Unsigned, B0, B1, U0};

pub type Count<A, V> = <A as CountOf<V>>::Count;
pub trait CountOf<V> {
    type Count: Unsigned;
}
impl<T> CountOf<T> for ta![] {
    type Count = U0;
}
impl<T: IsEqual<H>, H, U: CountOf<T, Count: Add<Eq<T, H>, Output: Unsigned>>> CountOf<T>
    for ta![H | U]
{
    type Count = Sum<U::Count, Eq<T, H>>;
}

impl CountOf<B1> for U0 {
    type Count = U0;
}
impl<U: CountOf<B1>> CountOf<B1> for UInt<U, B0> {
    type Count = U::Count;
}
impl<U: CountOf<B1>> CountOf<B1> for UInt<U, B1>
where
    U::Count: Add<B1, Output: Unsigned>,
{
    type Count = Add1<U::Count>;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ta;
    use typenum::{U1, U10, U2, U3, U4, U5, U6, U7, U8, U9};

    type A = ta![U8, U4, U4, U2, U4, U8, U4, U2, U1, U0];
    #[test]
    fn test_count_is_zero() {
        assert_eq!(Count::<A, U3>::USIZE, 0);
        assert_eq!(Count::<A, U5>::USIZE, 0);
        assert_eq!(Count::<A, U6>::USIZE, 0);
        assert_eq!(Count::<A, U7>::USIZE, 0);
        assert_eq!(Count::<A, U9>::USIZE, 0);
        assert_eq!(Count::<A, U10>::USIZE, 0);
    }
    #[test]
    fn test_count_is_one() {
        assert_eq!(Count::<A, U0>::USIZE, 1);
        assert_eq!(Count::<A, U1>::USIZE, 1);
    }
    #[test]
    fn test_count_multiple() {
        assert_eq!(Count::<A, U2>::USIZE, 2);
        assert_eq!(Count::<A, U4>::USIZE, 4);
        assert_eq!(Count::<A, U8>::USIZE, 2);
    }
    #[test]
    fn test_count_b1() {
        assert_eq!(Count::<U0, B1>::USIZE, 0);
        assert_eq!(Count::<U1, B1>::USIZE, 1);
        assert_eq!(Count::<U2, B1>::USIZE, 1);
        assert_eq!(Count::<U3, B1>::USIZE, 2);
        assert_eq!(Count::<U4, B1>::USIZE, 1);
        assert_eq!(Count::<U5, B1>::USIZE, 2);
        assert_eq!(Count::<U6, B1>::USIZE, 2);
        assert_eq!(Count::<U7, B1>::USIZE, 3);
        assert_eq!(Count::<U8, B1>::USIZE, 1);
        assert_eq!(Count::<U9, B1>::USIZE, 2);
        assert_eq!(Count::<U10, B1>::USIZE, 2);
    }
}
