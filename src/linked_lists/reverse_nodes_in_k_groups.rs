//! Reverse the nodes of a list `k` at a time; a trailing group shorter than
//! `k` is left as-is.
//!
//! Recursively: if at least `k` nodes remain, reverse the first `k`, recurse on
//! the rest, then connect the reversed group's tail (the original first node)
//! to the recursive result.

use super::Link;

pub fn reverse_k_group(head: Link, k: usize) -> Link {
    if k <= 1 {
        return head;
    }

    // Bail out (unchanged) if fewer than `k` nodes remain.
    let mut probe = &head;
    for _ in 0..k {
        match probe {
            Some(node) => probe = &node.next,
            None => return head,
        }
    }

    // Reverse the first `k` nodes.
    let mut prev: Link = None;
    let mut curr = head;
    for _ in 0..k {
        let mut node = curr.unwrap();
        curr = node.next.take();
        node.next = prev;
        prev = Some(node);
    }

    // `prev` heads the reversed group; its tail is the original first node.
    let reversed_rest = reverse_k_group(curr, k);

    let mut tail = &mut prev;
    while tail.as_ref().unwrap().next.is_some() {
        tail = &mut tail.as_mut().unwrap().next;
    }
    tail.as_mut().unwrap().next = reversed_rest;

    prev
}

#[cfg(test)]
mod tests {
    use super::super::{from_slice, to_vec};
    use super::*;

    #[test]
    fn examples() {
        assert_eq!(
            to_vec(&reverse_k_group(from_slice(&[1, 2, 3, 4, 5]), 2)),
            vec![2, 1, 4, 3, 5]
        );
        assert_eq!(
            to_vec(&reverse_k_group(from_slice(&[1, 2, 3, 4, 5]), 1)),
            vec![1, 2, 3, 4, 5]
        );
        assert_eq!(
            to_vec(&reverse_k_group(from_slice(&[1, 2, 3, 4, 5, 6]), 2)),
            vec![2, 1, 4, 3, 6, 5]
        );
        assert_eq!(
            to_vec(&reverse_k_group(from_slice(&[1, 2, 3, 4, 5, 6]), 7)),
            vec![1, 2, 3, 4, 5, 6]
        );
        assert_eq!(
            to_vec(&reverse_k_group(from_slice(&[1, 2, 3, 4, 5, 6, 7]), 3)),
            vec![3, 2, 1, 6, 5, 4, 7]
        );
        assert_eq!(
            to_vec(&reverse_k_group(from_slice(&[1, 2, 3, 4, 5, 6, 7]), 4)),
            vec![4, 3, 2, 1, 5, 6, 7]
        );
    }
}
