//! Swap every two adjacent nodes.
//!
//! Recursion mirrors the structure: swap the first pair, then recurse on the
//! remainder and hang it off the (now second) node.

use super::Link;

pub fn solve(head: Link) -> Link {
    match head {
        None => None,
        Some(mut first) => match first.next.take() {
            None => Some(first),
            Some(mut second) => {
                first.next = solve(second.next.take());
                second.next = Some(first);
                Some(second)
            }
        },
    }
}

#[cfg(test)]
mod tests {
    use super::super::{from_slice, to_vec};
    use super::*;

    #[test]
    fn even_length() {
        assert_eq!(to_vec(&solve(from_slice(&[1, 2, 3, 4]))), vec![2, 1, 4, 3]);
    }

    #[test]
    fn odd_length() {
        assert_eq!(
            to_vec(&solve(from_slice(&[1, 2, 3, 4, 5]))),
            vec![2, 1, 4, 3, 5]
        );
        assert_eq!(to_vec(&solve(from_slice(&[1, 2, 3]))), vec![2, 1, 3]);
    }

    #[test]
    fn short_lists() {
        assert_eq!(solve(None), None);
        assert_eq!(to_vec(&solve(from_slice(&[1]))), vec![1]);
    }
}
