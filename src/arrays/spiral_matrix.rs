//! Generate an `n x n` matrix filled with `1..=n²` in clockwise spiral order.
//!
//! Fill one concentric layer at a time: top row, right column, bottom row,
//! left column. An odd `n` leaves a single centre cell to fill last.

pub fn solve(n: usize) -> Vec<Vec<i32>> {
    let mut matrix = vec![vec![0; n]; n];
    let mut value = 1;

    for layer in 0..n / 2 {
        let last = n - 1 - layer;

        for col in layer..last {
            matrix[layer][col] = value;
            value += 1;
        }
        for row in layer..last {
            matrix[row][last] = value;
            value += 1;
        }
        for col in (layer + 1..=last).rev() {
            matrix[last][col] = value;
            value += 1;
        }
        for row in (layer + 1..=last).rev() {
            matrix[row][layer] = value;
            value += 1;
        }
    }

    if n % 2 == 1 {
        matrix[n / 2][n / 2] = value;
    }

    matrix
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn size_one() {
        assert_eq!(solve(1), vec![vec![1]]);
    }

    #[test]
    fn size_two() {
        assert_eq!(solve(2), vec![vec![1, 2], vec![4, 3]]);
    }

    #[test]
    fn size_three() {
        assert_eq!(
            solve(3),
            vec![vec![1, 2, 3], vec![8, 9, 4], vec![7, 6, 5]]
        );
    }

    #[test]
    fn size_four() {
        assert_eq!(
            solve(4),
            vec![
                vec![1, 2, 3, 4],
                vec![12, 13, 14, 5],
                vec![11, 16, 15, 6],
                vec![10, 9, 8, 7],
            ]
        );
    }
}
