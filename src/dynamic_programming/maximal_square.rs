//! Maximal Square — the area of the largest all-`'1'` square in a binary
//! matrix. Each row is given as a string of `'0'`/`'1'`.
//!
//! `cache[i][j]` is the side of the largest square whose top-left corner is
//! `(i, j)`: 1 plus the minimum of its right, down, and diagonal neighbours.

pub fn solve(matrix: &[&str]) -> usize {
    if matrix.is_empty() || matrix[0].is_empty() {
        return 0;
    }

    let rows = matrix.len();
    let cols = matrix[0].len();
    let grid: Vec<&[u8]> = matrix.iter().map(|r| r.as_bytes()).collect();

    let mut cache = vec![vec![0usize; cols]; rows];
    let mut max_side = 0;

    for i in (0..rows).rev() {
        for j in (0..cols).rev() {
            if grid[i][j] == b'0' {
                continue;
            }

            let down = if i + 1 < rows { cache[i + 1][j] } else { 0 };
            let right = if j + 1 < cols { cache[i][j + 1] } else { 0 };
            let diagonal = if i + 1 < rows && j + 1 < cols {
                cache[i + 1][j + 1]
            } else {
                0
            };

            cache[i][j] = 1 + down.min(right).min(diagonal);
            max_side = max_side.max(cache[i][j]);
        }
    }

    max_side * max_side
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn leetcode_examples() {
        assert_eq!(solve(&["10100", "10111", "11111", "10010"]), 4);
        assert_eq!(solve(&["01", "10"]), 1);
    }

    #[test]
    fn edge_cases() {
        assert_eq!(solve(&["00", "00"]), 0);
        assert_eq!(solve(&["1"]), 1);
        assert_eq!(solve(&["0"]), 0);
        assert_eq!(solve(&["11", "11"]), 4);
        assert_eq!(solve(&["111", "111", "111"]), 9);
        assert_eq!(solve(&["111"]), 1);
        assert_eq!(solve(&["1", "1", "1"]), 1);
        assert_eq!(solve(&["11", "10"]), 1);
    }

    #[test]
    fn large_matrix_with_diagonal_constraint() {
        assert_eq!(
            solve(&["11111111", "11111110", "11111110", "11111000", "01111000"]),
            16
        );
    }
}
