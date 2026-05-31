//! Binary-tree problems.
//!
//! Most use the LeetCode-style [`TreeNode`] (`Option<Box<TreeNode>>`, aliased
//! [`Tree`]); tests build trees with the fluent [`node`] builder. The
//! self-balancing AVL tree is generic and lives in its own module.

pub mod avl_tree;
pub mod binary_tree_unique_paths_to_leaves;
pub mod delete_node_in_binary_search_tree;
pub mod good_nodes;
pub mod level_order_traversal;
pub mod lowest_common_ancestor;
pub mod nodes_equal_to_average_of_subtree;
pub mod valid_bst;

/// A binary-tree node holding an `i32` value.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Tree,
    pub right: Tree,
}

/// An owning, possibly-empty subtree.
pub type Tree = Option<Box<TreeNode>>;

/// Start building a tree rooted at `val`.
pub fn node(val: i32) -> TreeBuilder {
    TreeBuilder {
        val,
        left: None,
        right: None,
    }
}

/// Fluent builder so test trees read like their diagrams:
/// `node(1).left(node(2)).right(node(3)).build()`.
pub struct TreeBuilder {
    val: i32,
    left: Option<Box<TreeBuilder>>,
    right: Option<Box<TreeBuilder>>,
}

impl TreeBuilder {
    pub fn left(mut self, subtree: TreeBuilder) -> Self {
        self.left = Some(Box::new(subtree));
        self
    }

    pub fn right(mut self, subtree: TreeBuilder) -> Self {
        self.right = Some(Box::new(subtree));
        self
    }

    pub fn build(self) -> Tree {
        Some(Box::new(TreeNode {
            val: self.val,
            left: self.left.and_then(|b| b.build()),
            right: self.right.and_then(|b| b.build()),
        }))
    }
}
