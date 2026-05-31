//! Read a matrix in clockwise spiral order.
//!
//! Shrink four boundaries (top, bottom, left, right) inward as each edge is
//! consumed, stopping when they cross.

pub fn spiral_order(matrix: &[Vec<i32>]) -> Vec<i32> {
    if matrix.is_empty() || matrix[0].is_empty() {
        return Vec::new();
    }

    let mut result = Vec::with_capacity(matrix.len() * matrix[0].len());
    let (mut top, mut bottom) = (0isize, matrix.len() as isize - 1);
    let (mut left, mut right) = (0isize, matrix[0].len() as isize - 1);

    while top <= bottom && left <= right {
        for col in left..=right {
            result.push(matrix[top as usize][col as usize]);
        }
        top += 1;

        for row in top..=bottom {
            result.push(matrix[row as usize][right as usize]);
        }
        right -= 1;

        if top <= bottom {
            for col in (left..=right).rev() {
                result.push(matrix[bottom as usize][col as usize]);
            }
            bottom -= 1;
        }

        if left <= right {
            for row in (top..=bottom).rev() {
                result.push(matrix[row as usize][left as usize]);
            }
            left += 1;
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn square_matrix() {
        let matrix = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        assert_eq!(spiral_order(&matrix), vec![1, 2, 3, 6, 9, 8, 7, 4, 5]);
    }

    #[test]
    fn wide_matrix() {
        let matrix = vec![vec![1, 2, 3, 4], vec![5, 6, 7, 8], vec![9, 10, 11, 12]];
        assert_eq!(
            spiral_order(&matrix),
            vec![1, 2, 3, 4, 8, 12, 11, 10, 9, 5, 6, 7]
        );
    }
}
