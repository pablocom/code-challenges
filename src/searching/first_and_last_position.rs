//! First and Last Position of a target in a sorted array (LeetCode 34).
//!
//! Two binary searches: the lower bound (first index `>= target`) and the upper
//! bound (first index `> target`). Returns `[-1, -1]` when the target is absent.

pub fn search_range(nums: &[i32], target: i32) -> [i32; 2] {
    let first = lower_bound(nums, target);
    if first == nums.len() || nums[first] != target {
        return [-1, -1];
    }
    let last = upper_bound(nums, target) - 1;
    [first as i32, last as i32]
}

/// First index whose value is `>= target`.
fn lower_bound(nums: &[i32], target: i32) -> usize {
    let (mut low, mut high) = (0, nums.len());
    while low < high {
        let mid = low + (high - low) / 2;
        if nums[mid] < target {
            low = mid + 1;
        } else {
            high = mid;
        }
    }
    low
}

/// First index whose value is `> target`.
fn upper_bound(nums: &[i32], target: i32) -> usize {
    let (mut low, mut high) = (0, nums.len());
    while low < high {
        let mid = low + (high - low) / 2;
        if nums[mid] <= target {
            low = mid + 1;
        } else {
            high = mid;
        }
    }
    low
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples() {
        assert_eq!(search_range(&[5, 7, 7, 8, 8, 10], 6), [-1, -1]);
        assert_eq!(search_range(&[5, 7, 7, 8, 8, 10], 8), [3, 4]);
        assert_eq!(search_range(&[1], 1), [0, 0]);
    }
}
