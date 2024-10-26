pub mod contains;
pub mod count;
pub mod parity;
pub mod typeset;

use typenum::{tarr, Bit, TArr, TypeArray, B0, B1};

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
impl Flatten for tarr![] {
    // flat([]) = []
    type Output = tarr![];
}
impl<B: Flatten> Flatten for TArr<tarr![], B> {
    // flat([[] | B]) = flat(B)
    type Output = B::Output;
}
impl<L, A, B> Flatten for TArr<TArr<L, A>, B>
where
    TArr<A, B>: Flatten,
{
    // flat([[L|A] | B]) = [L, flat([A | B])]
    type Output = TArr<L, <TArr<A, B> as Flatten>::Output>;
}
