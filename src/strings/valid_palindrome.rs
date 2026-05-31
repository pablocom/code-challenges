//! Valid Palindrome — ignoring non-alphanumerics and letter case.

pub fn solve(word: &str) -> bool {
    let bytes = word.as_bytes();
    let (mut left, mut right) = (0usize, bytes.len());

    while left < right {
        let l = bytes[left];
        if !l.is_ascii_alphanumeric() {
            left += 1;
            continue;
        }

        let r = bytes[right - 1];
        if !r.is_ascii_alphanumeric() {
            right -= 1;
            continue;
        }

        if !l.eq_ignore_ascii_case(&r) {
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
        assert!(solve("bla bla 12 _ 21 alb alb"));
        assert!(!solve("bla bla 1 _ 1 zlb alb"));
        assert!(solve("323_!"));
        assert!(!solve("323_!a"));
        assert!(!solve("123_!ab"));
        assert!(solve(""));
        assert!(!solve("0P"));
    }
}
