//! Best Time to Buy and Sell Stock — maximum profit from a single buy/sell.
//!
//! Kadane over consecutive price deltas: the best profit is the maximum-sum run
//! of daily gains.

pub fn max_profit(prices: &[i32]) -> i32 {
    let mut max_current = 0;
    let mut max_so_far = 0;

    for window in prices.windows(2) {
        max_current = 0.max(max_current + (window[1] - window[0]));
        max_so_far = max_so_far.max(max_current);
    }

    max_so_far
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples() {
        assert_eq!(max_profit(&[1, 2, 3, 4, 5]), 4);
        assert_eq!(max_profit(&[7, 1, 5, 3, 6, 4]), 5);
    }

    #[test]
    fn monotonically_decreasing_yields_no_profit() {
        assert_eq!(max_profit(&[7, 6, 4, 3, 1]), 0);
    }
}
