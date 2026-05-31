//! Top K Frequent Words — most frequent first, ties broken alphabetically.

use std::collections::HashMap;

pub fn top_k_frequent(words: &[&str], k: usize) -> Vec<String> {
    let mut counts: HashMap<&str, i32> = HashMap::new();
    for &word in words {
        *counts.entry(word).or_insert(0) += 1;
    }

    let mut entries: Vec<(&str, i32)> = counts.into_iter().collect();
    // Higher count first; on a tie, lexicographically smaller word first.
    entries.sort_by(|a, b| b.1.cmp(&a.1).then_with(|| a.0.cmp(b.0)));

    entries
        .into_iter()
        .take(k)
        .map(|(word, _)| word.to_string())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn returns_most_frequent_words() {
        let words = ["i", "love", "leetcode", "i", "love", "coding"];
        assert_eq!(top_k_frequent(&words, 2), vec!["i", "love"]);
    }

    #[test]
    fn ties_broken_alphabetically() {
        let words = [
            "the", "day", "is", "sunny", "the", "the", "the", "sunny", "is", "is",
        ];
        assert_eq!(
            top_k_frequent(&words, 4),
            vec!["the", "is", "sunny", "day"]
        );
    }

    #[test]
    fn single_word() {
        assert_eq!(top_k_frequent(&["hello"], 1), vec!["hello"]);
    }

    #[test]
    fn all_same_frequency_returned_alphabetically() {
        assert_eq!(top_k_frequent(&["c", "a", "b"], 2), vec!["a", "b"]);
    }

    #[test]
    fn k_equals_distinct_word_count() {
        assert_eq!(top_k_frequent(&["aa", "bb", "aa"], 2), vec!["aa", "bb"]);
    }
}
