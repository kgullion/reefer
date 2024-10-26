pub mod contains;
pub mod count;
pub mod parity;
pub mod typeset;
use crate::ta;
use typenum::{Bit, TypeArray, B0, B1};

// If-Then-Else
pub type If<Cond, Then, Else> = <Cond as Branch<Then, Else>>::Output;
pub trait Branch<T, F>: Bit {
    type Output;
}
impl<T, F> Branch<T, F> for B0 {
    type Output = F;
}
impl<T, F> Branch<T, F> for B1 {
    type Output = T;
}

// Flatten TypeArray of TypeArrays
// allow unused
#[allow(unused)]
pub type Flat<A> = <A as Flatten>::Output;
pub trait Flatten {
    type Output: TypeArray;
}
impl Flatten for ta![] {
    // flat([]) = []
    type Output = ta![];
}
impl<B: Flatten> Flatten for ta![ta![] | B] {
    // flat([[] | B]) = flat(B)
    type Output = B::Output;
}
impl<L, A, B> Flatten for ta![ta![L | A] | B]
where
    ta![A | B]: Flatten,
{
    // flat([[L|A] | B]) = [L, flat([A | B])]
    type Output = ta![L, <ta![A | B] as Flatten>::Output];
}
