//! Group words that are anagrams of one another.
//!
//! Two anagrams share the same multiset of letters, so a sorted-letters key
//! buckets them together.

use std::collections::HashMap;

pub fn group_anagrams(strs: &[&str]) -> Vec<Vec<String>> {
    let mut groups: HashMap<Vec<u8>, Vec<String>> = HashMap::new();

    for &s in strs {
        let mut key = s.as_bytes().to_vec();
        key.sort_unstable();
        groups.entry(key).or_default().push(s.to_string());
    }

    groups.into_values().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Group order and within-group order are unspecified, so normalize both.
    fn normalize(mut groups: Vec<Vec<String>>) -> Vec<String> {
        let mut joined: Vec<String> = groups
            .iter_mut()
            .map(|g| {
                g.sort();
                g.join(",")
            })
            .collect();
        joined.sort();
        joined
    }

    #[test]
    fn groups_anagrams_together() {
        let result = group_anagrams(&["eat", "tea", "tan", "ate", "nat", "bat"]);
        let expected = vec![
            vec!["bat".to_string()],
            vec!["nat".to_string(), "tan".to_string()],
            vec!["ate".to_string(), "eat".to_string(), "tea".to_string()],
        ];
        assert_eq!(normalize(result), normalize(expected));
    }
}
