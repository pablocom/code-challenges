//! Cosine similarity of two sparse vectors given as parallel `(keys, values)`
//! slices with keys in ascending order.
//!
//! Merge the two key lists to accumulate the dot product over shared indices,
//! then divide by the product of the vectors' magnitudes.

use std::cmp::Ordering;

pub fn solve(keys_a: &[i32], values_a: &[f64], keys_b: &[i32], values_b: &[f64]) -> f64 {
    let sum_sq_a: f64 = values_a.iter().map(|v| v * v).sum();
    let sum_sq_b: f64 = values_b.iter().map(|v| v * v).sum();

    if sum_sq_a == 0.0 || sum_sq_b == 0.0 {
        return 0.0;
    }

    let (mut a, mut b) = (0, 0);
    let mut dot_product = 0.0;
    while a < keys_a.len() && b < keys_b.len() {
        match keys_a[a].cmp(&keys_b[b]) {
            Ordering::Equal => {
                dot_product += values_a[a] * values_b[b];
                a += 1;
                b += 1;
            }
            Ordering::Less => a += 1,
            Ordering::Greater => b += 1,
        }
    }

    dot_product / (sum_sq_a.sqrt() * sum_sq_b.sqrt())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_vectors_return_zero() {
        assert_eq!(solve(&[], &[], &[], &[]), 0.0);
    }

    #[test]
    fn identical_single_entry_is_perfectly_similar() {
        assert_eq!(solve(&[42], &[3.5], &[42], &[7.2]), 1.0);
    }

    #[test]
    fn opposite_single_entry_is_minus_one() {
        assert_eq!(solve(&[7], &[2.0], &[7], &[-5.0]), -1.0);
    }

    #[test]
    fn disjoint_indices_return_zero() {
        assert_eq!(
            solve(&[1, 3, 5], &[2.0, 4.0, 6.0], &[2, 4, 6], &[1.0, 1.0, 1.0]),
            0.0
        );
    }

    #[test]
    fn multiple_matches_with_mixed_signs() {
        let result = solve(
            &[2, 4, 5, 8],
            &[7.0, 5.0, 12.0, 1.0],
            &[1, 2, 5, 8],
            &[3.0, 4.0, -1.0, 2.0],
        );
        let expected = (7.0 * 4.0 + 12.0 * -1.0 + 1.0 * 2.0)
            / ((7.0f64 * 7.0 + 5.0 * 5.0 + 12.0 * 12.0 + 1.0 * 1.0).sqrt()
                * (3.0f64 * 3.0 + 4.0 * 4.0 + 1.0 * 1.0 + 2.0 * 2.0).sqrt());
        assert!((result - expected).abs() < 1e-12);
    }

    #[test]
    fn known_scenario() {
        let result = solve(
            &[736565921, 2014363605],
            &[871.250834721723, 693.657922881496],
            &[736565921, 2014363605],
            &[160.05272192883, 758.489622156364],
        );
        assert!((result - 0.770969008850541).abs() < 1e-15);
    }
}
