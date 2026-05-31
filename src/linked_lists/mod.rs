//! Singly linked-list problems.
//!
//! The shared node type is the idiomatic Rust shape `Option<Box<ListNode>>`,
//! aliased as [`Link`]. Building lists from a slice and reading them back as a
//! `Vec` keeps the tests readable. The random-pointer copy problem needs shared
//! ownership, so it lives in its own module with an `Rc<RefCell<_>>` node.

pub mod add_two_numbers;
pub mod copy_list_with_random_pointer;
pub mod merge_k_sorted_lists;
pub mod merge_two_sorted_lists;
pub mod remove_nth_from_end;
pub mod reverse_linked_list;
pub mod reverse_nodes_in_k_groups;
pub mod swap_pairs;

/// A LeetCode-style singly linked-list node.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ListNode {
    pub val: i32,
    pub next: Link,
}

/// The owning link between nodes: either nothing, or a heap-allocated node.
pub type Link = Option<Box<ListNode>>;

impl ListNode {
    pub fn new(val: i32) -> Self {
        Self { val, next: None }
    }
}

/// Build a list from a slice; the first element becomes the head.
pub fn from_slice(values: &[i32]) -> Link {
    let mut head: Link = None;
    for &value in values.iter().rev() {
        head = Some(Box::new(ListNode {
            val: value,
            next: head,
        }));
    }
    head
}

/// Collect a list's values into a `Vec`, head first.
pub fn to_vec(head: &Link) -> Vec<i32> {
    let mut out = Vec::new();
    let mut cursor = head;
    while let Some(node) = cursor {
        out.push(node.val);
        cursor = &node.next;
    }
    out
}
