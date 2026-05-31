//! Minimum difference after at most three moves (LeetCode 1509).
//!
//! With three changes allowed, only the four smallest and four largest values
//! matter. Sort, then take the best of the four "drop k from one end" choices.

pub fn solve(nums: &[i32]) -> i32 {
    if nums.len() < 4 {
        return 0;
    }

    let mut nums = nums.to_vec();
    nums.sort_unstable();
    let n = nums.len();

    (nums[n - 4] - nums[0])
        .min(nums[n - 3] - nums[1])
        .min(nums[n - 2] - nums[2])
        .min(nums[n - 1] - nums[3])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples() {
        assert_eq!(solve(&[5, 3, 2, 4]), 0);
        assert_eq!(solve(&[1, 5, 0, 10, 14]), 1);
        assert_eq!(solve(&[3, 100, 20]), 0);
        assert_eq!(solve(&[6, 6, 0, 1, 1, 4, 6]), 2);
        assert_eq!(solve(&[1, 5, 6, 14, 15]), 1);
    }
}
