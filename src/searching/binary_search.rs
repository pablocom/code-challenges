//! Generic binary search over a sorted slice, returning the index of the first
//! occurrence of a value (or `None`).
//!
//! On a match it keeps searching the left half so duplicates resolve to the
//! earliest index.

use std::cmp::Ordering;

pub fn find_first<T: Ord>(ordered: &[T], value: &T) -> Option<usize> {
    let mut low = 0;
    let mut high = ordered.len();
    let mut found = None;

    while low < high {
        let mid = low + (high - low) / 2;
        match ordered[mid].cmp(value) {
            Ordering::Less => low = mid + 1,
            Ordering::Greater => high = mid,
            Ordering::Equal => {
                found = Some(mid);
                high = mid;
            }
        }
    }

    found
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn finds_existing_values() {
        assert_eq!(find_first(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10], &8), Some(7));
        assert_eq!(find_first(&[1, 2, 3, 4, 5, 6, 7, 8, 9], &7), Some(6));
    }

    #[test]
    fn missing_value_returns_none() {
        assert_eq!(find_first(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10], &-11), None);
    }

    #[test]
    fn returns_first_of_duplicates() {
        assert_eq!(find_first(&[1, 2, 2, 2, 3], &2), Some(1));
    }
}
