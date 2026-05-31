//! Count-and-Say — each term is the run-length encoding, read aloud, of the
//! previous term. Term 1 is "1"; term 2 is "one 1" → "11"; and so on.

pub fn solve(n: u32) -> String {
    assert!((1..=30).contains(&n), "n must be in 1..=30");

    if n == 1 {
        return "1".to_string();
    }

    run_length_encode(&solve(n - 1))
}

fn run_length_encode(term: &str) -> String {
    let bytes = term.as_bytes();
    let mut result = String::new();

    let mut previous = bytes[0];
    let mut occurrences = 1u32;
    for &digit in &bytes[1..] {
        if digit == previous {
            occurrences += 1;
        } else {
            result.push_str(&occurrences.to_string());
            result.push(previous as char);
            occurrences = 1;
        }
        previous = digit;
    }
    result.push_str(&occurrences.to_string());
    result.push(previous as char);

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first_six_terms() {
        assert_eq!(solve(1), "1");
        assert_eq!(solve(2), "11");
        assert_eq!(solve(3), "21");
        assert_eq!(solve(4), "1211");
        assert_eq!(solve(5), "111221");
        assert_eq!(solve(6), "312211");
    }
}
