//! Longest palindromic substring, expand-around-center variant that records
//! the first widest palindrome encountered (the earliest center wins ties).

pub fn solve(s: &str) -> String {
    let bytes = s.as_bytes();
    let mut best = (0usize, 0usize); // (start, length)

    for i in 0..bytes.len() {
        for (mut l, mut r) in [(i as isize, i as isize), (i as isize, i as isize + 1)] {
            while l >= 0 && (r as usize) < bytes.len() && bytes[l as usize] == bytes[r as usize] {
                let length = (r - l + 1) as usize;
                if length > best.1 {
                    best = (l as usize, length);
                }
                l -= 1;
                r += 1;
            }
        }
    }

    s[best.0..best.0 + best.1].to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn is_palindrome(s: &str) -> bool {
        s.bytes().eq(s.bytes().rev())
    }

    #[test]
    fn single_and_pairs() {
        assert_eq!(solve("a"), "a");
        assert_eq!(solve("aa"), "aa");
        assert_eq!(solve("ab"), "a");
    }

    #[test]
    fn first_widest_wins() {
        assert_eq!(solve("babad"), "bab");
        assert_eq!(solve("cbbd"), "bb");
    }

    #[test]
    fn full_and_embedded() {
        assert_eq!(solve("racecar"), "racecar");
        assert_eq!(solve("abba"), "abba");
        assert_eq!(solve("abacaba"), "abacaba");
        assert_eq!(solve("forgeeksskeegfor"), "geeksskeeg");
        assert_eq!(solve("pqabbarst"), "abba");
        assert_eq!(solve("pqrlevelstu"), "level");
    }

    #[test]
    fn result_is_always_a_palindrome() {
        for input in [
            "a", "ab", "abc", "aab", "baa", "abba", "racecar", "banana", "million", "abcba",
            "cbbd", "babad",
        ] {
            let result = solve(input);
            assert!(!result.is_empty());
            assert!(is_palindrome(&result));
            assert!(input.contains(&result));
        }
    }
}
