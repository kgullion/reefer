use super::{Basis, ZeroVector};
use crate::metric::Metric;
use core::marker::PhantomData;
use typenum::{Bit, Unsigned};

impl Default for ZeroVector {
    #[inline(always)]
    fn default() -> Self {
        Self
    }
}
impl<U: Unsigned, M: Metric, S: Bit> Default for Basis<U, M, S> {
    #[inline(always)]
    fn default() -> Self {
        Self(PhantomData)
    }
}
