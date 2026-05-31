//! Coin Change — fewest coins that sum to `amount`, or `-1` if impossible.
//!
//! Bottom-up: `dp[a]` is the minimum coin count for amount `a`, seeded with an
//! unreachable sentinel so "impossible" is detectable at the end.

pub fn solve(coins: &[i32], amount: i32) -> i32 {
    let amount = amount as usize;
    let unreachable = (amount + 1) as i32;

    let mut dp = vec![unreachable; amount + 1];
    dp[0] = 0;

    for a in 1..=amount {
        for &coin in coins {
            let coin = coin as usize;
            if a >= coin {
                dp[a] = dp[a].min(1 + dp[a - coin]);
            }
        }
    }

    if dp[amount] == unreachable {
        -1
    } else {
        dp[amount]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples() {
        assert_eq!(solve(&[1, 2, 5], 11), 3);
        assert_eq!(solve(&[2], 3), -1);
        assert_eq!(solve(&[1], 0), 0);
        assert_eq!(solve(&[1], 1), 1);
        assert_eq!(solve(&[1], 5), 5);
        assert_eq!(solve(&[5], 5), 1);
        assert_eq!(solve(&[2], 4), 2);
        assert_eq!(solve(&[3], 7), -1);
        assert_eq!(solve(&[1, 2, 5], 0), 0);
        assert_eq!(solve(&[1, 3, 4], 6), 2);
        assert_eq!(solve(&[1, 5, 10], 12), 3);
        assert_eq!(solve(&[2, 5, 10], 1), -1);
        assert_eq!(solve(&[186, 419, 83, 408], 6249), 20);
    }
}
