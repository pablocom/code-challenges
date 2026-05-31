//! Two Sum — return the indices of the two numbers that add up to `target`.
//!
//! Besides the canonical one-pass hash-map solution, this mirrors the
//! original "strategy" study: a `WriteHeavyTwoSum` that does cheap inserts and
//! a `ReadHeavyTwoSum` that precomputes every pair-sum for O(1) lookups.

use std::collections::HashMap;

/// One-pass hash map: O(n) time, O(n) space.
pub fn solve(nums: &[i32], target: i32) -> Option<[usize; 2]> {
    let mut position_by_number: HashMap<i32, usize> = HashMap::new();

    for (i, &n) in nums.iter().enumerate() {
        let complement = target - n;
        if let Some(&index) = position_by_number.get(&complement) {
            return Some([index, i]);
        }
        position_by_number.insert(n, i);
    }

    None
}

/// An incremental two-sum: numbers are streamed in, then queried by sum.
pub trait TwoSumStrategy {
    fn add(&mut self, number: i32);
    fn find(&self, target: i32) -> Option<[usize; 2]>;
}

/// Cheap inserts, more work per query.
#[derive(Default)]
pub struct WriteHeavyTwoSum {
    indices_by_number: HashMap<i32, Vec<usize>>,
    next_index: usize,
}

impl TwoSumStrategy for WriteHeavyTwoSum {
    fn add(&mut self, number: i32) {
        self.indices_by_number
            .entry(number)
            .or_default()
            .push(self.next_index);
        self.next_index += 1;
    }

    fn find(&self, target: i32) -> Option<[usize; 2]> {
        for (&number, indices) in &self.indices_by_number {
            let complement = target - number;

            if complement == number {
                if indices.len() > 1 {
                    return Some([indices[0], indices[1]]);
                }
            } else if let Some(complement_indices) = self.indices_by_number.get(&complement) {
                let (a, b) = (indices[0], complement_indices[0]);
                return Some([a.min(b), a.max(b)]);
            }
        }

        None
    }
}

/// Expensive inserts (records every pair sum), O(1) queries.
#[derive(Default)]
pub struct ReadHeavyTwoSum {
    numbers: Vec<i32>,
    pair_by_sum: HashMap<i32, [usize; 2]>,
}

impl TwoSumStrategy for ReadHeavyTwoSum {
    fn add(&mut self, number: i32) {
        let next_index = self.numbers.len();
        for (i, &existing) in self.numbers.iter().enumerate() {
            self.pair_by_sum.entry(existing + number).or_insert([i, next_index]);
        }
        self.numbers.push(number);
    }

    fn find(&self, target: i32) -> Option<[usize; 2]> {
        self.pair_by_sum.get(&target).copied()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SCENARIOS: &[(&[i32], i32, [usize; 2])] = &[
        (&[2, 7, 11, 15], 9, [0, 1]),
        (&[3, 2, 4], 6, [1, 2]),
        (&[3, 3], 6, [0, 1]),
        (&[-1, -2, -3], -5, [1, 2]),
    ];

    fn feed<S: TwoSumStrategy>(strategy: &mut S, nums: &[i32]) {
        for &n in nums {
            strategy.add(n);
        }
    }

    #[test]
    fn solve_finds_indices() {
        for &(nums, target, expected) in SCENARIOS {
            assert_eq!(solve(nums, target), Some(expected));
        }
    }

    #[test]
    fn write_heavy_finds_indices() {
        for &(nums, target, expected) in SCENARIOS {
            let mut strategy = WriteHeavyTwoSum::default();
            feed(&mut strategy, nums);
            assert_eq!(strategy.find(target), Some(expected));
        }
    }

    #[test]
    fn read_heavy_finds_indices() {
        for &(nums, target, expected) in SCENARIOS {
            let mut strategy = ReadHeavyTwoSum::default();
            feed(&mut strategy, nums);
            assert_eq!(strategy.find(target), Some(expected));
        }
    }

    #[test]
    fn no_solution_returns_none() {
        assert_eq!(solve(&[1, 2, 3], 100), None);

        let mut write_heavy = WriteHeavyTwoSum::default();
        feed(&mut write_heavy, &[1, 2, 3]);
        assert_eq!(write_heavy.find(100), None);

        let mut read_heavy = ReadHeavyTwoSum::default();
        feed(&mut read_heavy, &[1, 2, 3]);
        assert_eq!(read_heavy.find(100), None);
    }
}
