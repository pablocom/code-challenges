//! Two AWS interview questions.
//!
//! - [`min_swaps_required`]: adjacent-ish swaps to make a string a palindrome,
//!   or `-1` if impossible.
//! - [`get_max_freq_deviation`]: the largest difference between the lengths of
//!   two consecutive runs of equal characters.

/// Empty input returns `-1`; a single character needs no swaps.
pub fn min_swaps_required(s: &str) -> i32 {
    if s.is_empty() {
        return -1;
    }
    if s.len() == 1 {
        return 0;
    }

    let mut chars: Vec<u8> = s.bytes().collect();
    let mut swaps = 0;
    let mut left = 0;
    let mut right = chars.len() - 1;

    while left < right {
        if chars[left] == chars[right] {
            left += 1;
            right -= 1;
            continue;
        }

        // Find a character left of `right` that differs from chars[right] and
        // swap it into place to make the ends match.
        let mut scan = right - 1;
        while scan > left {
            if chars[scan] != chars[right] {
                swaps += 1;
                chars.swap(right, scan);
                break;
            }
            scan -= 1;
        }

        if chars[left] != chars[right] {
            return -1;
        }
        left += 1;
        right -= 1;
    }

    swaps
}

pub fn get_max_freq_deviation(s: &str) -> i32 {
    let bytes = s.as_bytes();
    let len = bytes.len();

    if len <= 1 || bytes.iter().all(|&c| c == bytes[0]) {
        return 0;
    }

    let mut max_deviation: i32 = 0;
    let mut previous_frequency: i32 = 1;
    let mut current_frequency: i32 = 1;
    let mut previous_char = bytes[0];

    for i in 1..=len {
        if i == len - 1 {
            max_deviation = max_deviation.max((previous_frequency - current_frequency).abs());
            break;
        }

        if previous_char == bytes[i] {
            current_frequency += 1;
        } else {
            max_deviation = max_deviation.max((previous_frequency - current_frequency).abs());
            previous_frequency = current_frequency;
            if i < len - 1 {
                current_frequency = 1;
            }
        }
        previous_char = bytes[i];
    }

    max_deviation
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn min_swaps_required_examples() {
        assert_eq!(min_swaps_required(""), -1);
        assert_eq!(min_swaps_required("a"), 0);
        assert_eq!(min_swaps_required("aa"), 0);
        assert_eq!(min_swaps_required("ab"), -1);
        assert_eq!(min_swaps_required("aba"), 0);
        assert_eq!(min_swaps_required("abba"), 0);
        assert_eq!(min_swaps_required("abab"), 1);
        assert_eq!(min_swaps_required("abcd"), -1);
        assert_eq!(min_swaps_required("aabb"), 1);
        assert_eq!(min_swaps_required("aaa"), 0);
    }

    #[test]
    fn get_max_freq_deviation_examples() {
        assert_eq!(get_max_freq_deviation("a"), 0);
        assert_eq!(get_max_freq_deviation("aaaa"), 0);
        assert_eq!(get_max_freq_deviation("aabbb"), 1);
        assert_eq!(get_max_freq_deviation("aabbbbcc"), 3);
        assert_eq!(get_max_freq_deviation("ababab"), 0);
        assert_eq!(get_max_freq_deviation("ab"), 0);
        assert_eq!(get_max_freq_deviation("aaaab"), 3);
    }
}
