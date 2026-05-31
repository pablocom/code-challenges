//! Longest palindromic substring via expand-around-center.
//!
//! For each index, expand outward treating it as both an odd center and the
//! left of an even center, keeping the widest palindrome found. O(n²) time,
//! O(1) extra space.

pub fn longest_palindrome(text: &str) -> String {
    if text.is_empty() {
        return String::new();
    }

    let bytes = text.as_bytes();
    let (mut start, mut end) = (0usize, 0usize);

    for i in 0..bytes.len() {
        let length = expand(bytes, i, i).max(expand(bytes, i, i + 1));
        if length > end - start {
            start = i - (length - 1) / 2;
            end = i + length / 2;
        }
    }

    text[start..=end].to_string()
}

/// Palindrome length obtained by expanding from the `(left, right)` center.
fn expand(bytes: &[u8], left: usize, right: usize) -> usize {
    let mut l = left as isize;
    let mut r = right as isize;
    while l >= 0 && (r as usize) < bytes.len() && bytes[l as usize] == bytes[r as usize] {
        l -= 1;
        r += 1;
    }
    (r - l - 1) as usize
}

#[cfg(test)]
mod tests {
    use super::*;

    fn is_palindrome(s: &str) -> bool {
        s.bytes().eq(s.bytes().rev())
    }

    #[test]
    fn single_character() {
        assert_eq!(longest_palindrome("a"), "a");
    }

    #[test]
    fn odd_length_palindrome() {
        let result = longest_palindrome("babad");
        assert_eq!(result.len(), 3);
        assert!(is_palindrome(&result));
        assert!("babad".contains(&result));
    }

    #[test]
    fn even_length_palindrome() {
        assert_eq!(longest_palindrome("abb"), "bb");
    }

    #[test]
    fn full_string_palindromes() {
        assert_eq!(longest_palindrome("racecar"), "racecar");
        assert_eq!(longest_palindrome("abba"), "abba");
    }
}
