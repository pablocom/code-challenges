//! Lowest common ancestor of two values in a binary tree.
//!
//! If `p` or `q` is found at a node, that node is an answer for its subtree.
//! A node whose two subtrees each contain one of the targets is the LCA.

use super::{Tree, TreeNode};

pub fn lowest_common_ancestor(root: &Tree, p: i32, q: i32) -> Option<&TreeNode> {
    let node = root.as_deref()?;

    if node.val == p || node.val == q {
        return Some(node);
    }

    let left = lowest_common_ancestor(&node.left, p, q);
    let right = lowest_common_ancestor(&node.right, p, q);

    match (left, right) {
        (Some(_), Some(_)) => Some(node),
        (found, None) | (None, found) => found,
    }
}

#[cfg(test)]
mod tests {
    use super::super::node;
    use super::*;

    fn sample() -> Tree {
        node(3)
            .left(
                node(5)
                    .left(node(6))
                    .right(node(2).left(node(7)).right(node(4))),
            )
            .right(node(1).left(node(0)).right(node(8)))
            .build()
    }

    #[test]
    fn ancestor_is_root() {
        let tree = sample();
        assert_eq!(lowest_common_ancestor(&tree, 5, 1).unwrap().val, 3);
    }

    #[test]
    fn ancestor_is_an_inner_node() {
        let tree = sample();
        assert_eq!(lowest_common_ancestor(&tree, 6, 4).unwrap().val, 5);
    }

    #[test]
    fn ancestor_in_right_subtree() {
        let tree = node(3).right(node(1).left(node(0)).right(node(8))).build();
        assert_eq!(lowest_common_ancestor(&tree, 0, 8).unwrap().val, 1);
    }
}
