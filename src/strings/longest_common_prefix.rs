//! Longest common prefix shared by all strings.
//!
//! The answer cannot be longer than the shortest string, so trim that
//! candidate one character at a time until every string starts with it.

pub fn solve(strs: &[&str]) -> String {
    let Some(shortest) = strs.iter().min_by_key(|s| s.len()) else {
        return String::new();
    };

    let mut prefix = shortest.as_bytes();
    while !prefix.is_empty() {
        if strs.iter().all(|s| s.as_bytes().starts_with(prefix)) {
            return String::from_utf8(prefix.to_vec()).unwrap();
        }
        prefix = &prefix[..prefix.len() - 1];
    }

    String::new()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn shrinks_to_common_prefix() {
        assert_eq!(solve(&["hello", "hel", "h"]), "h");
        assert_eq!(solve(&["hello", "hel", "he"]), "he");
    }

    #[test]
    fn empty_string_forces_empty_prefix() {
        assert_eq!(solve(&["", "hel", "he"]), "");
    }
}
