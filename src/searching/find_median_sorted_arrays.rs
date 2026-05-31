//! Median of two sorted arrays.
//!
//! Walk the merge of both arrays up to the middle, remembering the last two
//! values seen. The median is the middle value (odd total length) or the
//! average of the two middle values (even). O(m + n).

pub fn solve(nums1: &[i32], nums2: &[i32]) -> f64 {
    let total = nums1.len() + nums2.len();
    let (mut i, mut j) = (0, 0);
    let mut previous = 0;
    let mut current = 0;

    for _ in 0..=total / 2 {
        previous = current;
        current = if j >= nums2.len() || (i < nums1.len() && nums1[i] <= nums2[j]) {
            let value = nums1[i];
            i += 1;
            value
        } else {
            let value = nums2[j];
            j += 1;
            value
        };
    }

    if total.is_multiple_of(2) {
        f64::from(previous + current) / 2.0
    } else {
        f64::from(current)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn odd_total_length() {
        assert_eq!(solve(&[2, 3], &[1]), 2.0);
        assert_eq!(solve(&[1, 3], &[2]), 2.0);
        assert_eq!(solve(&[1, 2, 3], &[4, 5]), 3.0);
        assert_eq!(solve(&[1, 3], &[2, 4, 5]), 3.0);
    }

    #[test]
    fn even_total_length() {
        assert_eq!(solve(&[1, 2], &[2, 4]), 2.0);
        assert_eq!(solve(&[1, 2], &[2, 3]), 2.0);
        assert_eq!(solve(&[1], &[3, 3, 4, 5, 6]), 3.5);
        assert_eq!(solve(&[1, 2, 3, 6, 7], &[4]), 3.5);
        assert_eq!(solve(&[1, 3], &[2, 7]), 2.5);
        assert_eq!(solve(&[2, 2, 4, 4], &[2, 2, 4, 4]), 3.0);
        assert_eq!(solve(&[1, 2], &[-1, 3]), 1.5);
        assert_eq!(solve(&[100001], &[100000]), 100000.5);
        assert_eq!(solve(&[4], &[1, 2, 3]), 2.5);
        assert_eq!(solve(&[2, 3, 4], &[1]), 2.5);
    }

    #[test]
    fn one_array_empty() {
        assert_eq!(solve(&[], &[1, 2, 3, 4]), 2.5);
        assert_eq!(solve(&[2, 3], &[]), 2.5);
    }
}
