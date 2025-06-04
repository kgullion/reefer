use std::{cmp::min, fmt::Debug, mem::swap};

/// track parity of inversions in an array while performing merge sort
pub fn parity_sort<T: PartialOrd + Clone + Debug>(data: &mut [T]) -> bool {
    if data.len() <= 1 {
        return false;
    }
    let mut auxiliary = vec![data[0].clone(); data.len()];
    do_parity_sort(data, &mut auxiliary)
}

fn do_parity_sort<T: PartialOrd + Clone + Debug>(data: &mut [T], auxiliary: &mut [T]) -> bool {
    let len = data.len();
    let mut parity = false;
    // declare stride variables (a stride is a slice known to be in sorted order)
    let mut stride = 1; // strides of length 1 are guaranteed to be sorted
    let mut arr: &mut [T]; // all strides over data
    let mut aux: &mut [T]; // all strides over auxiliary
    let mut lhs: &mut [T]; // each left stride for arr
    let mut rhs: &mut [T]; // each right stride for arr
    let mut out: &mut [T]; // both l & r strides for out
    while stride < len {
        // reset striders
        arr = data;
        aux = auxiliary;
        // take first left- & right-hand sorted strides
        (lhs, arr) = split_at_mut(arr, stride);
        (rhs, arr) = split_at_mut(arr, stride);
        while !rhs.is_empty() {
            // merge the sorted lhs and rhs into out
            (out, aux) = split_at_mut(aux, 2 * stride);
            do_parity_merge(lhs, rhs, out, &mut parity);
            // split off next left- & right-hand sorted strides
            (lhs, arr) = split_at_mut(arr, stride);
            (rhs, arr) = split_at_mut(arr, stride);
        }
        let i = len - lhs.len(); // lhs is the sorted tail of data, so leave it in place
        swap_with_slice(&mut data[..i], &mut auxiliary[..i]); // and get sorted elems from aux
        stride <<= 1; // finally, double the stride and repeat until sorted
    }
    debug_assert!(data.is_sorted());
    parity
}

/// parity merge two sorted vecs in O(lhs.len + rhs.len)
pub fn parity_merge<T: PartialOrd + Clone + Debug>(
    mut lhs: Vec<T>,
    mut rhs: Vec<T>,
) -> (bool, Vec<T>) {
    match (lhs.len(), rhs.len()) {
        (_, 0) => (false, lhs),
        (0, _) => (false, rhs),
        (m, n) => {
            let mut out = vec![lhs[0].clone(); m + n];
            let mut par = false;
            do_parity_merge(&mut lhs, &mut rhs, &mut out, &mut par);
            (par, out)
        }
    }
}

/// merge all of lhs and rhs into out. panics if out is too small.
pub fn do_parity_merge<T: PartialOrd + Debug>(
    mut lhs: &mut [T],
    mut rhs: &mut [T],
    mut out: &mut [T],
    parity: &mut bool,
) {
    debug_assert!(!lhs.is_empty());
    debug_assert!(lhs.is_sorted());
    debug_assert!(!rhs.is_empty());
    debug_assert!(rhs.is_sorted());
    debug_assert_eq!(lhs.len() + rhs.len(), out.len());
    if lhs[lhs.len() - 1] <= rhs[0] {
        let (l_o, r_o) = split_at_mut(out, lhs.len());
        swap_with_slice(lhs, l_o);
        swap_with_slice(rhs, r_o);
        return;
    }
    if rhs[rhs.len() - 1] < lhs[0] {
        let (r_o, l_o) = split_at_mut(out, rhs.len());
        swap_with_slice(rhs, r_o);
        swap_with_slice(lhs, l_o);
        *parity ^= lhs.len() & rhs.len() & 1 == 1;
        return;
    }
    loop {
        if lhs[0] <= rhs[0] {
            swap(&mut out[0], &mut lhs[0]); // ordered, take the first element from lhs
            lhs = &mut lhs[1..]; // advance lhs read pointer
            out = &mut out[1..]; // advance write pointer
            if lhs.is_empty() {
                swap_with_slice(rhs, out); // move sorted leftovers
                debug_assert!(out.is_sorted());
                debug_assert_eq!(lhs.len() + rhs.len(), out.len());
                return;
            }
        } else {
            swap(&mut out[0], &mut rhs[0]); // inverted, take the first element from rhs
            *parity ^= lhs.len() & 1 != 0; // after swapping past all the elements left in lhs
            rhs = &mut rhs[1..]; // advance rhs read pointer
            out = &mut out[1..]; // advance write pointer
            if rhs.is_empty() {
                swap_with_slice(lhs, out); // move sorted leftovers
                debug_assert!(out.is_sorted());
                debug_assert_eq!(lhs.len() + rhs.len(), out.len());
                return;
            }
        }
    }
}

