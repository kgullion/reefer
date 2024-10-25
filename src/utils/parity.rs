use super::{At, Count, CountOf, Get};
use core::ops::{Add, BitAnd, BitXor};
use typenum::{And, Bit, UInt, Unsigned, Xor, B0, B1, U0, U1};

// ---------------------------------------------------------------------------------------
// SwapParity - parity of swaps needed to merge two basis vectorsets
//
// looping - easiest to understand
// const fn swap_par(mut lhs: u32, mut rhs: u32) -> bool {
//     let mut prt = lhs.count_ones(); // parity of swaps to move from back to front in lhs
//     let mut tot = 0;                // parity of swaps to mergesort lhs and rhs
//     while rhs != 0 {
//         prt ^= lhs;       // flip parity of lhs if lsb(lhs)
//         tot ^= rhs & prt; // flip total parity if odd dist to front
//         lhs >>= 1;        // drop lowest bit from lhs
//         rhs >>= 1;        // drop lowest bit from rhs
//     }
//     tot & 1 != 0          // return true if parity is odd
// }
// recursive - f=(l,r)->g(l,r,l.bitcnt,0);g=(l,r,p,t)->!r?t&1:g(l>>1,r>>1,p^l,t^(r&(p^l)))
// const fn swap_par(l: u32, r: u32) -> bool {
//     swap_par_inner(l, r, l.count_ones(), 0)
// const fn swap_par_inner(l: u32, r: u32, p: u32, t: u32) -> bool {
//     if r == 0 {
//         t & 1 != 0
//     } else { // I'm skeptical that you could yet🚬intrigued that you may
//         swap_par_inner(l >> 1, r >> 1, p ^ l, t ^ (r & (p ^ l)))
//     } //             ↙        ↙      ↙   ↙    ↓     ↘     ↘   ↘
// } //            LU: SwapPar<RU,Xor<LP,LB>,Xor<TP,And<RB,Xor<LP,LB>>>>>
//
// Dang-ol python one-liner:
// f=lambda l,r:g(l,r,l.bit_count(),0);g=lambda l,r,p,t:t&1 if r==0 else g(l>>1,r>>1,p^l,t^(r&(p^l)))
//
// and finally, encoded into types:
/// Parity of the symmetric difference of two basis vectors.
pub type SwapParity<L, R> = <L as SwapPar<R>>::Parity;
pub trait SwapPar<R> {
    type Parity;
}
impl<
        L: Unsigned + CountOf<B1, Count: At<U0>> + SwapParInner<R, Get<Count<L, B1>, U0>, B0>,
        R: Unsigned,
    > SwapPar<R> for L
{
    type Parity = <L as SwapParInner<R, Get<Count<L, B1>, U0>, B0>>::Parity;
}

pub trait SwapParInner<R, LP, TP> {
    type Parity;
}

// base cases, swap parity is total parity
impl<LP: Bit, TP: Bit> SwapParInner<U0, LP, TP> for U0 {
    type Parity = TP;
}
impl<LU: Unsigned, LB: Bit, LP: Bit, TP: Bit> SwapParInner<U0, LP, TP> for UInt<LU, LB> {
    type Parity = TP;
}
impl<RU: Unsigned, RB: Bit, LP: Bit, TP: Bit> SwapParInner<UInt<RU, RB>, LP, TP> for U0 {
    type Parity = TP;
}

// recursive case, see above for explanation
impl<
        LU: Unsigned + SwapParInner<RU, Xor<LP, LB>, Xor<TP, And<RB, Xor<LP, LB>>>>, // 🫠
        RU: Unsigned,
        LB: Bit,
        RB: Bit + BitAnd<Xor<LP, LB>>,
        LP: Bit + BitXor<LB>,
        TP: Bit + BitXor<And<RB, Xor<LP, LB>>>,
    > SwapParInner<UInt<RU, RB>, LP, TP> for UInt<LU, LB>
{
    type Parity = LU::Parity;
}

