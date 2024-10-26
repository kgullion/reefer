use crate::{
    basis::{Basis, ZeroVect},
    metric::{DegenCheck, IsDegen, Metric},
    traits::{Conjugate, Inverse, Involute, Normalize, Reverse},
    utils::{
        parity::{ConjugatePar, InvolutePar, ReversePar},
        Branch, If,
    },
};
use core::ops::{BitAnd, BitXor};
use typenum::{Bit, Unsigned, Xor, B1};

// -------------------------------------------------------------------------------------
// Negation
impl core::ops::Neg for ZeroVect {
    type Output = ZeroVect;
    #[inline(always)]
    fn neg(self) -> Self::Output {
        self
    }
}
impl<U: Unsigned, M: Metric, S: Bit + BitXor<B1, Output: Bit>> core::ops::Neg for Basis<U, M, S> {
    type Output = Basis<U, M, Xor<S, B1>>;
    #[inline(always)]
    fn neg(self) -> Self::Output {
        Self::Output::default()
    }
}

// -------------------------------------------------------------------------------------
// Involution
impl Involute for ZeroVect {
    type Output = ZeroVect;
    #[inline(always)]
    fn involute(self) -> Self::Output {
        self
    }
}
impl<U: Unsigned + InvolutePar, M: Metric, S: Bit + BitXor<B1, Output: Bit>> Involute
    for Basis<U, M, S>
{
    type Output = Basis<U, M, <U as InvolutePar>::Parity>;
    #[inline(always)]
    fn involute(self) -> Self::Output {
        Self::Output::default()
    }
}

// -------------------------------------------------------------------------------------
// Reverse
impl Reverse for ZeroVect {
    type Output = ZeroVect;
    #[inline(always)]
    fn reverse(self) -> Self::Output {
        self
    }
}
impl<U: Unsigned + ReversePar, M: Metric, S: Bit + BitXor<B1, Output: Bit>> Reverse
    for Basis<U, M, S>
{
    type Output = Basis<U, M, <U as ReversePar>::Parity>;
    #[inline(always)]
    fn reverse(self) -> Self::Output {
        Self::Output::default()
    }
}

// -------------------------------------------------------------------------------------
// Conjugation
impl Conjugate for ZeroVect {
    type Output = ZeroVect;
    #[inline(always)]
    fn conjugate(self) -> Self::Output {
        self
    }
}
impl<U: Unsigned + ConjugatePar, M: Metric, S: Bit + BitXor<B1, Output: Bit>> Conjugate
    for Basis<U, M, S>
{
    type Output = Basis<U, M, <U as ConjugatePar>::Parity>;
    #[inline(always)]
    fn conjugate(self) -> Self::Output {
        Self::Output::default()
    }
}

// -------------------------------------------------------------------------------------
// Inverse - only non-degenerate bases have inverses (the reverse)
impl Inverse for ZeroVect {
    type Output = ZeroVect;
    #[inline(always)]
    fn inverse(self) -> Option<Self::Output> {
        None
    }
}
impl<
        U: Unsigned
            + DegenCheck<
                M,
                Output: Branch<ZeroVect, Basis<U, M, <U as ReversePar>::Parity>, Output: Default>,
            > + ReversePar
            + BitAnd<M::ZeroMask>,
        M: Metric,
        S: Bit + BitXor<B1, Output: Bit>,
    > Inverse for Basis<U, M, S>
{
    type Output = If<IsDegen<M, U>, ZeroVect, Basis<U, M, <U as ReversePar>::Parity>>;
    #[inline(always)]
    fn inverse(self) -> Option<Self::Output> {
        if IsDegen::<M, U>::BOOL {
            None
        } else {
            Some(Self::Output::default())
        }
    }
}

// Normalize - Basis all all length 1 (other than the zero vector) so they are already normalized
impl Normalize for ZeroVect {
    #[inline(always)]
    fn normalize(self) -> Self {
        self
    }
}
impl<U: Unsigned, M: Metric, S: Bit> Normalize for Basis<U, M, S> {
    #[inline(always)]
    fn normalize(self) -> Self {
        self
    }
}
