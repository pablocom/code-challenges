//! Levenshtein (edit) distance — minimum single-character insertions,
//! deletions, and substitutions to turn one string into another.
//!
//! Classic 2-D tabulation: `dp[i][j]` is the distance between the `i`-char and
//! `j`-char prefixes.

pub fn levenshtein_distance(source: &str, target: &str) -> usize {
    let source = source.as_bytes();
    let target = target.as_bytes();
    let (m, n) = (source.len(), target.len());

    let mut dp = vec![vec![0usize; n + 1]; m + 1];
    for (i, row) in dp.iter_mut().enumerate() {
        row[0] = i;
    }
    for j in 0..=n {
        dp[0][j] = j;
    }

    for i in 1..=m {
        for j in 1..=n {
            dp[i][j] = if source[i - 1] == target[j - 1] {
                dp[i - 1][j - 1]
            } else {
                1 + dp[i - 1][j].min(dp[i - 1][j - 1]).min(dp[i][j - 1])
            };
        }
    }

    dp[m][n]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples() {
        assert_eq!(levenshtein_distance("pablo", "company"), 6);
        assert_eq!(
            levenshtein_distance("zoologicoarchaeologist", "zoogeologist"),
            10
        );
    }

    #[test]
    fn identical_strings_have_zero_distance() {
        assert_eq!(levenshtein_distance("rust", "rust"), 0);
    }
}
