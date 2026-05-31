//! Count, for each element, how many elements are strictly smaller.
//!
//! Sort a copy; the first index at which a value appears equals the number of
//! strictly-smaller values. A lookup table then answers each query in O(1).

use std::collections::HashMap;

pub fn solve(nums: &[i32]) -> Vec<usize> {
    let mut sorted = nums.to_vec();
    sorted.sort_unstable();

    let mut lower_count_by_number: HashMap<i32, usize> = HashMap::new();
    for (i, &value) in sorted.iter().enumerate() {
        lower_count_by_number.entry(value).or_insert(i);
    }

    nums.iter().map(|n| lower_count_by_number[n]).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples() {
        assert_eq!(solve(&[8, 1, 2, 2, 3]), vec![4, 0, 1, 1, 3]);
        assert_eq!(solve(&[6, 5, 4, 8]), vec![2, 1, 0, 3]);
        assert_eq!(solve(&[7, 7, 7, 7]), vec![0, 0, 0, 0]);
        assert_eq!(solve(&[0]), vec![0]);
        assert_eq!(solve(&[5, 0, 10]), vec![1, 0, 2]);
    }
}
