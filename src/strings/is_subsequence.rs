//! Is `s` a subsequence of `t`? (Same order, gaps allowed.)
//!
//! Walk `t` once, advancing a cursor in `s` on every match.

pub fn solve(s: &str, t: &str) -> bool {
    if s.len() > t.len() {
        return false;
    }

    let s = s.as_bytes();
    let mut cursor = 0;

    for &c in t.as_bytes() {
        if cursor == s.len() {
            return true;
        }
        if s[cursor] == c {
            cursor += 1;
        }
    }

    cursor == s.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples() {
        assert!(solve("abc", "ahbgdc"));
        assert!(!solve("axc", "ahbgdc"));
        assert!(solve("", "ahbgdc"));
        assert!(solve("abc", "abc"));
        assert!(!solve("abc", "ab"));
        assert!(solve("a", "a"));
        assert!(!solve("a", "b"));
        assert!(solve("ace", "abcde"));
        assert!(!solve("aec", "abcde"));
        assert!(solve("abc", "aabc"));
    }
}
