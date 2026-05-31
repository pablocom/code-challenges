//! Length of the longest valid (well-formed) parentheses substring.
//!
//! `dp[i]` is the length of the longest valid substring ending at index `i`.
//! A closing bracket either matches the char just before it, or jumps back over
//! an already-valid run to find its partner.

pub fn solve(s: &str) -> usize {
    let span = s.as_bytes();
    let n = span.len();
    let mut dp = vec![0usize; n];
    let mut max_len = 0;

    for i in 1..n {
        if span[i] != b')' {
            continue;
        }

        if span[i - 1] == b'(' {
            dp[i] = if i >= 2 { dp[i - 2] } else { 0 } + 2;
        } else if i > dp[i - 1] && span[i - dp[i - 1] - 1] == b'(' {
            let before_match = i - dp[i - 1];
            let inner = if before_match >= 2 {
                dp[before_match - 2]
            } else {
                0
            };
            dp[i] = dp[i - 1] + inner + 2;
        }

        max_len = max_len.max(dp[i]);
    }

    max_len
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples() {
        assert_eq!(solve("(()"), 2);
        assert_eq!(solve(")()())"), 4);
        assert_eq!(solve(""), 0);
        assert_eq!(solve("()()"), 4);
        assert_eq!(solve("()(())"), 6);
        assert_eq!(solve("((("), 0);
        assert_eq!(solve(")))"), 0);
        assert_eq!(solve("()"), 2);
    }
}
