//! A stack that also reports its minimum in O(1).
//!
//! A second stack tracks minimums: a value is pushed onto it whenever it is
//! less than or equal to the current minimum, and popped in lock-step.

pub struct MinStack {
    stack: Vec<i32>,
    minimums: Vec<i32>,
}

impl MinStack {
    pub fn new() -> Self {
        Self {
            stack: Vec::new(),
            minimums: Vec::new(),
        }
    }

    pub fn push(&mut self, value: i32) {
        if value <= self.get_min() {
            self.minimums.push(value);
        }
        self.stack.push(value);
    }

    pub fn pop(&mut self) {
        if let Some(value) = self.stack.pop() {
            if self.minimums.last() == Some(&value) {
                self.minimums.pop();
            }
        }
    }

    pub fn top(&self) -> i32 {
        *self.stack.last().expect("top of an empty stack")
    }

    pub fn get_min(&self) -> i32 {
        self.minimums.last().copied().unwrap_or(i32::MAX)
    }
}

impl Default for MinStack {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn push_and_get_min() {
        let mut stack = MinStack::new();
        stack.push(3);
        stack.push(5);
        stack.push(2);
        assert_eq!(stack.get_min(), 2);
    }

    #[test]
    fn pop_updates_min() {
        let mut stack = MinStack::new();
        stack.push(3);
        stack.push(2);
        stack.push(1);

        stack.pop();
        assert_eq!(stack.get_min(), 2);
        stack.pop();
        assert_eq!(stack.get_min(), 3);
    }

    #[test]
    fn top_returns_last_pushed() {
        let mut stack = MinStack::new();
        stack.push(10);
        stack.push(20);
        assert_eq!(stack.top(), 20);
    }

    #[test]
    fn min_with_duplicates() {
        let mut stack = MinStack::new();
        stack.push(1);
        stack.push(1);
        stack.pop();
        assert_eq!(stack.get_min(), 1);
    }

    #[test]
    fn negative_values() {
        let mut stack = MinStack::new();
        stack.push(-2);
        stack.push(0);
        stack.push(-3);

        assert_eq!(stack.get_min(), -3);
        stack.pop();
        assert_eq!(stack.top(), 0);
        assert_eq!(stack.get_min(), -2);
    }

    #[test]
    fn single_element() {
        let mut stack = MinStack::new();
        stack.push(42);
        assert_eq!(stack.top(), 42);
        assert_eq!(stack.get_min(), 42);
    }
}
