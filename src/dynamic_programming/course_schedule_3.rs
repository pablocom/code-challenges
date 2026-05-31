//! Course Schedule III — maximum number of `[duration, deadline]` courses that
//! can be taken sequentially.
//!
//! Greedy: process courses by ascending deadline, always taking the next one.
//! If that overruns its deadline, drop the longest course taken so far (a
//! max-heap of durations); this never reduces the count and frees the most time.

use std::collections::BinaryHeap;

pub fn solve(courses: &[[i32; 2]]) -> usize {
    let mut courses = courses.to_vec();
    courses.sort_by_key(|course| course[1]);

    let mut durations: BinaryHeap<i32> = BinaryHeap::new();
    let mut current_time = 0;

    for course in courses {
        let [duration, deadline] = course;

        durations.push(duration);
        current_time += duration;

        if current_time > deadline
            && let Some(longest) = durations.pop()
        {
            current_time -= longest;
        }
    }

    durations.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples() {
        assert_eq!(
            solve(&[[100, 200], [200, 1300], [1000, 1250], [2000, 3200]]),
            3
        );
        assert_eq!(solve(&[[1, 2]]), 1);
        assert_eq!(solve(&[[3, 2], [4, 3]]), 0);
    }
}
