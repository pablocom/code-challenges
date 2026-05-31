//! Remove the n-th node counting from the end of the list.
//!
//! Counting the length first turns this into "remove the node at front index
//! `len - n`". A dummy head removes the special case of deleting the first node.

use super::{Link, ListNode};

pub fn remove_nth_from_end(head: Link, n: usize) -> Link {
    let len = length(&head);
    if n == 0 || n > len {
        return head;
    }

    let remove_index = len - n;

    let mut dummy = Box::new(ListNode { val: 0, next: head });
    let mut before = &mut dummy;
    for _ in 0..remove_index {
        before = before.next.as_mut().unwrap();
    }

    if let Some(mut target) = before.next.take() {
        before.next = target.next.take();
    }

    dummy.next
}

fn length(head: &Link) -> usize {
    let mut count = 0;
    let mut cursor = head;
    while let Some(node) = cursor {
        count += 1;
        cursor = &node.next;
    }
    count
}

#[cfg(test)]
mod tests {
    use super::super::{from_slice, to_vec};
    use super::*;

    #[test]
    fn examples() {
        assert_eq!(
            to_vec(&remove_nth_from_end(from_slice(&[1, 2, 3, 4, 5]), 2)),
            vec![1, 2, 3, 5]
        );
        assert_eq!(
            to_vec(&remove_nth_from_end(from_slice(&[1, 2, 2, 3, 4, 5]), 3)),
            vec![1, 2, 2, 4, 5]
        );
        assert_eq!(remove_nth_from_end(from_slice(&[1]), 1), None);
        assert_eq!(
            to_vec(&remove_nth_from_end(from_slice(&[1, 2]), 1)),
            vec![1]
        );
        assert_eq!(
            to_vec(&remove_nth_from_end(from_slice(&[1, 2]), 2)),
            vec![2]
        );
        assert_eq!(
            to_vec(&remove_nth_from_end(from_slice(&[1, 2, 3]), 3)),
            vec![2, 3]
        );
    }
}
