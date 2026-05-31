//! Remove duplicate letters so the result is the lexicographically smallest
//! string containing each distinct letter exactly once (LeetCode 316).
//!
//! Greedy with a monotonic stack: pop a larger letter off the top whenever it
//! still appears later in the string, so a smaller letter can take its place.

pub fn remove_duplicate_letters(text: &str) -> String {
    if text.len() <= 1 {
        return text.to_string();
    }

    let bytes = text.as_bytes();

    let mut last_index = [0usize; 26];
    for (i, &c) in bytes.iter().enumerate() {
        last_index[(c - b'a') as usize] = i;
    }

    let mut stack: Vec<u8> = Vec::new();
    let mut in_stack = [false; 26];

    for (i, &c) in bytes.iter().enumerate() {
        if in_stack[(c - b'a') as usize] {
            continue;
        }

        while let Some(&top) = stack.last() {
            if top > c && i < last_index[(top - b'a') as usize] {
                in_stack[(top - b'a') as usize] = false;
                stack.pop();
            } else {
                break;
            }
        }

        in_stack[(c - b'a') as usize] = true;
        stack.push(c);
    }

    String::from_utf8(stack).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples() {
        assert_eq!(remove_duplicate_letters("bcabc"), "abc");
        assert_eq!(remove_duplicate_letters("cbacdcbc"), "acdb");
        assert_eq!(remove_duplicate_letters("cdadabcc"), "adbc");
        assert_eq!(remove_duplicate_letters("leetcode"), "letcod");
    }
}
