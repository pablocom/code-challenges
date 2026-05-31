//! Merge two sorted lists into one sorted list.
//!
//! Recursion reads cleanly here: take the smaller head, then merge the rest.

use super::Link;

pub fn merge(list1: Link, list2: Link) -> Link {
    match (list1, list2) {
        (None, rest) | (rest, None) => rest,
        (Some(mut a), Some(mut b)) => {
            if a.val <= b.val {
                a.next = merge(a.next.take(), Some(b));
                Some(a)
            } else {
                b.next = merge(Some(a), b.next.take());
                Some(b)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::super::{from_slice, to_vec};
    use super::*;

    #[test]
    fn examples() {
        assert_eq!(
            to_vec(&merge(from_slice(&[1, 2, 4]), from_slice(&[1, 3, 4]))),
            vec![1, 1, 2, 3, 4, 4]
        );
        assert_eq!(
            to_vec(&merge(from_slice(&[]), from_slice(&[]))),
            Vec::<i32>::new()
        );
        assert_eq!(to_vec(&merge(from_slice(&[]), from_slice(&[0]))), vec![0]);
        assert_eq!(
            to_vec(&merge(from_slice(&[2, 3, 5]), from_slice(&[1, 4, 6]))),
            vec![1, 2, 3, 4, 5, 6]
        );
        assert_eq!(
            to_vec(&merge(
                from_slice(&[-10, -5, 0, 5]),
                from_slice(&[-2, 3, 4])
            )),
            vec![-10, -5, -2, 0, 3, 4, 5]
        );
    }
}
