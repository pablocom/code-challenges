//! Longest subarray where `max - min <= limit`.
//!
//! A sliding window with two monotonic deques of indices: one decreasing (the
//! window max at its front) and one increasing (the window min at its front).
//! Each index enters and leaves a deque at most once, so the scan is O(n).

use std::collections::VecDeque;

pub fn solve(nums: &[i32], limit: i32) -> usize {
    let mut max_candidates: VecDeque<usize> = VecDeque::new();
    let mut min_candidates: VecDeque<usize> = VecDeque::new();

    let mut window_start = 0usize;
    let mut longest = 0usize;

    for window_end in 0..nums.len() {
        while max_candidates
            .back()
            .is_some_and(|&i| nums[i] <= nums[window_end])
        {
            max_candidates.pop_back();
        }
        while min_candidates
            .back()
            .is_some_and(|&i| nums[i] >= nums[window_end])
        {
            min_candidates.pop_back();
        }

        max_candidates.push_back(window_end);
        min_candidates.push_back(window_end);

        while nums[max_candidates[0]] - nums[min_candidates[0]] > limit {
            window_start += 1;
            if max_candidates[0] < window_start {
                max_candidates.pop_front();
            }
            if min_candidates[0] < window_start {
                min_candidates.pop_front();
            }
        }

        longest = longest.max(window_end - window_start + 1);
    }

    longest
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples() {
        assert_eq!(solve(&[8, 2, 4, 7], 4), 2);
        assert_eq!(solve(&[10, 1, 2, 4, 7, 2], 5), 4);
        assert_eq!(solve(&[4, 2, 2, 2, 4, 4, 2, 2], 0), 3);
    }
}
