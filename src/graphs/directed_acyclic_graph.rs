//! Encode a directed acyclic graph to a flat, serializable form and decode it
//! back, restoring shared references.
//!
//! Nodes can have several parents (a DAG, not a tree), so they need shared
//! ownership: `Rc<RefCell<GraphNode>>`. Each node has a stable `id`, which the
//! flat form uses to express adjacency. (In a real app `SerializedDag` would be
//! handed to serde for JSON; here it *is* the serialization boundary, keeping
//! the crate dependency-free.)

use std::cell::RefCell;
use std::collections::{HashMap, HashSet, VecDeque};
use std::rc::Rc;

pub type NodeRef = Rc<RefCell<GraphNode>>;

#[derive(Debug)]
pub struct GraphNode {
    pub id: u64,
    pub value: i32,
    pub neighbors: Vec<NodeRef>,
}

impl GraphNode {
    pub fn new(id: u64, value: i32) -> NodeRef {
        Rc::new(RefCell::new(GraphNode {
            id,
            value,
            neighbors: Vec::new(),
        }))
    }
}

/// Flat representation: each node's `(id, value)`, each node's neighbour ids,
/// and the root id (absent for an empty graph).
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct SerializedDag {
    pub nodes: Vec<(u64, i32)>,
    pub connections: Vec<(u64, Vec<u64>)>,
    pub root_id: Option<u64>,
}

/// Breadth-first encode: visit each node once, recording its value and the ids
/// of its neighbours.
pub fn encode(start: &Option<NodeRef>) -> SerializedDag {
    let Some(start) = start else {
        return SerializedDag::default();
    };

    let mut nodes = Vec::new();
    let mut connections = Vec::new();
    let mut visited: HashSet<u64> = HashSet::new();
    let mut queue: VecDeque<NodeRef> = VecDeque::from([start.clone()]);

    while let Some(current) = queue.pop_front() {
        let node = current.borrow();
        if !visited.insert(node.id) {
            continue;
        }

        nodes.push((node.id, node.value));
        connections.push((
            node.id,
            node.neighbors.iter().map(|n| n.borrow().id).collect(),
        ));

        for neighbor in &node.neighbors {
            queue.push_back(neighbor.clone());
        }
    }

    SerializedDag {
        nodes,
        connections,
        root_id: Some(start.borrow().id),
    }
}

