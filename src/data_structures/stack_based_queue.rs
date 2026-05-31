//! A FIFO queue built from two LIFO stacks.
//!
//! This variant keeps the front of the queue on top of the primary stack:
//! enqueue spills everything into a helper, pushes the new item, then spills
//! back. Dequeue and peek are then O(1).

pub struct StackBasedQueue<T> {
    primary: Vec<T>,
    helper: Vec<T>,
}

impl<T> StackBasedQueue<T> {
    pub fn new() -> Self {
        Self {
            primary: Vec::new(),
            helper: Vec::new(),
        }
    }

    pub fn enqueue(&mut self, item: T) {
        while let Some(x) = self.primary.pop() {
            self.helper.push(x);
        }
        self.primary.push(item);
        while let Some(x) = self.helper.pop() {
            self.primary.push(x);
        }
    }

    pub fn dequeue(&mut self) -> Option<T> {
        self.primary.pop()
    }

    pub fn peek(&self) -> Option<&T> {
        self.primary.last()
    }
}

impl<T> Default for StackBasedQueue<T> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fifo_order() {
        let mut queue = StackBasedQueue::new();
        queue.enqueue(1);
        queue.enqueue(2);
        queue.enqueue(3);
        assert_eq!(queue.dequeue(), Some(1));
        assert_eq!(queue.dequeue(), Some(2));
        assert_eq!(queue.dequeue(), Some(3));
    }

    #[test]
    fn peek_returns_first_without_removing() {
        let mut queue = StackBasedQueue::new();
        queue.enqueue(10);
        queue.enqueue(20);
        assert_eq!(queue.peek(), Some(&10));
        assert_eq!(queue.peek(), Some(&10));
    }

    #[test]
    fn interleaved_operations() {
        let mut queue = StackBasedQueue::new();
        queue.enqueue(1);
        queue.enqueue(2);
        assert_eq!(queue.dequeue(), Some(1));
        queue.enqueue(3);
        assert_eq!(queue.dequeue(), Some(2));
        assert_eq!(queue.dequeue(), Some(3));
    }

    #[test]
    fn works_with_strings() {
        let mut queue = StackBasedQueue::new();
        queue.enqueue("a");
        queue.enqueue("b");
        assert_eq!(queue.peek(), Some(&"a"));
        assert_eq!(queue.dequeue(), Some("a"));
        assert_eq!(queue.dequeue(), Some("b"));
    }
}
