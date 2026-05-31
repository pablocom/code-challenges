//! Least-Recently-Used cache with O(1) `get`/`put`.
//!
//! A hash map gives O(1) lookup; recency ordering comes from a doubly linked
//! list. Rather than `Rc<RefCell>`, the list lives in an arena (`Vec<Node>`)
//! addressed by index, with two sentinel nodes (head and tail) and a free list
//! for recycling evicted slots.

use std::collections::HashMap;

const HEAD: usize = 0;
const TAIL: usize = 1;

struct Node {
    key: i32,
    value: i32,
    prev: usize,
    next: usize,
}

pub struct LruCache {
    capacity: usize,
    by_key: HashMap<i32, usize>,
    nodes: Vec<Node>,
    free: Vec<usize>,
}

impl LruCache {
    pub fn new(capacity: usize) -> Self {
        let head = Node {
            key: 0,
            value: 0,
            prev: TAIL,
            next: TAIL,
        };
        let tail = Node {
            key: 0,
            value: 0,
            prev: HEAD,
            next: HEAD,
        };
        Self {
            capacity,
            by_key: HashMap::new(),
            nodes: vec![head, tail],
            free: Vec::new(),
        }
    }

    pub fn get(&mut self, key: i32) -> i32 {
        match self.by_key.get(&key) {
            Some(&index) => {
                self.unlink(index);
                self.push_front(index);
                self.nodes[index].value
            }
            None => -1,
        }
    }

    pub fn put(&mut self, key: i32, value: i32) {
        if let Some(&index) = self.by_key.get(&key) {
            self.nodes[index].value = value;
            self.unlink(index);
            self.push_front(index);
            return;
        }

        if self.by_key.len() == self.capacity {
            let lru = self.nodes[TAIL].prev;
            self.unlink(lru);
            self.by_key.remove(&self.nodes[lru].key);
            self.free.push(lru);
        }

        let index = self.allocate(key, value);
        self.push_front(index);
        self.by_key.insert(key, index);
    }

    fn allocate(&mut self, key: i32, value: i32) -> usize {
        let node = Node {
            key,
            value,
            prev: HEAD,
            next: HEAD,
        };
        if let Some(index) = self.free.pop() {
            self.nodes[index] = node;
            index
        } else {
            self.nodes.push(node);
            self.nodes.len() - 1
        }
    }

    fn unlink(&mut self, index: usize) {
        let (prev, next) = (self.nodes[index].prev, self.nodes[index].next);
        self.nodes[prev].next = next;
        self.nodes[next].prev = prev;
    }

    fn push_front(&mut self, index: usize) {
        let first = self.nodes[HEAD].next;
        self.nodes[index].prev = HEAD;
        self.nodes[index].next = first;
        self.nodes[HEAD].next = index;
        self.nodes[first].prev = index;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn most_recently_used_stays() {
        let mut lru = LruCache::new(2);
        lru.put(1, 1);
        lru.put(2, 2);
        assert_eq!(lru.get(1), 1);
    }

    #[test]
    fn least_recently_used_is_evicted() {
        let mut lru = LruCache::new(2);
        lru.put(1, 1);
        lru.put(2, 2);
        lru.get(1); // 1 becomes most-recent, so 2 is the LRU
        lru.put(3, 3); // evicts 2
        assert_eq!(lru.get(2), -1);
    }

    #[test]
    fn full_leetcode_sequence() {
        let mut lru = LruCache::new(2);
        lru.put(1, 1);
        lru.put(2, 2);
        lru.get(1);
        lru.put(3, 3); // evicts 2
        lru.get(2);
        lru.put(4, 4); // evicts 1
        assert_eq!(lru.get(1), -1);
        assert_eq!(lru.get(3), 3);
        assert_eq!(lru.get(4), 4);
    }

    #[test]
    fn put_updates_existing_value() {
        let mut lru = LruCache::new(2);
        lru.put(1, 1);
        lru.put(1, 10);
        assert_eq!(lru.get(1), 10);
    }
}
