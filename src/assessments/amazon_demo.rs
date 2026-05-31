//! Two Amazon demo problems.
//!
//! - [`minimal_heaviest_set`]: the smallest set of elements whose sum strictly
//!   exceeds the rest, preferring the heaviest elements.
//! - [`count_groups`]: connected components in a "who received whose book"
//!   adjacency matrix given as rows of `'0'`/`'1'`.

/// Take the largest elements until their sum outweighs the remainder; return
/// them in ascending order.
pub fn minimal_heaviest_set(arr: &[i32]) -> Vec<i32> {
    let total: i64 = arr.iter().map(|&x| i64::from(x)).sum();

    let mut sorted = arr.to_vec();
    sorted.sort_unstable();

    let mut accumulated: i64 = 0;
    for cut in (0..sorted.len()).rev() {
        accumulated += i64::from(sorted[cut]);
        if accumulated > total - accumulated {
            return sorted[cut..].to_vec();
        }
    }

    Vec::new()
}

pub fn count_groups(related: &[&str]) -> i32 {
    let rows: Vec<&[u8]> = related.iter().map(|r| r.as_bytes()).collect();
    let mut visited = vec![false; rows.len()];
    let mut groups = 0;

    for person in 0..rows.len() {
        if !visited[person] {
            visit(person, &rows, &mut visited);
            groups += 1;
        }
    }

    groups
}

fn visit(person: usize, rows: &[&[u8]], visited: &mut [bool]) {
    visited[person] = true;
    for other in 0..rows.len() {
        if other != person && !visited[other] && rows[person][other] == b'1' {
            visit(other, rows, visited);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn minimal_heaviest_set_examples() {
        assert_eq!(minimal_heaviest_set(&[5, 3, 2, 4, 1, 2]), vec![4, 5]);
        assert_eq!(minimal_heaviest_set(&[4, 2, 5, 1, 6]), vec![5, 6]);
    }

    #[test]
    fn count_groups_examples() {
        assert_eq!(count_groups(&["1100", "1110", "0110", "0001"]), 2);
        assert_eq!(
            count_groups(&["10000", "01000", "00100", "00010", "00001"]),
            5
        );
        assert_eq!(
            count_groups(&["10011", "01000", "00100", "01010", "10001"]),
            2
        );
        assert_eq!(
            count_groups(&["11111", "11111", "11111", "11111", "11111"]),
            1
        );
        assert_eq!(count_groups(&["1010", "0101", "1010", "0101"]), 2);
        assert_eq!(count_groups(&["1001", "0110", "0111", "1011"]), 1);
    }
}
