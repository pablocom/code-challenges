//! Does any pair of elements add up to a given sum?
//!
//! Two flavours: a two-pointer scan that assumes ascending input, and a
//! hash-set pass for unordered input.

use std::cmp::Ordering;
use std::collections::HashSet;

/// Assumes `numbers` is sorted ascending. O(n) time, O(1) space.
pub fn solve_for_asc_ordered(numbers: &[i32], sum: i32) -> bool {
    if numbers.is_empty() {
        return false;
    }

    let (mut low, mut high) = (0usize, numbers.len() - 1);
    while low < high {
        match (numbers[low] + numbers[high]).cmp(&sum) {
            Ordering::Equal => return true,
            Ordering::Less => low += 1,
            Ordering::Greater => high -= 1,
        }
    }

    false
}

/// Works on unordered input. O(n) time, O(n) space.
pub fn solve_for_unordered(numbers: &[i32], sum: i32) -> bool {
    let mut complements_seen = HashSet::new();

    for &number in numbers {
        if complements_seen.contains(&number) {
            return true;
        }
        complements_seen.insert(sum - number);
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ascending_ordered() {
        assert!(!solve_for_asc_ordered(&[1, 2, 3, 9], 8));
        assert!(solve_for_asc_ordered(&[1, 2, 4, 4], 8));
        assert!(!solve_for_asc_ordered(&[1, 2, 3], 6));
        assert!(solve_for_asc_ordered(&[-2, -1, 0, 3, 7], -1));
        assert!(!solve_for_asc_ordered(&[-2, -1, 0, 3, 7], 0));
    }

    #[test]
    fn unordered() {
        assert!(solve_for_unordered(&[7, 1, 6, 7], 7));
        assert!(solve_for_unordered(&[7, 1, -6, -7], -13));
        assert!(!solve_for_unordered(&[7, 1, -6, -7], -17));
    }
}
