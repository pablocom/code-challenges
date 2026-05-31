//! Can the string be built by repeating one of its substrings?
//!
//! Two takes: the KMP failure function (the elegant O(n) answer) and a
//! straightforward divisor scan.

pub fn solve(s: &str) -> bool {
    solve_with_kmp(s)
}

/// If the longest proper prefix that is also a suffix leaves a remainder `unit`
/// that divides the length, the string is `len / unit` copies of that unit.
pub fn solve_with_kmp(s: &str) -> bool {
    let bytes = s.as_bytes();
    let len = bytes.len();
    if len == 0 {
        return false;
    }

    let mut failure = vec![0usize; len];
    for i in 1..len {
        let mut j = failure[i - 1];
        while j > 0 && bytes[i] != bytes[j] {
            j = failure[j - 1];
        }
        if bytes[i] == bytes[j] {
            j += 1;
        }
        failure[i] = j;
    }

    let longest_prefix_suffix = failure[len - 1];
    let unit = len - longest_prefix_suffix;

    longest_prefix_suffix != 0 && len.is_multiple_of(unit)
}

/// Try every candidate repeat count that divides the length.
pub fn solve_with_divisors(s: &str) -> bool {
    let bytes = s.as_bytes();
    let n = bytes.len();

    let mut divisor = 2;
    while divisor < n + 1 {
        if !n.is_multiple_of(divisor) {
            divisor += 1;
            continue;
        }

        let unit = n / divisor;
        let candidate = &bytes[..unit];
        if (1..divisor).all(|i| bytes[unit * i..].starts_with(candidate)) {
            return true;
        }

        divisor += 1;
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;

    const SCENARIOS: &[(&str, bool)] = &[
        ("bcdbcdbcdbcd", true),
        ("abcdabcd", true),
        ("aabaaba", false),
        ("abab", true),
        ("abcabc", true),
        ("a", false),
        ("aa", true),
        ("ab", false),
        ("aaa", true),
        ("abcabcabc", true),
        ("abac", false),
    ];

    #[test]
    fn kmp() {
        for &(s, expected) in SCENARIOS {
            assert_eq!(solve(s), expected, "kmp failed for {s:?}");
        }
    }

    #[test]
    fn divisors() {
        for &(s, expected) in SCENARIOS {
            assert_eq!(
                solve_with_divisors(s),
                expected,
                "divisors failed for {s:?}"
            );
        }
    }
}
