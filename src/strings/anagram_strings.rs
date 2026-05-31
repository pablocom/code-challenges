//! Are two lowercase strings anagrams of each other?
//!
//! One pass over a 26-slot tally: `+1` for each letter of `s`, `-1` for each
//! letter of `t`. They are anagrams iff every slot returns to zero.

pub fn is_anagram(s: &str, t: &str) -> bool {
    if s.len() != t.len() {
        return false;
    }

    let mut counts = [0i32; 26];
    for (&a, &b) in s.as_bytes().iter().zip(t.as_bytes()) {
        counts[(a - b'a') as usize] += 1;
        counts[(b - b'a') as usize] -= 1;
    }

    counts.iter().all(|&c| c == 0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn anagrams_are_detected() {
        assert!(is_anagram("pablo", "blpao"));
    }

    #[test]
    fn non_anagrams_are_rejected() {
        assert!(!is_anagram("rat", "car"));
        assert!(!is_anagram("ab", "abc"));
    }
}
