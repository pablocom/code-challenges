//! Last Stone Weight — repeatedly smash the two heaviest stones together.
//!
//! A max-heap ([`BinaryHeap`]) yields the two largest in O(log n) each round.

use std::collections::BinaryHeap;

/// Reinsert the difference only when the two stones differ.
pub fn solve(stones: &[i32]) -> i32 {
    let mut heap: BinaryHeap<i32> = stones.iter().copied().collect();

    while heap.len() > 1 {
        let first = heap.pop().unwrap();
        let second = heap.pop().unwrap();
        if first != second {
            heap.push(first - second);
        }
    }

    heap.pop().unwrap_or(0)
}

/// A variant that always reinserts the difference (pushing 0 on a tie).
pub fn solve_always_reinserting(stones: &[i32]) -> i32 {
    let mut heap: BinaryHeap<i32> = stones.iter().copied().collect();

    while heap.len() > 1 {
        let first = heap.pop().unwrap();
        let second = heap.pop().unwrap();
        heap.push(first - second);
    }

    heap.pop().unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    const SCENARIOS: &[(&[i32], i32)] = &[
        (&[2, 7, 4, 1, 8, 1], 1),
        (&[1], 1),
        (&[2, 2], 0),
        (&[3, 7, 2], 2),
        (&[1, 3, 5, 7, 9], 1),
        (&[10, 10, 10, 10], 0),
    ];

    #[test]
    fn solve_examples() {
        for &(stones, expected) in SCENARIOS {
            assert_eq!(solve(stones), expected);
        }
    }

    #[test]
    fn solve_always_reinserting_examples() {
        for &(stones, expected) in SCENARIOS {
            assert_eq!(solve_always_reinserting(stones), expected);
        }
    }
}
