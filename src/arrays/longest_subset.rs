//! Longest subset whose elements are pairwise congruent modulo `m`
//! (i.e. all differences are multiples of `m`).
//!
//! Two takes are preserved from the original study: a memoized scan that marks
//! already-grouped indices, and a brute-force scan that recounts from each
//! divisible pair.

/// Memoized: once an index has been folded into a group, skip re-processing it.
pub fn solve_optimized(a: &[i32], m: i32) -> usize {
    let mut maximum = 1usize;
    let mut solved = vec![false; a.len()];
    let upper = a.len().saturating_sub(1);

    for i in 0..upper {
        if solved[i] {
            break;
        }

        let mut any_subset = false;
        let current_value = a[i];
        let mut current_count = 1usize;

        for j in i + 1..upper {
            if (current_value - a[j]).abs() % m == 0 {
                any_subset = true;
                current_count += 1;
                solved[j] = true;
            }
        }

        maximum = if any_subset {
            current_count += 1;
            maximum.max(current_count)
        } else {
            1
        };
    }

    maximum
}

/// Brute force: for every divisible pair, count the chain that follows.
pub fn solve(a: &[i32], m: i32) -> usize {
    let mut maximum = 1usize;
    let upper = a.len().saturating_sub(1);

    for i in 0..upper {
        for j in i + 1..a.len() {
            let distance = (a[i] - a[j]).abs();
            if distance == 0 {
                continue;
            }
            if distance % m == 0 {
                maximum = maximum.max(count_subset(a, j, m));
            }
        }
    }

    maximum
}

fn count_subset(a: &[i32], mut last_divisible: usize, m: i32) -> usize {
    let mut count = 2usize;

    for i in last_divisible + 1..a.len() {
        let distance = (a[last_divisible] - a[i]).abs();
        if distance == 0 {
            continue;
        }
        if distance % m == 0 {
            count += 1;
            last_divisible = i;
        }
    }

    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn optimized_examples() {
        assert_eq!(solve_optimized(&[-2, 1, 5, 8, 11], 3), 3);
        assert_eq!(solve_optimized(&[-2, 0, 1, 5, 8, 4, 11, 8, 12], 4), 5);
        assert_eq!(solve_optimized(&[1, 0, 1, 4, 8, 4, 2, 3, 2], 1), 9);
        assert_eq!(solve_optimized(&[1, 0, 1, 4, 6, 4, 2, 0, 3, 2], 2), 7);
    }
}
