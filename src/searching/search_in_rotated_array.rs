//! Search in a rotated sorted array (LeetCode 33).
//!
//! Each midpoint splits the array into one sorted half and one rotated half.
//! Determine which half is sorted, and whether the target lies within it, to
//! decide which way to shrink the window. Returns the index, or `-1`.

pub fn search(nums: &[i32], target: i32) -> i32 {
    if nums.is_empty() {
        return -1;
    }

    let (mut left, mut right) = (0i32, nums.len() as i32 - 1);
    while left <= right {
        let mid = left + (right - left) / 2;
        let mid_value = nums[mid as usize];

        if mid_value == target {
            return mid;
        }

        if nums[left as usize] <= mid_value {
            // Left half is sorted.
            if nums[left as usize] <= target && target < mid_value {
                right = mid - 1;
            } else {
                left = mid + 1;
            }
        } else {
            // Right half is sorted.
            if mid_value < target && target <= nums[right as usize] {
                left = mid + 1;
            } else {
                right = mid - 1;
            }
        }
    }

    -1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn finds_target_in_rotated_array() {
        assert_eq!(search(&[3, 4, 0, 1, 2], 1), 3);
    }

    #[test]
    fn target_absent_returns_minus_one() {
        assert_eq!(search(&[4, 5, 6, 7, 0, 1, 2], 3), -1);
        assert_eq!(search(&[], 1), -1);
    }

    #[test]
    fn finds_first_element() {
        assert_eq!(search(&[4, 5, 6, 7, 0, 1, 2], 4), 0);
        assert_eq!(search(&[4, 5, 6, 7, 0, 1, 2], 0), 4);
    }
}
