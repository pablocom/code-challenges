//! Length of the Longest Increasing Subsequence.
//!
//! `dp[i]` is the LIS length starting at `i`; filling it right-to-left lets each
//! position extend any later, larger element. O(n²).

pub fn solve(nums: &[i32]) -> usize {
    if nums.is_empty() {
        return 0;
    }

    let n = nums.len();
    let mut dp = vec![1usize; n];

    for i in (0..n).rev() {
        for j in i + 1..n {
            if nums[i] < nums[j] {
                dp[i] = dp[i].max(1 + dp[j]);
            }
        }
    }

    dp.into_iter().max().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples() {
        assert_eq!(solve(&[]), 0);
        assert_eq!(solve(&[7]), 1);
        assert_eq!(solve(&[1, 2]), 2);
        assert_eq!(solve(&[2, 1]), 1);
        assert_eq!(solve(&[5, 4, 3, 2, 1]), 1);
        assert_eq!(solve(&[1, 2, 3, 4, 5]), 5);
        assert_eq!(solve(&[7, 7, 7, 7, 7]), 1);
        assert_eq!(solve(&[10, 9, 2, 5, 3, 7, 101, 18]), 4);
        assert_eq!(solve(&[0, 1, 0, 3, 2, 3]), 4);
        assert_eq!(solve(&[3, 10, 2, 1, 20]), 3);
        assert_eq!(solve(&[1, 3, 6, 7, 9, 4, 10, 5, 6]), 6);
        assert_eq!(solve(&[-5, -3, -1, 0, 2]), 5);
        assert_eq!(solve(&[-7, -2, -5, 0, -1, 3]), 4);
        assert_eq!(solve(&[4, 10, 4, 3, 8, 9]), 3);
        assert_eq!(solve(&[1, 2, 4, 3]), 3);
    }
}
