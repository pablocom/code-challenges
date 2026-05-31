//! Deep-copy a linked list whose nodes each carry an extra `random` pointer to
//! any node in the list (or none).
//!
//! Unlike the other list problems, a node here can be referenced from several
//! places (via `next` and various `random`s), so it needs shared ownership:
//! `Rc<RefCell<_>>` for the strong `next` chain, and `Weak` for `random` to
//! avoid reference cycles.
//!
//! Two classic approaches are provided: a hash-map from original → copy, and
//! the O(1)-extra-space interleaving trick.

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::{Rc, Weak};

pub type NodeRef = Rc<RefCell<RandomNode>>;

#[derive(Debug)]
pub struct RandomNode {
    pub val: i32,
    pub next: Option<NodeRef>,
    pub random: Option<Weak<RefCell<RandomNode>>>,
}

impl RandomNode {
    pub fn new(val: i32) -> NodeRef {
        Rc::new(RefCell::new(RandomNode {
            val,
            next: None,
            random: None,
        }))
    }
}

/// Identity key for a node, so two `Rc`s to the same cell map together.
fn key(node: &NodeRef) -> *const RefCell<RandomNode> {
    Rc::as_ptr(node)
}

/// Hash-map approach: clone every node, then wire `next`/`random` by looking up
/// each original's copy.
pub fn copy_random_list(head: &Option<NodeRef>) -> Option<NodeRef> {
    let mut copies: HashMap<*const RefCell<RandomNode>, NodeRef> = HashMap::new();

    let mut cursor = head.clone();
    while let Some(node) = cursor {
        copies.insert(key(&node), RandomNode::new(node.borrow().val));
        cursor = node.borrow().next.clone();
    }

    let mut cursor = head.clone();
    while let Some(node) = cursor {
        let copy = copies[&key(&node)].clone();
        let original = node.borrow();

        if let Some(next) = &original.next {
            copy.borrow_mut().next = Some(copies[&key(next)].clone());
        }
        if let Some(random) = original.random.as_ref().and_then(Weak::upgrade) {
            copy.borrow_mut().random = Some(Rc::downgrade(&copies[&key(&random)]));
        }

        cursor = original.next.clone();
    }

    head.as_ref().map(|h| copies[&key(h)].clone())
}

/// Interleaving approach: weave each copy in just after its original
/// (A → A' → B → B' …), set copies' `random`s from their neighbours, then
/// unzip the two lists.
pub fn copy_random_list_interleaved(head: &Option<NodeRef>) -> Option<NodeRef> {
    head.as_ref()?;

    // 1. Insert a copy after each original node.
    let mut cursor = head.clone();
    while let Some(node) = cursor {
        let next = node.borrow().next.clone();
        let copy = RandomNode::new(node.borrow().val);
        copy.borrow_mut().next = next.clone();
        node.borrow_mut().next = Some(copy);
        cursor = next;
    }

    // 2. A copy's random is the copy of the original's random, i.e. its
    //    `random.next`.
    let mut cursor = head.clone();
    while let Some(node) = cursor {
        let copy = node.borrow().next.clone().unwrap();
        if let Some(random) = node.borrow().random.as_ref().and_then(Weak::upgrade) {
            let random_copy = random.borrow().next.clone();
            copy.borrow_mut().random = random_copy.as_ref().map(Rc::downgrade);
        }
        cursor = copy.borrow().next.clone();
    }

    // 3. Detach the copies, restoring the originals' `next` pointers.
    let new_head = head.as_ref().unwrap().borrow().next.clone();
    let mut cursor = head.clone();
    while let Some(node) = cursor {
        let copy = node.borrow().next.clone().unwrap();
        let next_original = copy.borrow().next.clone();
        node.borrow_mut().next = next_original.clone();
        copy.borrow_mut().next = next_original.as_ref().and_then(|n| n.borrow().next.clone());
        cursor = next_original;
    }

    new_head
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Build a list from `(value, random_index)` pairs.
    fn build(spec: &[(i32, Option<usize>)]) -> Option<NodeRef> {
        let nodes: Vec<NodeRef> = spec.iter().map(|&(v, _)| RandomNode::new(v)).collect();
        for i in 0..nodes.len() {
            if i + 1 < nodes.len() {
                nodes[i].borrow_mut().next = Some(nodes[i + 1].clone());
            }
            if let Some(r) = spec[i].1 {
                nodes[i].borrow_mut().random = Some(Rc::downgrade(&nodes[r]));
            }
        }
        nodes.first().cloned()
    }

    /// Flatten to `(value, random_index)` so two lists can be compared by shape.
    fn layout(head: &Option<NodeRef>) -> Vec<(i32, Option<usize>)> {
        let mut index_of: HashMap<*const RefCell<RandomNode>, usize> = HashMap::new();
        let mut nodes = Vec::new();
        let mut cursor = head.clone();
        while let Some(node) = cursor {
            index_of.insert(key(&node), nodes.len());
            let next = node.borrow().next.clone();
            nodes.push(node);
            cursor = next;
        }
        nodes
            .iter()
            .map(|node| {
                let random_index = node
                    .borrow()
                    .random
                    .as_ref()
                    .and_then(Weak::upgrade)
                    .map(|r| index_of[&key(&r)]);
                (node.borrow().val, random_index)
            })
            .collect()
    }

    fn nodes_are_distinct(a: &Option<NodeRef>, b: &Option<NodeRef>) -> bool {
        let collect = |head: &Option<NodeRef>| {
            let mut ptrs = Vec::new();
            let mut cursor = head.clone();
            while let Some(node) = cursor {
                ptrs.push(key(&node));
                cursor = node.borrow().next.clone();
            }
            ptrs
        };
        let (pa, pb) = (collect(a), collect(b));
        pa.iter().all(|p| !pb.contains(p))
    }

    const SPEC: &[(i32, Option<usize>)] = &[(6, Some(1)), (32, Some(0))];

    #[test]
    fn hash_map_copy_is_a_faithful_deep_copy() {
        let original = build(SPEC);
        let copy = copy_random_list(&original);

        assert_eq!(layout(&copy), SPEC.to_vec());
        assert!(nodes_are_distinct(&original, &copy));
    }

    #[test]
    fn interleaved_copy_is_a_faithful_deep_copy() {
        let original = build(SPEC);
        let copy = copy_random_list_interleaved(&original);

        assert_eq!(layout(&copy), SPEC.to_vec());
        // The original list must be intact afterward.
        assert_eq!(layout(&original), SPEC.to_vec());
        assert!(nodes_are_distinct(&original, &copy));
    }

    #[test]
    fn empty_list_copies_to_empty() {
        assert!(copy_random_list(&None).is_none());
        assert!(copy_random_list_interleaved(&None).is_none());
    }

    #[test]
    fn random_pointing_to_none() {
        let spec = &[(1, None), (2, None), (3, None)];
        let original = build(spec);
        assert_eq!(layout(&copy_random_list(&original)), spec.to_vec());
    }
}
