//! Top K Frequent Elements.
//!
//! Tally counts, then drain a max-heap keyed on frequency `k` times. (The C#
//! version leaned on a bespoke `MaxHeap`; Rust's [`BinaryHeap`] is one already.)

use std::collections::{BinaryHeap, HashMap};

pub fn solve(nums: &[i32], k: usize) -> Vec<i32> {
    let mut counts: HashMap<i32, i32> = HashMap::new();
    for &n in nums {
        *counts.entry(n).or_insert(0) += 1;
    }

    // Ordered by frequency first; the number breaks ties deterministically.
    let mut heap: BinaryHeap<(i32, i32)> =
        counts.into_iter().map(|(num, count)| (count, num)).collect();

    let mut result = Vec::with_capacity(k);
    for _ in 0..k {
        match heap.pop() {
            Some((_, num)) => result.push(num),
            None => break,
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sorted(mut v: Vec<i32>) -> Vec<i32> {
        v.sort_unstable();
        v
    }

    #[test]
    fn returns_top_two_frequent() {
        assert_eq!(solve(&[1, 1, 1, 2, 2, 3], 2), vec![1, 2]);
    }

    #[test]
    fn single_element() {
        assert_eq!(solve(&[1], 1), vec![1]);
    }

    #[test]
    fn all_equal_frequency() {
        assert_eq!(sorted(solve(&[1, 2, 3], 3)), vec![1, 2, 3]);
    }

    #[test]
    fn negative_numbers() {
        assert_eq!(solve(&[-1, -1, 2, 2, 2, 3], 1), vec![2]);
    }

    #[test]
    fn k_equals_distinct_count() {
        assert_eq!(sorted(solve(&[3, 0, 1, 0], 3)), vec![0, 1, 3]);
    }
}
