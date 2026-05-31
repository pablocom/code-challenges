//! Container With Most Water — maximize `width * min(height_left, height_right)`.
//!
//! Two pointers from the ends, always advancing the shorter wall (the only
//! move that could ever increase the area). O(n).

pub fn solve(heights: &[i32]) -> i32 {
    if heights.len() < 2 {
        return 0;
    }

    let (mut left, mut right) = (0usize, heights.len() - 1);
    let mut highest = 0;

    while left < right {
        let volume = (right - left) as i32 * heights[left].min(heights[right]);
        highest = highest.max(volume);

        if heights[left] < heights[right] {
            left += 1;
        } else {
            right -= 1;
        }
    }

    highest
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples() {
        assert_eq!(solve(&[1, 1]), 1);
        assert_eq!(solve(&[2, 1]), 1);
        assert_eq!(solve(&[2, 1, 2]), 4);
        assert_eq!(solve(&[1, 2, 1, 2]), 4);
        assert_eq!(solve(&[1, 8, 6, 2, 5, 4, 8, 3, 7]), 49);
        assert_eq!(solve(&[1, 2, 1]), 2);
    }
}
