use super::{Count, Get};
use core::ops::{BitAnd, BitXor};
use typenum::{And, Bit, UInt, Unsigned, Xor, B0, B1, U0};

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
#[allow(unused)]
pub type SwapParity<L, R> = <L as SwapPar<R, Get<Count<L, B1>, U0>, B0>>::Parity;

pub trait SwapPar<R, LP, TP> {
    type Parity;
}

// base cases, swap parity is total parity
impl<LP: Bit, TP: Bit> SwapPar<U0, LP, TP> for U0 {
    type Parity = TP;
}
impl<LU: Unsigned, LB: Bit, LP: Bit, TP: Bit> SwapPar<U0, LP, TP> for UInt<LU, LB> {
    type Parity = TP;
}
impl<RU: Unsigned, RB: Bit, LP: Bit, TP: Bit> SwapPar<UInt<RU, RB>, LP, TP> for U0 {
    type Parity = TP;
}

// recursive case, see above for explanation
impl<
        LU: Unsigned + SwapPar<RU, Xor<LP, LB>, Xor<TP, And<RB, Xor<LP, LB>>>>, // 🫠
        RU: Unsigned,
        LB: Bit,
        RB: Bit + BitAnd<Xor<LP, LB>>,
        LP: Bit + BitXor<LB>,
        TP: Bit + BitXor<And<RB, Xor<LP, LB>>>,
    > SwapPar<UInt<RU, RB>, LP, TP> for UInt<LU, LB>
{
    type Parity = LU::Parity;
}

#[cfg(test)]
mod tests {
    use super::*;
    #[allow(unused_imports)]
    use typenum::{B0, B1, U0, U1, U2, U3, U4, U5, U6, U7, U8, U9};

    #[test]
    fn swap_parity() {
        assert!(!SwapParity::<U0, U0>::BOOL);
        assert!(!SwapParity::<U1, U2>::BOOL);
        assert!(SwapParity::<U2, U1>::BOOL);
    }
}
