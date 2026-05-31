//! Locate the earliest month that best balances the average price before it
//! against the average price after it (smallest absolute difference of the two
//! running averages). Returns a 1-based month.

pub fn locate_earliest_month(stock_price: &[i32]) -> usize {
    if stock_price.len() == 2 {
        return if stock_price[0] <= stock_price[1] {
            1
        } else {
            2
        };
    }

    let total: i64 = stock_price.iter().map(|&p| i64::from(p)).sum();

    let mut best_index = 0;
    let mut smallest_gap = i64::MAX;
    let mut prefix_sum: i64 = 0;

    for (i, &price) in stock_price.iter().enumerate() {
        prefix_sum += i64::from(price);
        let before_average = prefix_sum / (i as i64 + 1);

        let remaining_count = stock_price.len() as i64 - (i as i64 + 1);
        let after_average = if remaining_count > 0 {
            (total - prefix_sum) / remaining_count
        } else {
            0
        };

        let gap = (before_average - after_average).abs();
        if gap < smallest_gap {
            smallest_gap = gap;
            best_index = i;
        }
    }

    best_index + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples() {
        assert_eq!(locate_earliest_month(&[1, 3, 2, 4, 5]), 2);
        assert_eq!(locate_earliest_month(&[1, 1, 1, 1, 1]), 1);
        assert_eq!(locate_earliest_month(&[1, 3, 2, 3, 1]), 2);
        assert_eq!(locate_earliest_month(&[2, 5]), 1);
        assert_eq!(locate_earliest_month(&[2, 2]), 1);
        assert_eq!(locate_earliest_month(&[2]), 1);
    }
}
