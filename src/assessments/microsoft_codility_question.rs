//! Two Microsoft/Codility questions.
//!
//! - [`question1`]: minimum cost to delete characters so no two adjacent are
//!   equal — for each run, delete all but the costliest character.
//! - [`question2`]: a running selection over odd (1-based) positions, bounded
//!   by `k`.

pub fn question1(s: &str, costs: &[i32]) -> i32 {
    let bytes = s.as_bytes();
    let mut total_cost = 0;
    let mut run_cost = 0;
    let mut run_max = i32::MAX;
    let mut last_char = b':'; // a sentinel that never appears in the input

    for i in 0..bytes.len() {
        if bytes[i] == last_char {
            run_max = run_max.max(costs[i]);
            run_cost += costs[i];

            let run_ends = i + 1 >= bytes.len() || bytes[i + 1] != last_char;
            if run_ends {
                total_cost += run_cost - run_max;
                run_cost = costs[i];
                run_max = costs[i];
            }
        } else {
            run_cost = costs[i];
            run_max = costs[i];
            last_char = bytes[i];
        }
    }

    total_cost
}

pub fn question2(a: &[i32], k: i32) -> i32 {
    let mut result = 0;
    let mut added = 0;
    let mut last_min_added = i32::MIN;

    for (i, &value) in a.iter().enumerate() {
        if (i + 1).is_multiple_of(2) {
            continue;
        }

        if added < k {
            result += value;
            added += 1;
            last_min_added = last_min_added.min(value);
        } else if added == k && last_min_added < value {
            result = result + last_min_added + value;
            last_min_added = value;
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn question1_examples() {
        assert_eq!(question1("abccbd", &[0, 1, 2, 3, 4, 5]), 2);
        assert_eq!(question1("aaaa", &[3, 4, 5, 6]), 12);
        assert_eq!(question1("aabbcc", &[1, 2, 1, 2, 1, 2]), 3);
    }

    #[test]
    fn question2_example() {
        assert_eq!(question2(&[4, 9, 8, 2, 6], 3), 18);
    }
}
