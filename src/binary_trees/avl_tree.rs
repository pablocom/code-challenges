//! A generic, self-balancing AVL tree.
//!
//! Each node tracks its height (empty = -1, leaf = 0). After every insert and
//! remove the affected path is rebalanced with single or double rotations so
//! the balance factor stays within ±1.

use std::cmp::Ordering;

type Link<T> = Option<Box<AvlNode<T>>>;

struct AvlNode<T> {
    value: T,
    height: i32,
    left: Link<T>,
    right: Link<T>,
}

impl<T> AvlNode<T> {
    fn leaf(value: T) -> Box<Self> {
        Box::new(AvlNode {
            value,
            height: 0,
            left: None,
            right: None,
        })
    }
}

#[derive(Default)]
pub struct AvlTree<T: Ord + Clone> {
    root: Link<T>,
}

impl<T: Ord + Clone> AvlTree<T> {
    pub fn new() -> Self {
        Self { root: None }
    }

    pub fn from_items<I: IntoIterator<Item = T>>(items: I) -> Self {
        let mut tree = Self::new();
        for item in items {
            tree.insert(item);
        }
        tree
    }

    /// Height of the tree; -1 when empty.
    pub fn height(&self) -> i32 {
        height_of(&self.root)
    }

    /// Balance factor of the root (left height − right height).
    pub fn balance(&self) -> i32 {
        balance_of(&self.root)
    }

    pub fn insert(&mut self, item: T) {
        self.root = Self::insert_node(self.root.take(), item);
    }

    pub fn remove(&mut self, item: &T) {
        self.root = Self::remove_node(self.root.take(), item);
    }

    /// Values in ascending order.
    pub fn in_order(&self) -> Vec<T> {
        let mut out = Vec::new();
        collect_in_order(&self.root, &mut out);
        out
    }

    fn insert_node(link: Link<T>, item: T) -> Link<T> {
        let mut node = match link {
            None => return Some(AvlNode::leaf(item)),
            Some(node) => node,
        };

        if item < node.value {
            node.left = Self::insert_node(node.left.take(), item);
        } else {
            node.right = Self::insert_node(node.right.take(), item);
        }

        Some(rebalance(node))
    }

    fn remove_node(link: Link<T>, item: &T) -> Link<T> {
        let mut node = link?;

        match item.cmp(&node.value) {
            Ordering::Less => node.left = Self::remove_node(node.left.take(), item),
            Ordering::Greater => node.right = Self::remove_node(node.right.take(), item),
            Ordering::Equal => {
                if node.left.is_none() {
                    return node.right.take();
                }
                if node.right.is_none() {
                    return node.left.take();
                }
                let successor = min_value(&node.right);
                node.value = successor.clone();
                node.right = Self::remove_node(node.right.take(), &successor);
            }
        }

        Some(rebalance(node))
    }
}

fn height_of<T>(link: &Link<T>) -> i32 {
    link.as_ref().map_or(-1, |node| node.height)
}

fn balance_of<T>(link: &Link<T>) -> i32 {
    match link {
        None => 0,
        Some(node) => height_of(&node.left) - height_of(&node.right),
    }
}

fn update_height<T>(node: &mut AvlNode<T>) {
    node.height = 1 + height_of(&node.left).max(height_of(&node.right));
}

fn rebalance<T>(mut node: Box<AvlNode<T>>) -> Box<AvlNode<T>> {
    update_height(&mut node);
    let balance = height_of(&node.left) - height_of(&node.right);

    if balance > 1 {
        if balance_of(&node.left) >= 0 {
            return rotate_right(node);
        }
        node.left = Some(rotate_left(node.left.take().unwrap()));
        return rotate_right(node);
    }

    if balance < -1 {
        if balance_of(&node.right) <= 0 {
            return rotate_left(node);
        }
        node.right = Some(rotate_right(node.right.take().unwrap()));
        return rotate_left(node);
    }

    node
}

fn rotate_right<T>(mut node: Box<AvlNode<T>>) -> Box<AvlNode<T>> {
    let mut pivot = node.left.take().unwrap();
    node.left = pivot.right.take();
    update_height(&mut node);
    pivot.right = Some(node);
    update_height(&mut pivot);
    pivot
}

fn rotate_left<T>(mut node: Box<AvlNode<T>>) -> Box<AvlNode<T>> {
    let mut pivot = node.right.take().unwrap();
    node.right = pivot.left.take();
    update_height(&mut node);
    pivot.left = Some(node);
    update_height(&mut pivot);
    pivot
}

fn min_value<T: Clone>(link: &Link<T>) -> T {
    let mut node = link.as_ref().unwrap();
    while let Some(left) = &node.left {
        node = left;
    }
    node.value.clone()
}

fn collect_in_order<T: Clone>(link: &Link<T>, out: &mut Vec<T>) {
    if let Some(node) = link {
        collect_in_order(&node.left, out);
        out.push(node.value.clone());
        collect_in_order(&node.right, out);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn starts_empty() {
        let tree: AvlTree<i32> = AvlTree::new();
        assert!(tree.in_order().is_empty());
        assert_eq!(tree.height(), -1);
    }

    #[test]
    fn builds_from_a_collection() {
        let tree = AvlTree::from_items([1, 2, 3]);
        assert_eq!(tree.in_order(), vec![1, 2, 3]);
        assert_eq!(tree.height(), 1);
    }

    #[test]
    fn inserts_first_item() {
        let mut tree = AvlTree::new();
        tree.insert(1);
        assert_eq!(tree.in_order(), vec![1]);
        assert_eq!(tree.height(), 0);
    }

    #[test]
    fn inserts_two_items_in_order() {
        let mut tree = AvlTree::new();
        tree.insert(2);
        tree.insert(1);
        assert_eq!(tree.in_order(), vec![1, 2]);
        assert_eq!(tree.height(), 1);
        assert_eq!(tree.balance(), 1);
    }

    #[test]
    fn inserts_multiple_items() {
        let mut tree = AvlTree::new();
        for item in [3, 2, 1, 4] {
            tree.insert(item);
        }
        assert_eq!(tree.in_order(), vec![1, 2, 3, 4]);
        assert_eq!(tree.height(), 2);
    }

    #[test]
    fn rebalances_on_insert() {
        let mut tree = AvlTree::new();
        for item in ['b', 'a', 'c'] {
            tree.insert(item);
        }
        assert_eq!(tree.in_order(), vec!['a', 'b', 'c']);
        assert_eq!(tree.height(), 1);
    }

    #[test]
    fn removes_and_rebalances() {
        let mut tree = AvlTree::from_items([1, 2, 3, 4]);
        tree.remove(&3);
        assert_eq!(tree.in_order(), vec![1, 2, 4]);
        assert_eq!(tree.height(), 1);
    }
}