/// Rebuild the graph, wiring neighbours through a shared id → node map so that
/// a node referenced by several parents is a single shared instance.
pub fn decode(dag: &SerializedDag) -> Option<NodeRef> {
    let root_id = dag.root_id?;

    let mut by_id: HashMap<u64, NodeRef> = HashMap::with_capacity(dag.nodes.len());
    for &(id, value) in &dag.nodes {
        by_id.insert(id, GraphNode::new(id, value));
    }

    for (id, neighbor_ids) in &dag.connections {
        let node = by_id[id].clone();
        for neighbor_id in neighbor_ids {
            node.borrow_mut().neighbors.push(by_id[neighbor_id].clone());
        }
    }

    Some(by_id[&root_id].clone())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn connect(parent: &NodeRef, child: &NodeRef) {
        parent.borrow_mut().neighbors.push(child.clone());
    }

    /// Sorted `(id, value)` pairs, for order-independent comparison.
    fn sorted_nodes(dag: &SerializedDag) -> Vec<(u64, i32)> {
        let mut nodes = dag.nodes.clone();
        nodes.sort_unstable();
        nodes
    }

    fn neighbors_of(dag: &SerializedDag, id: u64) -> Vec<u64> {
        dag.connections
            .iter()
            .find(|(n, _)| *n == id)
            .unwrap()
            .1
            .clone()
    }

    #[test]
    fn single_node() {
        let root = GraphNode::new(1, 42);
        let dag = encode(&Some(root));

        assert_eq!(dag.root_id, Some(1));
        assert_eq!(dag.nodes, vec![(1, 42)]);
        assert_eq!(neighbors_of(&dag, 1), Vec::<u64>::new());
    }

    #[test]
    fn linear_chain_in_bfs_order() {
        let a = GraphNode::new(1, 10);
        let b = GraphNode::new(2, 20);
        let c = GraphNode::new(3, 30);
        connect(&a, &b);
        connect(&b, &c);

        let dag = encode(&Some(a));
        assert_eq!(dag.nodes, vec![(1, 10), (2, 20), (3, 30)]);
        assert_eq!(neighbors_of(&dag, 1), vec![2]);
        assert_eq!(neighbors_of(&dag, 2), vec![3]);
        assert_eq!(neighbors_of(&dag, 3), Vec::<u64>::new());
    }

    #[test]
    fn diamond_shared_descendant_encoded_once() {
        let a = GraphNode::new(1, 1);
        let b = GraphNode::new(2, 2);
        let c = GraphNode::new(3, 3);
        let d = GraphNode::new(4, 4);
        connect(&a, &b);
        connect(&a, &c);
        connect(&b, &d);
        connect(&c, &d);

        let dag = encode(&Some(a));
        assert_eq!(sorted_nodes(&dag), vec![(1, 1), (2, 2), (3, 3), (4, 4)]);
        assert_eq!(neighbors_of(&dag, 1), vec![2, 3]);
        assert_eq!(neighbors_of(&dag, 2), vec![4]);
        assert_eq!(neighbors_of(&dag, 3), vec![4]);
        assert_eq!(neighbors_of(&dag, 4), Vec::<u64>::new());
    }

    #[test]
    fn empty_graph_round_trips_to_none() {
        assert_eq!(encode(&None), SerializedDag::default());
        assert!(decode(&SerializedDag::default()).is_none());
    }

    #[test]
    fn decode_restores_shared_instances() {
        // 1 -> {2, 3}; both 2 and 3 -> 4
        let dag = SerializedDag {
            nodes: vec![(1, 1), (2, 2), (3, 3), (4, 4)],
            connections: vec![(1, vec![2, 3]), (2, vec![4]), (3, vec![4]), (4, vec![])],
            root_id: Some(1),
        };

        let root = decode(&dag).unwrap();
        let root = root.borrow();
        let d_from_b = root.neighbors[0].borrow().neighbors[0].clone();
        let d_from_c = root.neighbors[1].borrow().neighbors[0].clone();

        assert_eq!(d_from_b.borrow().value, 4);
        assert!(Rc::ptr_eq(&d_from_b, &d_from_c));
    }

    #[test]
    fn round_trip_preserves_structure_and_sharing() {
        // A(10) -> B(5), C(5), D(10); B->E, C->{E,F}, D->F; E->G, F->G
        let a = GraphNode::new(1, 10);
        let b = GraphNode::new(2, 5);
        let c = GraphNode::new(3, 5);
        let d = GraphNode::new(4, 10);
        let e = GraphNode::new(5, 3);
        let f = GraphNode::new(6, 3);
        let g = GraphNode::new(7, 5);
        connect(&a, &b);
        connect(&a, &c);
        connect(&a, &d);
        connect(&b, &e);
        connect(&c, &e);
        connect(&c, &f);
        connect(&d, &f);
        connect(&e, &g);
        connect(&f, &g);

        let decoded = decode(&encode(&Some(a))).unwrap();
        let decoded = decoded.borrow();
        assert_eq!(decoded.value, 10);
        assert_eq!(decoded.neighbors.len(), 3);

        let e_from_b = decoded.neighbors[0].borrow().neighbors[0].clone();
        let e_from_c = decoded.neighbors[1].borrow().neighbors[0].clone();
        assert!(Rc::ptr_eq(&e_from_b, &e_from_c));

        let f_from_c = decoded.neighbors[1].borrow().neighbors[1].clone();
        let f_from_d = decoded.neighbors[2].borrow().neighbors[0].clone();
        assert!(Rc::ptr_eq(&f_from_c, &f_from_d));

        let g_from_e = e_from_b.borrow().neighbors[0].clone();
        let g_from_f = f_from_c.borrow().neighbors[0].clone();
        assert!(Rc::ptr_eq(&g_from_e, &g_from_f));
    }
}
