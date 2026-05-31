//! Starting indices where `s` contains a concatenation of every word in
//! `words` (each used exactly once, any order). All words share one length.
//!
//! For each of the `word_length` alignments, run a sliding window measured in
//! whole words, shrinking from the left whenever a word appears too often.

use std::collections::HashMap;

pub fn find_substring(s: &str, words: &[&str]) -> Vec<usize> {
    if words.is_empty() {
        return Vec::new();
    }

    let word_length = words[0].len();
    let total_length = word_length * words.len();
    if s.len() < total_length {
        return Vec::new();
    }

    let mut needed: HashMap<&str, i32> = HashMap::new();
    for &w in words {
        *needed.entry(w).or_insert(0) += 1;
    }

    let mut result = Vec::new();

    for offset in 0..word_length {
        let mut window_start = offset;
        let mut found: HashMap<&str, i32> = HashMap::new();
        let mut count = 0usize;

        let mut right = offset;
        while right + word_length <= s.len() {
            let word = &s[right..right + word_length];

            if let Some(&need) = needed.get(word) {
                *found.entry(word).or_insert(0) += 1;
                count += 1;

                while found[word] > need {
                    let left_word = &s[window_start..window_start + word_length];
                    *found.get_mut(left_word).unwrap() -= 1;
                    count -= 1;
                    window_start += word_length;
                }

                if count == words.len() {
                    result.push(window_start);
                }
            } else {
                found.clear();
                count = 0;
                window_start = right + word_length;
            }

            right += word_length;
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sorted(mut v: Vec<usize>) -> Vec<usize> {
        v.sort_unstable();
        v
    }

    #[test]
    fn examples() {
        assert_eq!(
            find_substring("barfoothefoobarman", &["foo", "bar"]),
            vec![0, 9]
        );
        assert!(
            find_substring(
                "wordgoodgoodgoodbestword",
                &["word", "good", "best", "word"]
            )
            .is_empty()
        );
        assert_eq!(
            find_substring("barfoofoobarthefoobarman", &["bar", "foo", "the"]),
            vec![6, 9, 12]
        );
        assert_eq!(find_substring("abaa", &["a", "b"]), vec![0, 1]);
        assert_eq!(find_substring("aaaa", &["a", "a"]), vec![0, 1, 2]);
        assert_eq!(
            find_substring(
                "wordgoodgoodgoodbestword",
                &["word", "good", "best", "good"]
            ),
            vec![8]
        );
        assert_eq!(
            find_substring(
                "bcabbcaabbccacacbabccacaababcbb",
                &["c", "b", "a", "c", "a", "a", "a", "b", "c"]
            ),
            vec![6, 16, 17, 18, 19, 20]
        );
    }

    #[test]
    fn overlapping_matches_unordered() {
        assert_eq!(
            sorted(find_substring("aaaaaaaaaaaaaa", &["aa", "aa"])),
            vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10]
        );
    }
}
