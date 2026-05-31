//! Merge two strings by alternating characters; the longer tail is appended.

pub fn solve(word1: &str, word2: &str) -> String {
    let (a, b) = (word1.as_bytes(), word2.as_bytes());
    let mut result = String::with_capacity(a.len() + b.len());

    for i in 0..a.len().max(b.len()) {
        if let Some(&c) = a.get(i) {
            result.push(c as char);
        }
        if let Some(&c) = b.get(i) {
            result.push(c as char);
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples() {
        assert_eq!(solve("abc", "pqr"), "apbqcr");
        assert_eq!(solve("ab", "pqrs"), "apbqrs");
        assert_eq!(solve("abcd", "pq"), "apbqcd");
        assert_eq!(solve("a", "b"), "ab");
        assert_eq!(solve("a", "bcd"), "abcd");
        assert_eq!(solve("", "bcd"), "bcd");
    }
}
