//! Validate a Binary Search Tree by carrying an open `(low, high)` bound down
//! to each node. Shown both breadth-first and depth-first.

use super::Tree;
use std::collections::VecDeque;

pub fn is_valid_bst_bfs(root: &Tree) -> bool {
    let mut queue: VecDeque<(&Tree, i64, i64)> = VecDeque::new();
    queue.push_back((root, i64::MIN, i64::MAX));

    while let Some((link, low, high)) = queue.pop_front() {
        let Some(node) = link else {
            continue;
        };
        let value = node.val as i64;
        if value <= low || value >= high {
            return false;
        }
        queue.push_back((&node.right, value, high));
        queue.push_back((&node.left, low, value));
    }

    true
}

pub fn is_valid_bst_dfs(root: &Tree) -> bool {
    let mut stack: Vec<(&Tree, i64, i64)> = vec![(root, i64::MIN, i64::MAX)];

    while let Some((link, low, high)) = stack.pop() {
        let Some(node) = link else {
            continue;
        };
        let value = node.val as i64;
        if value <= low || value >= high {
            return false;
        }
        stack.push((&node.right, value, high));
        stack.push((&node.left, low, value));
    }

    true
}

#[cfg(test)]
mod tests {
    use super::super::node;
    use super::*;

    #[test]
    fn valid_tree_passes_both() {
        let root = node(1).left(node(0).left(node(-2))).right(node(3)).build();
        assert!(is_valid_bst_bfs(&root));
        assert!(is_valid_bst_dfs(&root));
    }

    #[test]
    fn out_of_order_tree_fails_both() {
        // 5's right child 4 is smaller than 5 — invalid.
        let root = node(5)
            .left(node(1))
            .right(node(4).left(node(3)).right(node(6)))
            .build();
        assert!(!is_valid_bst_bfs(&root));
        assert!(!is_valid_bst_dfs(&root));
    }
}
