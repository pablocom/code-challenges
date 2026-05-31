//! Sliding Window Maximum — the maximum of every length-`k` window.
//!
//! A monotonic decreasing deque of indices keeps the current window's maximum
//! at its front. Each index is pushed and popped at most once, so O(n) overall.

use std::collections::VecDeque;

pub fn solve(nums: &[i32], k: usize) -> Vec<i32> {
    let mut result = Vec::with_capacity(nums.len() + 1 - k);
    let mut candidates: VecDeque<usize> = VecDeque::new();
    let mut left = 0usize;

    for right in 0..nums.len() {
        while candidates.back().is_some_and(|&i| nums[i] < nums[right]) {
            candidates.pop_back();
        }
        candidates.push_back(right);

        if left > candidates[0] {
            candidates.pop_front();
        }

        if right + 1 >= k {
            result.push(nums[candidates[0]]);
            left += 1;
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples() {
        assert_eq!(
            solve(&[1, 3, -1, -3, 5, 3, 6, 7], 3),
            vec![3, 3, 5, 5, 6, 7]
        );
        assert_eq!(solve(&[1, 2, 3], 3), vec![3]);
        assert_eq!(solve(&[1, -1, 3], 1), vec![1, -1, 3]);
        assert_eq!(solve(&[5, 5, 5, 5], 2), vec![5, 5, 5]);
        assert_eq!(solve(&[5, 4, 3, 2, 1], 3), vec![5, 4, 3]);
        assert_eq!(solve(&[1, 2, 3, 4, 5], 3), vec![3, 4, 5]);
    }
}
