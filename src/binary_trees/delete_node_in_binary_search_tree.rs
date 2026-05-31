//! Delete a key from a BST, keeping it a valid BST.
//!
//! Recurse to the target; if it has two children, replace its value with the
//! smallest value in the right subtree and delete that successor instead.

use super::Tree;

pub fn delete_node(root: Tree, key: i32) -> Tree {
    let mut node = root?;

    if key < node.val {
        node.left = delete_node(node.left.take(), key);
    } else if key > node.val {
        node.right = delete_node(node.right.take(), key);
    } else {
        if node.left.is_none() {
            return node.right.take();
        }
        if node.right.is_none() {
            return node.left.take();
        }
        let successor = min_value(&node.right);
        node.val = successor;
        node.right = delete_node(node.right.take(), successor);
    }

    Some(node)
}

fn min_value(link: &Tree) -> i32 {
    let mut node = link.as_ref().unwrap();
    while let Some(left) = &node.left {
        node = left;
    }
    node.val
}

#[cfg(test)]
mod tests {
    use super::super::node;
    use super::*;

    #[test]
    fn deletes_node_with_two_children() {
        let tree = node(5)
            .left(node(3).left(node(2)).right(node(4)))
            .right(node(6).right(node(7)))
            .build();

        let root = delete_node(tree, 3).unwrap();
        assert_eq!(root.left.unwrap().val, 4);
    }

    #[test]
    fn successor_comes_from_right_subtree() {
        let tree = node(10)
            .left(
                node(5)
                    .left(node(2))
                    .right(node(8).left(node(7)).right(node(9))),
            )
            .right(node(17))
            .build();

        let root = delete_node(tree, 5).unwrap();
        assert_eq!(root.left.unwrap().val, 7);
    }
}
