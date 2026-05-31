//! Length of the longest substring without repeating characters.
//!
//! A sliding window holds only distinct bytes: when a duplicate arrives, shrink
//! from the left until it is gone. O(n).

use std::collections::HashSet;

pub fn length_of_longest_substring(s: &str) -> usize {
    let bytes = s.as_bytes();
    let mut window: HashSet<u8> = HashSet::new();
    let mut left = 0;
    let mut max_length = 0;

    for right in 0..bytes.len() {
        while window.contains(&bytes[right]) {
            window.remove(&bytes[left]);
            left += 1;
        }
        window.insert(bytes[right]);
        max_length = max_length.max(right - left + 1);
    }

    max_length
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples() {
        assert_eq!(length_of_longest_substring(" "), 1);
        assert_eq!(length_of_longest_substring("dvdf"), 3);
        assert_eq!(length_of_longest_substring("abcabcbb"), 3);
        assert_eq!(length_of_longest_substring("bbbbb"), 1);
        assert_eq!(length_of_longest_substring("cdd"), 2);
    }
}
