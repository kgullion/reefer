use core::ops::{Add, BitOr, Sub};
use typenum::{ATerm, Bit, Eq, IsEqual, Or, Sub1, Sum, TArr, UInt, Unsigned, B0, B1, U0};

#[allow(unused)]
pub type IdxOf<A, V> = <A as IndexOf<V>>::Idx;
#[allow(unused)]
pub type Contains<A, V> = <A as IndexOf<V>>::Found;
pub type Get<A, I> = <A as At<I>>::Output;
// allows unused type parameters
#[allow(unused)]
pub trait IndexOf<V> {
    type Idx: Unsigned;
    type Found: Bit;
}
impl<V> IndexOf<V> for ATerm {
    type Idx = U0; // Always zero if not found
    type Found = B0;
}
impl<V: IsEqual<HD>, HD, TL: IndexOf<V>> IndexOf<V> for TArr<HD, TL>
where
    TL::Found: Add<TL::Idx> + BitOr<Eq<V, HD>>,
    Sum<TL::Found, TL::Idx>: Unsigned,
    Or<TL::Found, Eq<V, HD>>: Bit,
{
    type Idx = Sum<TL::Found, TL::Idx>; // Add one if V in TL
    type Found = Or<TL::Found, Eq<V, HD>>;
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
