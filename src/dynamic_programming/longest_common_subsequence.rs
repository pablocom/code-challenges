//! Longest Common Subsequence, shown three ways to contrast the techniques:
//! naive recursion, top-down memoization, and bottom-up tabulation.

/// Plain recursion — exponential, kept for contrast.
pub fn brute_force(x: &str, y: &str) -> usize {
    fn rec(x: &[u8], y: &[u8], m: usize, n: usize) -> usize {
        if m == 0 || n == 0 {
            return 0;
        }
        if x[m - 1] == y[n - 1] {
            1 + rec(x, y, m - 1, n - 1)
        } else {
            rec(x, y, m, n - 1).max(rec(x, y, m - 1, n))
        }
    }

    rec(x.as_bytes(), y.as_bytes(), x.len(), y.len())
}

/// Top-down with a memo table — O(m·n).
pub fn memoization(text1: &str, text2: &str) -> usize {
    fn rec(x: &[u8], y: &[u8], m: usize, n: usize, memo: &mut [Vec<Option<usize>>]) -> usize {
        if m == 0 || n == 0 {
            return 0;
        }
        if let Some(cached) = memo[m][n] {
            return cached;
        }

        let result = if x[m - 1] == y[n - 1] {
            1 + rec(x, y, m - 1, n - 1, memo)
        } else {
            rec(x, y, m, n - 1, memo).max(rec(x, y, m - 1, n, memo))
        };

        memo[m][n] = Some(result);
        result
    }

    let (x, y) = (text1.as_bytes(), text2.as_bytes());
    let mut memo = vec![vec![None; y.len() + 1]; x.len() + 1];
    rec(x, y, x.len(), y.len(), &mut memo)
}

/// Bottom-up table — O(m·n) time, O(m·n) space.
pub fn tabulation(text1: &str, text2: &str) -> usize {
    let (x, y) = (text1.as_bytes(), text2.as_bytes());
    let (m, n) = (x.len(), y.len());

    let mut table = vec![vec![0usize; n + 1]; m + 1];
    for i in 1..=m {
        for j in 1..=n {
            table[i][j] = if x[i - 1] == y[j - 1] {
                table[i - 1][j - 1] + 1
            } else {
                table[i - 1][j].max(table[i][j - 1])
            };
        }
    }

    table[m][n]
}

#[cfg(test)]
mod tests {
    use super::*;

    const SCENARIOS: &[(&str, &str, usize)] = &[
        ("abcde", "ace", 3),
        ("ACB", "AIB", 2),
        ("AGGTAB", "GXTXAYB", 4),
        ("ABCDGH", "AEDFHR", 3),
        ("ABCDGH", "", 0),
        ("", "ADV", 0),
    ];

    #[test]
    fn all_three_agree() {
        for &(a, b, expected) in SCENARIOS {
            assert_eq!(brute_force(a, b), expected, "brute_force {a:?} {b:?}");
            assert_eq!(memoization(a, b), expected, "memoization {a:?} {b:?}");
            assert_eq!(tabulation(a, b), expected, "tabulation {a:?} {b:?}");
        }
    }
}
