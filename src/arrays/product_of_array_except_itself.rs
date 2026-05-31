//! Product of Array Except Self — without division.
//!
//! A left-to-right prefix pass followed by a right-to-left suffix pass. Each
//! output slot ends up holding "product of everything to the left" times
//! "product of everything to the right". O(n) time, O(1) extra space beyond
//! the output.

pub fn product_except_self(nums: &[i32]) -> Vec<i32> {
    let n = nums.len();
    let mut result = vec![1; n];

    let mut prefix = 1;
    for i in 0..n {
        result[i] = prefix;
        prefix *= nums[i];
    }

    let mut suffix = 1;
    for i in (0..n).rev() {
        result[i] *= suffix;
        suffix *= nums[i];
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(product_except_self(&[1, 2, 3, 4]), vec![24, 12, 8, 6]);
    }

    #[test]
    fn handles_zero() {
        assert_eq!(product_except_self(&[0, 4, 0]), vec![0, 0, 0]);
        assert_eq!(product_except_self(&[1, 0, 3, 4]), vec![0, 12, 0, 0]);
    }
}