fn split_at_mut<T>(arr: &mut [T], mid: usize) -> (&mut [T], &mut [T]) {
    debug_assert!(min(mid, arr.len()) <= arr.len());
    // SAFETY: the min of stride and arr len is always less or equal to arr len
    unsafe { arr.split_at_mut_unchecked(min(mid, arr.len())) }
}
fn swap_with_slice<T>(x: &mut [T], y: &mut [T]) {
    debug_assert_eq!(x.len(), y.len());
    // SAFETY: every call in this module always has the same lengths
    unsafe { core::ptr::swap_nonoverlapping(x.as_mut_ptr(), y.as_mut_ptr(), y.len()) }
}

#[cfg(test)]
mod tests {
    use super::*;
    use itertools::Itertools;

    /// wrapper to ensure no aux data is leaking anywhere, even if it happens to be the correct value
    #[derive(Debug, Clone, PartialOrd, PartialEq)]
    enum AuxGuard<T> {
        Aux,
        Data(T),
    }
    fn guarded_parity_sort<T: PartialOrd + Clone + Debug>(data: Vec<T>) -> (Vec<T>, bool) {
        let len = data.len();
        if len <= 1 {
            return (data, false);
        }
        let mut guarded = data.into_iter().map(AuxGuard::Data).collect_vec();
        let mut aux = vec![AuxGuard::Aux; len];
        let p = do_parity_sort(&mut guarded, &mut aux);
        let out = guarded
            .into_iter()
            .map(|g| match g {
                AuxGuard::Aux => panic!("OHNO"),
                AuxGuard::Data(d) => d,
            })
            .collect();
        (out, p)
    }

    #[test]
    fn test_empty_array() {
        let arr: Vec<usize> = vec![];
        let (arr, inversions) = guarded_parity_sort(arr);
        assert!(arr.is_empty());
        assert_eq!(inversions, false);
    }

    #[test]
    fn test_single_element() {
        let arr = [5].to_vec();
        let (arr, inversions) = guarded_parity_sort(arr);
        assert_eq!(inversions, false);
        assert_eq!(arr, [5]);
    }

    #[test]
    fn test_two_element_sorted() {
        let arr = ["1", "2"].to_vec();
        let (arr, inversions) = guarded_parity_sort(arr);
        assert_eq!(inversions, false);
        assert_eq!(arr, ["1", "2"]);
    }

    #[test]
    fn test_two_element() {
        let arr = [5, 1].to_vec();
        let (arr, inversions) = guarded_parity_sort(arr);
        assert_eq!(inversions, true);
        assert_eq!(arr, [1, 5]);
    }

    #[test]
    fn test_sorted_array() {
        let arr = [1, 2, 3, 4, 5].to_vec();
        let (arr, inversions) = guarded_parity_sort(arr);
        assert_eq!(inversions, false);
        assert_eq!(arr, [1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_reverse_sorted_array() {
        let arr = [5, 4, 3, 2, 1].to_vec();
        let (arr, inversions) = guarded_parity_sort(arr);
        assert_eq!(inversions, false);
        assert_eq!(arr, [1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_partially_sorted_array() {
        let arr = [1, 3, 2, 5].to_vec();
        let (arr, inversions) = guarded_parity_sort(arr);
        assert_eq!(inversions, true);
        assert_eq!(arr, [1, 2, 3, 5]);
    }

    #[test]
    fn test_example_array() {
        let arr = [5, 3, 2, 4, 1].to_vec();
        let (arr, inversions) = guarded_parity_sort(arr);
        assert_eq!(inversions, false);
        assert_eq!(arr, [1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_duplicate_elements() {
        let arr = [4, 2, 3, 1, 2].to_vec();
        let (arr, inversions) = guarded_parity_sort(arr);
        assert_eq!(inversions, true);
        assert_eq!(arr, [1, 2, 2, 3, 4]);
    }

    #[test]
    fn test_large_array() {
        let arr: Vec<i32> = (0..1000).rev().collect();
        let (arr, inversions) = guarded_parity_sort(arr);
        assert_eq!(inversions, false);
        assert!(arr.is_sorted());
    }
}
