//! 4Sum — all unique quadruplets that sum to `target`.
//!
//! Two nested anchors plus a two-pointer scan, with duplicate-skipping at
//! every level. Sums are widened to `i64` to avoid overflow.

pub fn solve(nums: &[i32], target: i32) -> Vec<Vec<i32>> {
    let mut nums = nums.to_vec();
    nums.sort_unstable();

    let n = nums.len();
    let mut result = Vec::new();
    if n < 4 {
        return result;
    }

    for k in 0..n - 3 {
        if k > 0 && nums[k] == nums[k - 1] {
            continue;
        }

        for i in k + 1..n - 2 {
            if i > k + 1 && nums[i] == nums[i - 1] {
                continue;
            }

            let to_find = target as i64 - (nums[i] as i64 + nums[k] as i64);
            let (mut lo, mut hi) = (i + 1, n - 1);

            while lo < hi {
                let sum = nums[lo] as i64 + nums[hi] as i64;
                if sum == to_find {
                    result.push(vec![nums[k], nums[i], nums[lo], nums[hi]]);
                    while lo < hi && nums[lo] == nums[lo - 1] {
                        lo += 1;
                    }
                    while lo < hi && nums[hi] == nums[hi - 1] {
                        hi -= 1;
                    }
                }

                if sum < to_find {
                    lo += 1;
                } else {
                    hi -= 1;
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
    fn example_with_zero_target() {
        let result = solve(&[1, 0, -1, 0, -2, 2], 0);
        assert_eq!(
            result,
            vec![vec![-2, -1, 1, 2], vec![-2, 0, 0, 2], vec![-1, 0, 0, 1]]
        );
    }

    #[test]
    fn all_equal_elements() {
        assert_eq!(solve(&[2, 2, 2, 2, 2], 8), vec![vec![2, 2, 2, 2]]);
    }
}
