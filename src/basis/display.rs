use crate::{
    basis::{Basis, ZeroVect},
    metric::Metric,
};
use core::fmt;
use typenum::{Unsigned, B0, B1};

impl fmt::Display for ZeroVect {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "0")
    }
}

impl<U: Unsigned, M: Metric> fmt::Display for Basis<U, M, B0> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "e")?;
        let mut n = U::to_usize();
        let mut i = 0;
        while n > 0 {
            if n & 1 == 1 {
                write!(f, "{}", i)?;
            }
            n >>= 1;
            i += 1;
        }
        Ok(())
    }
}

impl<U: Unsigned, M: Metric> fmt::Display for Basis<U, M, B1> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "-{}", Basis::<U, M, B0>::new())
    }
}
