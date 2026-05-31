//! Merge `k` sorted lists, three ways: a min-heap k-way merge, iterative
//! pairwise merging, and recursive divide-and-conquer.

use super::merge_two_sorted_lists::merge;
use super::{Link, from_slice};
use std::cmp::Reverse;
use std::collections::BinaryHeap;

/// Min-heap over the current front of each list. `BinaryHeap` is a max-heap, so
/// wrap entries in `Reverse`.
pub fn solve_with_min_heap(mut lists: Vec<Link>) -> Link {
    let mut heap: BinaryHeap<Reverse<(i32, usize)>> = BinaryHeap::new();
    for (i, list) in lists.iter().enumerate() {
        if let Some(node) = list {
            heap.push(Reverse((node.val, i)));
        }
    }

    let mut values = Vec::new();
    while let Some(Reverse((val, i))) = heap.pop() {
        values.push(val);
        if let Some(node) = lists[i].take() {
            lists[i] = node.next;
            if let Some(next) = &lists[i] {
                heap.push(Reverse((next.val, i)));
            }
        }
    }

    from_slice(&values)
}

/// Repeatedly merge lists in pairs until a single list remains.
pub fn solve_optimized(mut lists: Vec<Link>) -> Link {
    if lists.is_empty() {
        return None;
    }

    let mut count = lists.len();
    while count > 1 {
        let mut merged = 0;
        let mut i = 0;
        while i < count {
            let left = lists[i].take();
            let right = if i + 1 < count {
                lists[i + 1].take()
            } else {
                None
            };
            lists[merged] = merge(left, right);
            merged += 1;
            i += 2;
        }
        count = merged;
    }

    lists[0].take()
}

/// Divide the slice of lists in half, merge each half, then merge the two.
pub fn solve_with_divide_and_conquer(mut lists: Vec<Link>) -> Link {
    fn partition(lists: &mut [Link]) -> Link {
        match lists.len() {
            0 => None,
            1 => lists[0].take(),
            n => {
                let (left, right) = lists.split_at_mut(n / 2);
                merge(partition(left), partition(right))
            }
        }
    }

    partition(&mut lists)
}

#[cfg(test)]
mod tests {
    use super::super::{from_slice, to_vec};
    use super::*;

    fn build(input: &[&[i32]]) -> Vec<Link> {
        input.iter().map(|values| from_slice(values)).collect()
    }

    const SCENARIOS: &[(&[&[i32]], &[i32])] = &[
        (
            &[&[1, 4, 5], &[1, 3, 4], &[2, 6]],
            &[1, 1, 2, 3, 4, 4, 5, 6],
        ),
        (&[&[1]], &[1]),
        (&[&[-1, 0, 1], &[-3, -2]], &[-3, -2, -1, 0, 1]),
    ];

    #[test]
    fn with_min_heap() {
        for &(input, expected) in SCENARIOS {
            assert_eq!(
                to_vec(&solve_with_min_heap(build(input))),
                expected.to_vec()
            );
        }
    }

    #[test]
    fn optimized_pairwise() {
        for &(input, expected) in SCENARIOS {
            assert_eq!(to_vec(&solve_optimized(build(input))), expected.to_vec());
        }
    }

    #[test]
    fn divide_and_conquer() {
        for &(input, expected) in SCENARIOS {
            assert_eq!(
                to_vec(&solve_with_divide_and_conquer(build(input))),
                expected.to_vec()
            );
        }
    }
}
