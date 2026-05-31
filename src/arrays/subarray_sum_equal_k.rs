//! Count contiguous subarrays summing to `k`.
//!
//! Running prefix sums: a subarray `(i, j]` sums to `k` exactly when
//! `prefix[j] - prefix[i] == k`. Counting previously seen prefix sums lets us
//! answer in one O(n) pass.

use std::collections::HashMap;

pub fn solve(nums: &[i32], k: i32) -> i32 {
    let mut count_by_prefix: HashMap<i32, i32> = HashMap::from([(0, 1)]);
    let mut result = 0;
    let mut running_sum = 0;

    for &n in nums {
        running_sum += n;
        result += count_by_prefix.get(&(running_sum - k)).copied().unwrap_or(0);
        *count_by_prefix.entry(running_sum).or_insert(0) += 1;
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples() {
        assert_eq!(solve(&[1, 1, 1], 2), 2);
        assert_eq!(solve(&[1, 2, 3], 3), 2);
        assert_eq!(solve(&[1], 1), 1);
        assert_eq!(solve(&[1], 2), 0);
        assert_eq!(solve(&[-1, -1, 1], -1), 3);
    }
}
