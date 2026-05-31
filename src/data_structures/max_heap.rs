//! A binary max-heap backed by a `Vec`, built from scratch (rather than reusing
//! the standard `BinaryHeap`) to exercise the sift-up / sift-down mechanics.

pub struct MaxHeap<T: Ord> {
    heap: Vec<T>,
}

impl<T: Ord> MaxHeap<T> {
    pub fn new() -> Self {
        Self { heap: Vec::new() }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            heap: Vec::with_capacity(capacity),
        }
    }

    /// Build a heap from existing items by heapifying in O(n).
    pub fn from_vec(items: Vec<T>) -> Self {
        let mut heap = Self { heap: items };
        for index in (0..heap.heap.len() / 2).rev() {
            heap.sift_down(index);
        }
        heap
    }

    pub fn len(&self) -> usize {
        self.heap.len()
    }

    pub fn is_empty(&self) -> bool {
        self.heap.is_empty()
    }

    pub fn peek_max(&self) -> Option<&T> {
        self.heap.first()
    }

    pub fn insert(&mut self, value: T) {
        self.heap.push(value);
        self.sift_up(self.heap.len() - 1);
    }

    pub fn pop_max(&mut self) -> Option<T> {
        if self.heap.is_empty() {
            return None;
        }
        let last = self.heap.len() - 1;
        self.heap.swap(0, last);
        let max = self.heap.pop();
        if !self.heap.is_empty() {
            self.sift_down(0);
        }
        max
    }

    fn sift_up(&mut self, mut index: usize) {
        while index != 0 {
            let parent = (index - 1) / 2;
            if self.heap[parent] >= self.heap[index] {
                break;
            }
            self.heap.swap(parent, index);
            index = parent;
        }
    }

    fn sift_down(&mut self, mut index: usize) {
        let len = self.heap.len();
        loop {
            let mut largest = index;
            let left = 2 * index + 1;
            let right = 2 * index + 2;

            if left < len && self.heap[left] > self.heap[largest] {
                largest = left;
            }
            if right < len && self.heap[right] > self.heap[largest] {
                largest = right;
            }
            if largest == index {
                break;
            }

            self.heap.swap(index, largest);
            index = largest;
        }
    }
}

impl<T: Ord> Default for MaxHeap<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Ord> FromIterator<T> for MaxHeap<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        Self::from_vec(iter.into_iter().collect())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_heap_property(heap: &[i32]) {
        for parent in 0..heap.len() {
            for child in [2 * parent + 1, 2 * parent + 2] {
                if child < heap.len() {
                    assert!(
                        heap[parent] >= heap[child],
                        "parent {} < child {}",
                        heap[parent],
                        heap[child]
                    );
                }
            }
        }
    }

    fn from(values: &[i32]) -> MaxHeap<i32> {
        let mut heap = MaxHeap::new();
        for &v in values {
            heap.insert(v);
        }
        heap
    }

    #[test]
    fn insert_maintains_max_at_root() {
        assert_eq!(from(&[10]).peek_max(), Some(&10));
        assert_eq!(from(&[1, 2, 3]).peek_max(), Some(&3));
        assert_eq!(from(&[3, 2, 1]).peek_max(), Some(&3));
    }

    #[test]
    fn insert_satisfies_heap_property() {
        let heap = from(&[5, 3, 8, 1, 10, 2, 7]);
        assert_eq!(heap.len(), 7);
        assert_eq!(heap.peek_max(), Some(&10));
        assert_heap_property(&heap.heap);
    }

    #[test]
    fn heapify_constructor() {
        let heap = MaxHeap::from_vec(vec![3, 1, 6, 5, 2, 4]);
        assert_eq!(heap.peek_max(), Some(&6));
        assert_heap_property(&heap.heap);

        assert!(MaxHeap::<i32>::from_vec(vec![]).is_empty());
        assert_eq!(MaxHeap::from_vec(vec![42]).peek_max(), Some(&42));
    }

    #[test]
    fn heapify_preserves_all_elements() {
        let mut heap = MaxHeap::from_vec(vec![9, 1, 5, 3, 8, 2]);
        let mut drained = Vec::new();
        while let Some(max) = heap.pop_max() {
            drained.push(max);
        }
        drained.sort_unstable();
        assert_eq!(drained, vec![1, 2, 3, 5, 8, 9]);
    }

    #[test]
    fn peek_does_not_remove() {
        let heap = from(&[5, 10, 3]);
        assert_eq!(heap.peek_max(), Some(&10));
        assert_eq!(heap.peek_max(), Some(&10));
        assert_eq!(heap.len(), 3);
    }

    #[test]
    fn pop_on_empty_is_none() {
        assert_eq!(MaxHeap::<i32>::new().pop_max(), None);
        assert_eq!(MaxHeap::<i32>::new().peek_max(), None);
    }

    #[test]
    fn consecutive_pops_return_descending() {
        let mut heap = from(&[4, 1, 7, 3, 9]);
        let mut drained = Vec::new();
        while let Some(max) = heap.pop_max() {
            drained.push(max);
        }
        assert_eq!(drained, vec![9, 7, 4, 3, 1]);
    }

    #[test]
    fn insert_after_drain_works() {
        let mut heap = from(&[1]);
        heap.pop_max();
        heap.insert(99);
        assert_eq!(heap.peek_max(), Some(&99));
    }
}
