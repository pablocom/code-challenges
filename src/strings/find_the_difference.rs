//! `t` is `s` shuffled with one extra character added — find that character.
//!
//! Two takes: a frequency table, and the classic XOR trick (every shared
//! character cancels, leaving only the added one).

use std::collections::HashMap;

pub fn solve_with_dictionary(s: &str, t: &str) -> char {
    let mut counts: HashMap<char, i32> = HashMap::new();
    for c in t.chars() {
        *counts.entry(c).or_insert(0) += 1;
    }

    for c in s.chars() {
        if let Some(count) = counts.get_mut(&c) {
            *count -= 1;
            if *count == 0 {
                counts.remove(&c);
            }
        }
    }

    *counts
        .keys()
        .next()
        .expect("t must contain one extra character")
}

pub fn solve_with_xor(s: &str, t: &str) -> char {
    let mut acc = 0u32;
    for c in s.chars().chain(t.chars()) {
        acc ^= c as u32;
    }
    char::from_u32(acc).expect("xor of valid chars is a valid char")
}

#[cfg(test)]
mod tests {
    use super::*;

    const SCENARIOS: &[(&str, &str, char)] = &[
        ("abcd", "abcde", 'e'),
        ("", "y", 'y'),
        ("a", "aa", 'a'),
        ("ae", "aea", 'a'),
        ("abcde", "abcdef", 'f'),
    ];

    #[test]
    fn with_dictionary() {
        for &(s, t, expected) in SCENARIOS {
            assert_eq!(solve_with_dictionary(s, t), expected);
        }
    }

    #[test]
    fn with_xor() {
        for &(s, t, expected) in SCENARIOS {
            assert_eq!(solve_with_xor(s, t), expected);
        }
    }
}
