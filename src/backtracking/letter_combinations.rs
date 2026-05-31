//! Letter combinations of a phone number — the Cartesian product of each
//! digit's letters, in digit order.

/// Letters for digits 2–9, indexable as `LETTERS[digit - 2]`.
const LETTERS: [&str; 8] = ["abc", "def", "ghi", "jkl", "mno", "pqrs", "tuv", "wxyz"];

pub fn solve(digits: &str) -> Vec<String> {
    if digits.is_empty() {
        return Vec::new();
    }

    let mut result = Vec::new();
    let mut current = String::new();
    combine(digits.as_bytes(), 0, &mut current, &mut result);
    result
}

fn combine(digits: &[u8], position: usize, current: &mut String, result: &mut Vec<String>) {
    if position == digits.len() {
        result.push(current.clone());
        return;
    }

    let group = LETTERS[(digits[position] - b'2') as usize];
    for letter in group.chars() {
        current.push(letter);
        combine(digits, position + 1, current, result);
        current.pop();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_input() {
        assert!(solve("").is_empty());
    }

    #[test]
    fn single_digit() {
        assert_eq!(solve("2"), vec!["a", "b", "c"]);
    }

    #[test]
    fn two_digits() {
        assert_eq!(
            solve("23"),
            vec!["ad", "ae", "af", "bd", "be", "bf", "cd", "ce", "cf"]
        );
    }

    #[test]
    fn three_digits() {
        assert_eq!(
            solve("234"),
            vec![
                "adg", "adh", "adi", "aeg", "aeh", "aei", "afg", "afh", "afi", "bdg", "bdh", "bdi",
                "beg", "beh", "bei", "bfg", "bfh", "bfi", "cdg", "cdh", "cdi", "ceg", "ceh", "cei",
                "cfg", "cfh", "cfi"
            ]
        );
    }
}
