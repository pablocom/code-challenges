//! Reverse a singly linked list.
//!
//! The canonical Rust idiom: pop each node off `head`, relink it onto a growing
//! `prev` chain. Ownership moves make the pointer juggling explicit and safe.

use super::Link;

pub fn solve(mut head: Link) -> Link {
    let mut prev: Link = None;

    while let Some(mut node) = head {
        head = node.next.take();
        node.next = prev;
        prev = Some(node);
    }

    prev
}

#[cfg(test)]
mod tests {
    use super::super::{from_slice, to_vec};
    use super::*;

    #[test]
    fn examples() {
        assert_eq!(
            to_vec(&solve(from_slice(&[1, 2, 3, 4, 5]))),
            vec![5, 4, 3, 2, 1]
        );
        assert_eq!(to_vec(&solve(from_slice(&[1, 2]))), vec![2, 1]);
        assert_eq!(to_vec(&solve(from_slice(&[1]))), vec![1]);
    }

    #[test]
    fn empty_list() {
        assert_eq!(to_vec(&solve(None)), Vec::<i32>::new());
    }
}
