//! Task Scheduler — least time to run all tasks with a cooldown of `n` between
//! repeats of the same task. Demonstrates reusing the home-grown [`MaxHeap`].
//!
//! - [`solve`] simulates each tick: run the most frequent ready task, parking
//!   it in a cooldown queue until it is allowed again.
//! - [`solve_with_formula`] computes the answer directly from the most frequent
//!   task(s).

use super::max_heap::MaxHeap;
use std::collections::{HashMap, VecDeque};

pub fn solve(tasks: &[char], n: i32) -> i32 {
    let mut counts: HashMap<char, i32> = HashMap::new();
    for &task in tasks {
        *counts.entry(task).or_insert(0) += 1;
    }

    let mut ready: MaxHeap<i32> = counts.into_values().collect();
    let mut cooling: VecDeque<(i32, i32)> = VecDeque::new(); // (remaining, ready_at)
    let mut time = 0;

    while !ready.is_empty() || !cooling.is_empty() {
        time += 1;

        if let Some(max) = ready.pop_max() {
            let remaining = max - 1;
            if remaining != 0 {
                cooling.push_back((remaining, time + n));
            }
        }

        if let Some(&(remaining, ready_at)) = cooling.front()
            && ready_at == time
        {
            cooling.pop_front();
            ready.insert(remaining);
        }
    }

    time
}

pub fn solve_with_formula(tasks: &[char], n: i32) -> i32 {
    let mut frequencies = [0i32; 26];
    for &task in tasks {
        frequencies[(task as u8 - b'A') as usize] += 1;
    }

    let max_frequency = *frequencies.iter().max().unwrap();
    let max_frequency_count = frequencies.iter().filter(|&&f| f == max_frequency).count() as i32;

    // The busiest task(s) dictate the idle slots; everything else fills the gaps.
    let min_length = (max_frequency - 1) * (n + 1) + max_frequency_count;
    min_length.max(tasks.len() as i32)
}

#[cfg(test)]
mod tests {
    use super::*;

    const SCENARIOS: &[(&[char], i32, i32)] = &[
        (&['A', 'A', 'A', 'B', 'B', 'B'], 2, 8),
        (&['A', 'A', 'A', 'B', 'B', 'B'], 0, 6),
        (
            &['A', 'A', 'A', 'A', 'A', 'A', 'B', 'C', 'D', 'E', 'F', 'G'],
            2,
            16,
        ),
        (&['A'], 2, 1),
        (&['A', 'B'], 2, 2),
        (&['A', 'A'], 1, 3),
        (&['A', 'A'], 2, 4),
        (&['A', 'A', 'A'], 2, 7),
        (&['A', 'B', 'C', 'D'], 2, 4),
        (&['A', 'A', 'B', 'B', 'C', 'C'], 2, 6),
        (&['A', 'A', 'A', 'B', 'B', 'B', 'C', 'C', 'C'], 2, 9),
        (
            &['A', 'A', 'A', 'B', 'B', 'B', 'C', 'C', 'C', 'D', 'D', 'E'],
            2,
            12,
        ),
    ];

    #[test]
    fn simulation() {
        for &(tasks, n, expected) in SCENARIOS {
            assert_eq!(solve(tasks, n), expected, "solve({tasks:?}, {n})");
        }
    }

    #[test]
    fn formula() {
        for &(tasks, n, expected) in SCENARIOS {
            assert_eq!(
                solve_with_formula(tasks, n),
                expected,
                "formula({tasks:?}, {n})"
            );
        }
    }
}
