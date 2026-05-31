//! 3Sum — all unique triplets that sum to zero.
//!
//! Sort, then for each anchor run a two-pointer scan, skipping duplicates so
//! every triplet is reported exactly once. O(n^2) time.

use std::cmp::Ordering;

pub fn solve(nums: &[i32]) -> Vec<Vec<i32>> {
    if nums.len() < 3 {
        return Vec::new();
    }

    let mut nums = nums.to_vec();
    nums.sort_unstable();

    let n = nums.len();
    let mut result = Vec::new();

    for i in 0..n - 2 {
        if i > 0 && nums[i] == nums[i - 1] {
            continue;
        }

        let (mut lo, mut hi) = (i + 1, n - 1);
        while lo < hi {
            match (nums[i] + nums[lo] + nums[hi]).cmp(&0) {
                Ordering::Greater => hi -= 1,
                Ordering::Less => lo += 1,
                Ordering::Equal => {
                    result.push(vec![nums[i], nums[lo], nums[hi]]);
                    lo += 1;
                    while lo < hi && nums[lo] == nums[lo - 1] {
                        lo += 1;
                    }
                }
            }
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn finds_unique_triplets() {
        let result = solve(&[-1, 0, 1, 2, -1, -4]);
        assert_eq!(result, vec![vec![-1, -1, 2], vec![-1, 0, 1]]);
    }

    #[test]
    fn fewer_than_three_elements_yields_nothing() {
        assert!(solve(&[1, 2]).is_empty());
    }

    #[test]
    fn no_triplet_sums_to_zero() {
        assert!(solve(&[1, 2, 3, 4]).is_empty());
    }
}
