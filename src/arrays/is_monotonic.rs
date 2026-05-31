//! Is the array monotonic (entirely non-increasing or non-decreasing)?
//!
//! Lock in a direction at the first strict change, then verify every later
//! change agrees with it.

use std::cmp::Ordering;

pub fn solve(nums: &[i32]) -> bool {
    if nums.len() <= 1 {
        return true;
    }

    let mut direction: Option<Ordering> = None;

    for window in nums.windows(2) {
        let change = window[0].cmp(&window[1]);
        if change == Ordering::Equal {
            continue;
        }

        match direction {
            None => direction = Some(change),
            Some(locked) if locked != change => return false,
            _ => {}
        }
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples() {
        assert!(solve(&[1, 2, 2, 3]));
        assert!(solve(&[6, 5, 4, 4]));
        assert!(!solve(&[1, 3, 2]));
        assert!(solve(&[1, 1, 1]));
        assert!(solve(&[1]));
        assert!(solve(&[1, 2, 3, 4, 5]));
        assert!(solve(&[5, 4, 3, 2, 1]));
        assert!(!solve(&[1, 2, 3, 2, 1]));
        assert!(solve(&[3, 3, 3, 2, 1]));
        assert!(solve(&[1, 1, 2, 3, 3]));
    }
}
