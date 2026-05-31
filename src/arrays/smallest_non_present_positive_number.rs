//! Smallest positive integer not present in the array.
//!
//! Drop everything into a set, then probe 1, 2, 3, ... until one is missing.

use std::collections::HashSet;

pub fn solve(a: &[i32]) -> i32 {
    let present: HashSet<i32> = a.iter().copied().collect();

    let mut candidate = 1;
    while present.contains(&candidate) {
        candidate += 1;
    }
    candidate
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn finds_first_gap() {
        assert_eq!(solve(&[1, 3, 6, 4, 1, 2]), 5);
    }

    #[test]
    fn contiguous_run_returns_next() {
        assert_eq!(solve(&[1, 2, 3]), 4);
    }

    #[test]
    fn no_positives_returns_one() {
        assert_eq!(solve(&[-1, -3, 0]), 1);
    }
}
