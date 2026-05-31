//! Maximum Subarray Sum (Kadane's algorithm).
//!
//! Track the best sum ending here; whenever the running sum dips below zero it
//! cannot help any later subarray, so reset it.

pub fn solve(nums: &[i32]) -> i32 {
    let mut max_sum = i32::MIN;
    let mut current = 0;

    for &num in nums {
        current += num;
        max_sum = max_sum.max(current);
        if current < 0 {
            current = 0;
        }
    }

    max_sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples() {
        assert_eq!(solve(&[5]), 5);
        assert_eq!(solve(&[-1]), -1);
        assert_eq!(solve(&[0]), 0);
        assert_eq!(solve(&[1, 2, 3]), 6);
        assert_eq!(solve(&[-3, -2, -1]), -1);
        assert_eq!(solve(&[-5, -1, -3]), -1);
        assert_eq!(solve(&[-2, 1, -3, 4, -1, 2, 1, -5, 4]), 6);
        assert_eq!(solve(&[-2, -1, 3]), 3);
        assert_eq!(solve(&[-1, 3, -1, 3, -1]), 5);
        assert_eq!(solve(&[-5, -2, 3, 4]), 7);
        assert_eq!(solve(&[3, 4, -5, -2]), 7);
        assert_eq!(solve(&[-10000, 10000]), 10000);
        assert_eq!(solve(&[5, -100, 5]), 5);
        assert_eq!(solve(&[5, -1, 5]), 9);
        assert_eq!(solve(&[3, -10, 3]), 3);
        assert_eq!(solve(&[-10000, -10000]), -10000);
    }
}
