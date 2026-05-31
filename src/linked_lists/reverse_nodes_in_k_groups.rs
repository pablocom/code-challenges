//! Reverse the nodes of a list `k` at a time; a trailing group shorter than
//! `k` is left as-is.
//!
//! Recursively: if at least `k` nodes remain, reverse the first `k`, recurse on
//! the rest, then connect the reversed group's tail (the original first node)
//! to the recursive result.

use super::Link;

pub fn reverse_k_group(head: Link, k: usize) -> Link {
    if k <= 1 || !has_at_least_k_nodes(&head, k) {
        return head;
    }

    let mut reversed_group: Link = None;
    let mut rest = head;
    for _ in 0..k {
        let mut node = rest.unwrap();
        rest = node.next.take();
        node.next = reversed_group;
        reversed_group = Some(node);
    }

    let group_tail = last_node_mut(&mut reversed_group);
    group_tail.next = reverse_k_group(rest, k);

    reversed_group
}

fn has_at_least_k_nodes(head: &Link, k: usize) -> bool {
    let mut node = head;
    for _ in 0..k {
        match node {
            Some(next) => node = &next.next,
            None => return false,
        }
    }
    true
}

fn last_node_mut(group: &mut Link) -> &mut super::ListNode {
    let mut tail = group;
    while tail.as_ref().unwrap().next.is_some() {
        tail = &mut tail.as_mut().unwrap().next;
    }
    tail.as_mut().unwrap()
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
