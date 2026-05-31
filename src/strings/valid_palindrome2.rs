//! Valid Palindrome II — is it a palindrome after deleting at most one char?
//!
//! Two pointers from the ends. On the first mismatch, try skipping the left
//! character or the right one; the remainder must be a clean palindrome.

pub fn solve(text: &str) -> bool {
    let bytes = text.as_bytes();
    if bytes.is_empty() {
        return true;
    }

    let (mut left, mut right) = (0usize, bytes.len() - 1);
    while left < right {
        if bytes[left] == bytes[right] {
            left += 1;
            right -= 1;
        } else {
            return is_palindrome(bytes, left + 1, right) || is_palindrome(bytes, left, right - 1);
        }
    }

    true
}

fn is_palindrome(bytes: &[u8], mut left: usize, mut right: usize) -> bool {
    while left < right {
        if bytes[left] != bytes[right] {
            return false;
        }
        left += 1;
        right -= 1;
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples() {
        assert!(solve("aba"));
        assert!(solve("abac"));
        assert!(!solve("acbac"));
        assert!(solve("cacbac"));
        assert!(!solve("abc"));
    }
}
