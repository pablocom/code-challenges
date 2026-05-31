//! Find-and-replace with simultaneous, index-anchored substitutions.
//!
//! Each operation only applies if `source[k]` actually appears at `index[k]`.
//! All replacements refer to positions in the *original* string, so we first
//! record which start positions fire, then rebuild the output left to right.

use std::collections::HashMap;

pub fn find_replace_string(
    text: &str,
    indexes: &[usize],
    sources: &[&str],
    targets: &[&str],
) -> String {
    let text = text.as_bytes();

    let mut replacement_at: HashMap<usize, usize> = HashMap::new();
    for (op, &start) in indexes.iter().enumerate() {
        if start <= text.len() && text[start..].starts_with(sources[op].as_bytes()) {
            replacement_at.insert(start, op);
        }
    }

    let mut result = String::new();
    let mut i = 0;
    while i < text.len() {
        if let Some(&op) = replacement_at.get(&i) {
            result.push_str(targets[op]);
            i += sources[op].len();
        } else {
            result.push(text[i] as char);
            i += 1;
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn both_replacements_apply() {
        let result = find_replace_string("abcd", &[0, 2], &["a", "cd"], &["eee", "ffff"]);
        assert_eq!(result, "eeebffff");
    }

    #[test]
    fn non_matching_source_is_skipped() {
        let result = find_replace_string("abcd", &[0, 2], &["ab", "ec"], &["eee", "ffff"]);
        assert_eq!(result, "eeecd");
    }

    #[test]
    fn operations_out_of_order() {
        let result = find_replace_string("abcd", &[2, 0], &["cd", "a"], &["ffff", "eee"]);
        assert_eq!(result, "eeebffff");
    }
}
