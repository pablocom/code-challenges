//! Minimum Path Sum — cheapest top-left to bottom-right path moving only right
//! or down. Fill costs from the bottom-right corner backward.

pub fn solve(grid: &[Vec<i32>]) -> i32 {
    let mut dp = grid.to_vec();
    let rows = dp.len();
    let cols = dp[0].len();

    for i in (0..rows).rev() {
        for j in (0..cols).rev() {
            let down = if i + 1 < rows { dp[i + 1][j] } else { i32::MAX };
            let right = if j + 1 < cols { dp[i][j + 1] } else { i32::MAX };

            let best = down.min(right);
            if best != i32::MAX {
                dp[i][j] += best;
            }
        }
    }

    dp[0][0]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn grid(rows: &[&[i32]]) -> Vec<Vec<i32>> {
        rows.iter().map(|r| r.to_vec()).collect()
    }

    #[test]
    fn examples() {
        assert_eq!(solve(&grid(&[&[5]])), 5);
        assert_eq!(solve(&grid(&[&[1, 2, 3]])), 6);
        assert_eq!(solve(&grid(&[&[1], &[2], &[3]])), 6);
        assert_eq!(solve(&grid(&[&[1, 3, 1], &[1, 5, 1], &[4, 2, 1]])), 7);
        assert_eq!(solve(&grid(&[&[1, 2, 3], &[4, 5, 6]])), 12);
        assert_eq!(solve(&grid(&[&[0, 0], &[0, 0]])), 0);
        assert_eq!(solve(&grid(&[&[1, 9], &[1, 1], &[1, 1]])), 4);
        assert_eq!(solve(&grid(&[&[1, 1, 1], &[9, 9, 1]])), 4);
    }
}
