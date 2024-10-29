use crate::{
    basis::{Basis, ZeroVect},
    metric::Metric,
};
use core::marker::PhantomData;
use typenum::{Bit, Unsigned};

impl<M: Metric> Default for ZeroVect<M> {
    #[inline(always)]
    fn default() -> Self {
        Self(PhantomData)
    }
}
impl<U: Unsigned, M: Metric, S: Bit> Default for Basis<U, M, S> {
    #[inline(always)]
    fn default() -> Self {
        Self(PhantomData)
    }
}