// type IsInvolute = Flip<Get<U::Count, U0>>; // grade & 1 != 0
// type IsReverse = Flip<Get<Self::Grade, U1>>; // grade & 2 != 0
// type IsConjugate = Flip<Get<Add1<Self::Grade>, U1>>; // (grade + 1) & 2 != 0
// type IsZero = B0;

// ---------------------------------------------------------------------------------------
#[allow(unused)]
pub type InvoluteParity<U> = <U as InvolutePar>::Parity;
pub trait InvolutePar: Unsigned {
    type Parity: Bit;
}
impl<U: Unsigned + CountOf<B1, Count: At<U0, Output: Bit>>> InvolutePar for U {
    // grade & 1 != 0
    type Parity = Get<Count<U, B1>, U0>;
}

// ---------------------------------------------------------------------------------------
pub type ReverseParity<U> = <U as ReversePar>::Parity;
pub trait ReversePar: Unsigned {
    type Parity: Bit;
}
impl<U: Unsigned + CountOf<B1, Count: At<U1, Output: Bit>>> ReversePar for U {
    // grade & 2 != 0
    type Parity = Get<Count<U, B1>, U1>;
}

// ---------------------------------------------------------------------------------------
#[allow(unused)]
pub type ConjugateParity<U> = <U as ConjugatePar>::Parity;
pub trait ConjugatePar: Unsigned {
    type Parity: Bit;
}
impl<U: Unsigned + CountOf<B1, Count: At<U1, Output: Bit> + Add<B1>>> ConjugatePar for U {
    // (grade + 1) & 2 != 0
    type Parity = Get<Count<U, B1>, U1>;
}

