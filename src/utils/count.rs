use core::ops::Add;
use typenum::{Add1, UInt, B0, B1, U0};

pub type Count<A, V> = <A as CountOf<V>>::Count;
pub trait CountOf<V> {
    type Count;
}
impl CountOf<B1> for U0 {
    type Count = U0;
}
impl<U: CountOf<B1>> CountOf<B1> for UInt<U, B0> {
    type Count = U::Count;
}
impl<U: CountOf<B1>> CountOf<B1> for UInt<U, B1>
where
    U::Count: Add<B1>,
{
    type Count = Add1<U::Count>;
}
