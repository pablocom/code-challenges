//! Kth largest element in an array (1-indexed `k`).
//!
//! Two takes: a full sort, and a max-heap from which we pop `k` times. Rust's
//! [`BinaryHeap`] is already a max-heap, so it stands in for the bespoke heap
//! the C# version used.

use std::collections::BinaryHeap;

/// O(n log n): sort and index from the end.
pub fn solve_with_sort(nums: &[i32], k: usize) -> i32 {
    let mut nums = nums.to_vec();
    nums.sort_unstable();
    nums[nums.len() - k]
}

/// O(n + k log n): heapify, then pop `k` times.
pub fn solve_with_max_heap(nums: &[i32], k: usize) -> i32 {
    let mut heap: BinaryHeap<i32> = nums.iter().copied().collect();
    for _ in 1..k {
        heap.pop();
    }
    heap.pop().expect("k must be within bounds")
}

#[cfg(test)]
mod tests {
    use super::*;

    const SCENARIOS: &[(&[i32], usize, i32)] = &[
        (&[3, 2, 1, 4, 5, 6], 2, 5),
        (&[3, 2, 3, 1, 2, 4, 5, 5, 6], 4, 4),
        (&[1], 1, 1),
        (&[7, 7, 7], 2, 7),
        (&[5, 3, 1, 6, 4, 2], 6, 1),
    ];

    #[test]
    fn with_sort() {
        for &(nums, k, expected) in SCENARIOS {
            assert_eq!(solve_with_sort(nums, k), expected);
        }
    }

    #[test]
    fn with_max_heap() {
        for &(nums, k, expected) in SCENARIOS {
            assert_eq!(solve_with_max_heap(nums, k), expected);
        }
    }
}
