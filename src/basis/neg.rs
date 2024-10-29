use crate::{
    basis::{Basis, ZeroVect},
    metric::{DegenCheck, IsDegen, Metric},
    parity::{
        ConjugatePar, ConjugateParity, InvolutePar, InvoluteParity, ReversePar, ReverseParity,
    },
    traits::{Conjugate, Inverse, Involute, Normalize, Reverse},
    utils::{Branch, If},
};
use core::ops::{BitAnd, BitXor, Neg};
use typenum::{Bit, Unsigned, Xor, B1};

// -------------------------------------------------------------------------------------
// Negation
impl<M: Metric> Neg for ZeroVect<M> {
    type Output = ZeroVect<M>;
    #[inline(always)]
    fn neg(self) -> Self::Output {
        self
    }
}
impl<U: Unsigned, M: Metric, S: Bit + BitXor<B1, Output: Bit>> Neg for Basis<U, M, S> {
    type Output = Basis<U, M, Xor<S, B1>>;
    #[inline(always)]
    fn neg(self) -> Self::Output {
        Self::Output::default()
    }
}

// -------------------------------------------------------------------------------------
// Involution
impl<M: Metric> Involute for ZeroVect<M> {
    type Output = ZeroVect<M>;
    #[inline(always)]
    fn involute(self) -> Self::Output {
        self
    }
}
impl<U: Unsigned + InvolutePar, M: Metric, S: Bit + BitXor<InvoluteParity<U>, Output: Bit>> Involute
    for Basis<U, M, S>
{
    type Output = Basis<U, M, Xor<S, InvoluteParity<U>>>;
    #[inline(always)]
    fn involute(self) -> Self::Output {
        Self::Output::default()
    }
}

// -------------------------------------------------------------------------------------
// Reverse
impl<M: Metric> Reverse for ZeroVect<M> {
    type Output = ZeroVect<M>;
    #[inline(always)]
    fn reverse(self) -> Self::Output {
        self
    }
}
impl<U: Unsigned + ReversePar, M: Metric, S: Bit + BitXor<ReverseParity<U>, Output: Bit>> Reverse
    for Basis<U, M, S>
{
    type Output = Basis<U, M, Xor<S, ReverseParity<U>>>;
    #[inline(always)]
    fn reverse(self) -> Self::Output {
        Self::Output::default()
    }
}

// -------------------------------------------------------------------------------------
// Conjugation
impl<M: Metric> Conjugate for ZeroVect<M> {
    type Output = ZeroVect<M>;
    #[inline(always)]
    fn conjugate(self) -> Self::Output {
        self
    }
}
impl<U: Unsigned + ConjugatePar, M: Metric, S: Bit + BitXor<ConjugateParity<U>, Output: Bit>>
    Conjugate for Basis<U, M, S>
{
    type Output = Basis<U, M, Xor<S, ConjugateParity<U>>>;
    #[inline(always)]
    fn conjugate(self) -> Self::Output {
        Self::Output::default()
    }
}

// -------------------------------------------------------------------------------------
// Inverse - only non-degenerate bases have inverses (the reverse)
impl<M: Metric> Inverse for ZeroVect<M> {
    type Output = ZeroVect<M>;
    #[inline(always)]
    fn inverse(self) -> Option<Self::Output> {
        None
    }
}
impl<
        U: Unsigned
            + DegenCheck<
                M,
                Output: Branch<ZeroVect<M>, Basis<U, M, Xor<S, ReverseParity<U>>>, Output: Default>,
            > + ReversePar
            + BitAnd<M::ZeroMask>,
        M: Metric,
        S: Bit + BitXor<ReverseParity<U>, Output: Bit>,
    > Inverse for Basis<U, M, S>
{
    type Output = If<IsDegen<M, U>, ZeroVect<M>, Basis<U, M, Xor<S, ReverseParity<U>>>>;
    #[inline(always)]
    fn inverse(self) -> Option<Self::Output> {
        if IsDegen::<M, U>::BOOL {
            None
        } else {
            Some(Self::Output::default())
        }
    }
}

// Normalize - Basis all length 1 (other than the zero vector) so they are already normalized
impl<M: Metric> Normalize for ZeroVect<M> {
    type Output = ZeroVect<M>;
    #[inline(always)]
    fn normalize(self) -> Self {
        self
    }
}
impl<U: Unsigned, M: Metric, S: Bit> Normalize for Basis<U, M, S> {
    type Output = Self;
    #[inline(always)]
    fn normalize(self) -> Self {
        self
    }
}