#[cfg(test)]
mod tests {
    use super::*;
    #[allow(unused_imports)]
    use typenum::{B0, B1, U0, U1, U2, U3, U4, U5, U6, U7, U8, U9};
    /// U0 represents the empty set, so the parity of the symmetric difference is always false.
    #[test]
    fn test_swap_parity_scalar() {
        assert_eq!(SwapParity::<U0, U1>::BOOL, false); // 1 * e1 = e1
        assert_eq!(SwapParity::<U1, U0>::BOOL, false); // e1 * 1 = e1

        assert_eq!(SwapParity::<U0, U2>::BOOL, false); // 1 * e2 = e2
        assert_eq!(SwapParity::<U2, U0>::BOOL, false); // e2 * 1 = e2

        assert_eq!(SwapParity::<U0, U3>::BOOL, false); // 1 * e12 = e12
        assert_eq!(SwapParity::<U3, U0>::BOOL, false); // e12 * 1 = e12

        assert_eq!(SwapParity::<U0, U4>::BOOL, false); // 1 * e3 = e3
        assert_eq!(SwapParity::<U4, U0>::BOOL, false); // e3 * 1 = e3

        assert_eq!(SwapParity::<U0, U7>::BOOL, false); // 1 * e123 = e123
        assert_eq!(SwapParity::<U7, U0>::BOOL, false); // e123 * 1 = e123

        assert_eq!(SwapParity::<U0, U9>::BOOL, false); // 1 * e14 = e14
        assert_eq!(SwapParity::<U9, U0>::BOOL, false); // e14 * 1 = e14
    }
    /// Basis vectors are represented by PowersOfTwo, so the swap parity should be asymmetric for pairs of PowersOfTwo.
    #[test]
    fn test_swap_parity_vectors() {
        assert_eq!(SwapParity::<U1, U2>::BOOL, false); // e1 * e2 = e12
        assert_eq!(SwapParity::<U2, U1>::BOOL, true); // e2 * e1 = -e12

        assert_eq!(SwapParity::<U1, U4>::BOOL, false); // e1 * e3 = e13
        assert_eq!(SwapParity::<U4, U1>::BOOL, true); // e3 * e1 = -e13

        assert_eq!(SwapParity::<U1, U8>::BOOL, false); // e1 * e4 = e14
        assert_eq!(SwapParity::<U8, U1>::BOOL, true); // e4 * e1 = -e14

        assert_eq!(SwapParity::<U2, U4>::BOOL, false); // e2 * e3 = e23
        assert_eq!(SwapParity::<U4, U2>::BOOL, true); // e3 * e2 = -e23

        assert_eq!(SwapParity::<U2, U8>::BOOL, false); // e2 * e4 = e24
        assert_eq!(SwapParity::<U8, U2>::BOOL, true); // e4 * e2 = -e24

        assert_eq!(SwapParity::<U4, U8>::BOOL, false); // e3 * e4 = e34
        assert_eq!(SwapParity::<U8, U4>::BOOL, true); // e4 * e3 = -e34
    }
    /// General multivectors are Unions of PowersOfTwo, so the swap parity depends on the particular multivectors.
    #[test]
    fn test_swap_parity_blades() {
        assert_eq!(SwapParity::<U3, U5>::BOOL, true); // e12 * e13 = -e23
        assert_eq!(SwapParity::<U5, U3>::BOOL, false); // e13 * e12 = e23

        assert_eq!(SwapParity::<U3, U6>::BOOL, false); // e12 * e23 = e13
        assert_eq!(SwapParity::<U6, U3>::BOOL, true); // e23 * e12 = -e13

        assert_eq!(SwapParity::<U3, U7>::BOOL, true); // e12 * e123 = -e3
        assert_eq!(SwapParity::<U7, U3>::BOOL, true); // e123 * e12 = -e3

        assert_eq!(SwapParity::<U3, U9>::BOOL, true); // e12 * e14 = -e24
        assert_eq!(SwapParity::<U9, U3>::BOOL, false); // e14 * e12 = e24

        assert_eq!(SwapParity::<U5, U6>::BOOL, true); // e13 * e23 = -e12
        assert_eq!(SwapParity::<U6, U5>::BOOL, false); // e23 * e13 = e12

        assert_eq!(SwapParity::<U5, U7>::BOOL, false); // e13 * e123 = e2
        assert_eq!(SwapParity::<U7, U5>::BOOL, false); // e123 * e13 = e2

        assert_eq!(SwapParity::<U5, U9>::BOOL, true); // e13 * e14 = -e4
        assert_eq!(SwapParity::<U9, U5>::BOOL, false); // e14 * e13 = e4

        assert_eq!(SwapParity::<U6, U7>::BOOL, true); // e23 * e123 = -e1
        assert_eq!(SwapParity::<U7, U6>::BOOL, true); // e123 * e23 = -e1

        assert_eq!(SwapParity::<U6, U9>::BOOL, false); // e23 * e14 = e1234
        assert_eq!(SwapParity::<U9, U6>::BOOL, false); // e14 * e23 = e1234
    }
    /// Test squares of various blades.
    #[test]
    fn test_swap_parity_scalars() {
        assert_eq!(SwapParity::<U0, U0>::BOOL, false); // 1 * 1 = 1
        assert_eq!(SwapParity::<U1, U1>::BOOL, false); // e1 * e1 = 1
        assert_eq!(SwapParity::<U2, U2>::BOOL, false); // e2 * e2 = 1
        assert_eq!(SwapParity::<U3, U3>::BOOL, true); // e12 * e12 = -1
        assert_eq!(SwapParity::<U4, U4>::BOOL, false); // e3 * e3 = 1
        assert_eq!(SwapParity::<U5, U5>::BOOL, true); // e13 * e13 = -1
        assert_eq!(SwapParity::<U6, U6>::BOOL, true); // e23 * e23 = -1
        assert_eq!(SwapParity::<U7, U7>::BOOL, true); // e123 * e123 = -1
        assert_eq!(SwapParity::<U8, U8>::BOOL, false); // e4 * e4 = 1
        assert_eq!(SwapParity::<U9, U9>::BOOL, true); // e14 * e14 = -1
    }
}
