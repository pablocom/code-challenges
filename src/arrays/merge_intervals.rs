//! Merge all overlapping intervals.
//!
//! Sort by start, then sweep: extend the last kept interval when the next one
//! overlaps (or merely touches) it, otherwise start a fresh interval.

pub fn solve(intervals: &[[i32; 2]]) -> Vec<[i32; 2]> {
    if intervals.len() <= 1 {
        return intervals.to_vec();
    }

    let mut sorted = intervals.to_vec();
    sorted.sort_by_key(|interval| interval[0]);

    let mut merged: Vec<[i32; 2]> = Vec::new();
    for interval in sorted {
        match merged.last_mut() {
            Some(last) if last[1] >= interval[0] => last[1] = last[1].max(interval[1]),
            _ => merged.push(interval),
        }
    }

    merged
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn merges_overlapping_intervals() {
        let result = solve(&[[1, 3], [2, 6], [8, 10], [15, 18]]);
        assert_eq!(result, vec![[1, 6], [8, 10], [15, 18]]);
    }

    #[test]
    fn merges_intervals_touching_at_boundary() {
        assert_eq!(solve(&[[1, 4], [4, 5]]), vec![[1, 5]]);
    }

    #[test]
    fn single_interval_returns_as_is() {
        assert_eq!(solve(&[[5, 10]]), vec![[5, 10]]);
    }

    #[test]
    fn non_overlapping_intervals_return_all() {
        assert_eq!(
            solve(&[[1, 2], [4, 6], [8, 10]]),
            vec![[1, 2], [4, 6], [8, 10]]
        );
    }

    #[test]
    fn all_intervals_overlap_into_one() {
        assert_eq!(solve(&[[1, 4], [2, 6], [3, 8], [5, 10]]), vec![[1, 10]]);
    }

    #[test]
    fn nested_interval_is_absorbed() {
        assert_eq!(solve(&[[1, 10], [2, 5]]), vec![[1, 10]]);
    }

    #[test]
    fn unsorted_input_is_handled() {
        let result = solve(&[[8, 10], [1, 3], [15, 18], [2, 6]]);
        assert_eq!(result, vec![[1, 6], [8, 10], [15, 18]]);
    }

    #[test]
    fn adjacent_non_overlapping_intervals_stay_separate() {
        assert_eq!(solve(&[[1, 3], [5, 7]]), vec![[1, 3], [5, 7]]);
    }
}
