//! Reorganize a string so no two adjacent characters are equal.
//!
//! If any character occurs more than `ceil(n/2)` times it is impossible
//! (returns `""`). Otherwise greedily place the most frequent remaining
//! character, holding the just-used one aside for one round so it cannot repeat.

use std::collections::{BinaryHeap, HashMap};

pub fn solve(s: &str) -> String {
    if s.len() <= 1 {
        return s.to_string();
    }

    let threshold = s.len().div_ceil(2);

    let mut counts: HashMap<u8, i32> = HashMap::new();
    for &c in s.as_bytes() {
        let entry = counts.entry(c).or_insert(0);
        *entry += 1;
        if *entry as usize > threshold {
            return String::new();
        }
    }

    // Max-heap on count; the character byte breaks ties deterministically.
    let mut heap: BinaryHeap<(i32, u8)> = counts.into_iter().map(|(c, n)| (n, c)).collect();

    let mut result = Vec::with_capacity(s.len());
    let mut held_back: Option<(i32, u8)> = None;

    while let Some((count, ch)) = heap.pop() {
        result.push(ch);

        if let Some(prev) = held_back.take()
            && prev.0 > 0
        {
            heap.push(prev);
        }

        let remaining = count - 1;
        if remaining > 0 {
            held_back = Some((remaining, ch));
        }
    }

    String::from_utf8(result).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples() {
        assert_eq!(solve("aab"), "aba");
        assert_eq!(solve("aaab"), "");
        assert_eq!(solve("aaabbcc"), "acbacba");
        assert_eq!(solve("aaaaaabbcc"), "");
    }
}
