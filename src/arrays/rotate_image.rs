//! Rotate an `n x n` matrix 90° clockwise, in place.
//!
//! Process one concentric layer at a time; within a layer, rotate elements in
//! groups of four (top-left → top-right → bottom-right → bottom-left).

pub fn solve(matrix: &mut [Vec<i32>]) {
    let n = matrix.len();
    let layers = n / 2;

    for layer in 0..layers {
        let last = n - 1 - layer;
        let rotations = last - layer;

        for offset in 0..rotations {
            let top_left = matrix[layer][layer + offset];
            let top_right = matrix[layer + offset][last];
            let bottom_right = matrix[last][last - offset];
            let bottom_left = matrix[last - offset][layer];

            matrix[layer][layer + offset] = bottom_left;
            matrix[layer + offset][last] = top_left;
            matrix[last][last - offset] = top_right;
            matrix[last - offset][layer] = bottom_right;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rotates_3x3_clockwise() {
        let mut matrix = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        solve(&mut matrix);
        assert_eq!(matrix, vec![vec![7, 4, 1], vec![8, 5, 2], vec![9, 6, 3]]);
    }

    #[test]
    fn rotates_4x4_clockwise() {
        let mut matrix = vec![
            vec![1, 2, 3, 4],
            vec![5, 6, 7, 8],
            vec![9, 10, 11, 12],
            vec![13, 14, 15, 16],
        ];
        solve(&mut matrix);
        assert_eq!(
            matrix,
            vec![
                vec![13, 9, 5, 1],
                vec![14, 10, 6, 2],
                vec![15, 11, 7, 3],
                vec![16, 12, 8, 4],
            ]
        );
    }

    #[test]
    fn single_element_is_unchanged() {
        let mut matrix = vec![vec![42]];
        solve(&mut matrix);
        assert_eq!(matrix, vec![vec![42]]);
    }
}
