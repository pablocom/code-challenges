//! Find the distinct elements present in one array but not the other.
//!
//! Returns `[only_in_first, only_in_second]`, each free of duplicates.

use std::collections::HashSet;

pub fn solve(nums1: &[i32], nums2: &[i32]) -> [Vec<i32>; 2] {
    let set1: HashSet<i32> = nums1.iter().copied().collect();
    let set2: HashSet<i32> = nums2.iter().copied().collect();

    let only_in_first = set1.difference(&set2).copied().collect();
    let only_in_second = set2.difference(&set1).copied().collect();

    [only_in_first, only_in_second]
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Set difference order is unspecified, so compare as sorted sets.
    fn sorted(mut v: Vec<i32>) -> Vec<i32> {
        v.sort_unstable();
        v
    }

    #[test]
    fn partial_overlap_returns_missing_on_each_side() {
        let [first, second] = solve(&[1, 2, 3], &[2, 4, 6]);
        assert_eq!(sorted(first), vec![1, 3]);
        assert_eq!(sorted(second), vec![4, 6]);
    }

    #[test]
    fn identical_arrays_return_two_empty_lists() {
        let [first, second] = solve(&[1, 2, 3], &[1, 2, 3]);
        assert!(first.is_empty());
        assert!(second.is_empty());
    }

    #[test]
    fn no_overlap_returns_both_arrays() {
        let [first, second] = solve(&[1, 3, 5], &[2, 4, 6]);
        assert_eq!(sorted(first), vec![1, 3, 5]);
        assert_eq!(sorted(second), vec![2, 4, 6]);
    }

    #[test]
    fn duplicates_in_input_return_only_unique_elements() {
        let [first, second] = solve(&[1, 2, 2, 3], &[2, 3, 4]);
        assert_eq!(sorted(first), vec![1]);
        assert_eq!(sorted(second), vec![4]);
    }

    #[test]
    fn subset_returns_empty_first_and_extra_in_second() {
        let [first, second] = solve(&[1, 2, 3], &[1, 2, 3, 4]);
        assert!(first.is_empty());
        assert_eq!(sorted(second), vec![4]);
    }
}
