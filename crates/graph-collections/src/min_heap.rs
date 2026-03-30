/// A binary min-heap backed by a [`Vec`].
///
/// The smallest element (by [`Ord`]) is always at the top. Every push and pop
/// runs in **O(log n)**; peek is **O(1)**.
///
/// # Examples
///
/// ```
/// use graph_collections::MinHeap;
///
/// let mut heap = MinHeap::new();
/// heap.push(5u32);
/// heap.push(1);
/// heap.push(3);
///
/// assert_eq!(heap.peek(), Some(&1));
/// assert_eq!(heap.pop(), Some(1));
/// assert_eq!(heap.pop(), Some(3));
/// assert_eq!(heap.pop(), Some(5));
/// assert!(heap.is_empty());
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MinHeap<T: Ord> {
    data: Vec<T>,
}

impl<T: Ord> MinHeap<T> {
    #[inline]
    fn parent(idx: usize) -> usize {
        (idx - 1) / 2
    }

    #[inline]
    fn left(idx: usize) -> usize {
        2 * idx + 1
    }

    #[inline]
    fn right(idx: usize) -> usize {
        2 * idx + 2
    }
}

impl<T: Ord> Default for MinHeap<T> {
    fn default() -> Self {
        Self { data: Vec::new() }
    }
}

impl<T: Ord> MinHeap<T> {
    /// Creates a new, empty min-heap.
    ///
    /// # Examples
    ///
    /// ```
    /// use graph_collections::MinHeap;
    ///
    /// let heap: MinHeap<u32> = MinHeap::new();
    /// assert!(heap.is_empty());
    /// ```
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates a new, empty min-heap with at least the given capacity
    /// pre-allocated, avoiding reallocations for the first `capacity` pushes.
    ///
    /// # Examples
    ///
    /// ```
    /// use graph_collections::MinHeap;
    ///
    /// let heap: MinHeap<u32> = MinHeap::with_capacity(64);
    /// assert!(heap.is_empty());
    /// ```
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            data: Vec::with_capacity(capacity),
        }
    }
}

impl<T: Ord> MinHeap<T> {
    /// Returns `true` if the heap contains no elements.
    ///
    /// # Examples
    ///
    /// ```
    /// use graph_collections::MinHeap;
    ///
    /// let mut heap: MinHeap<u32> = MinHeap::new();
    /// assert!(heap.is_empty());
    /// heap.push(1);
    /// assert!(!heap.is_empty());
    /// ```
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    /// Returns the number of elements in the heap.
    ///
    /// # Examples
    ///
    /// ```
    /// use graph_collections::MinHeap;
    ///
    /// let mut heap: MinHeap<u32> = MinHeap::new();
    /// assert_eq!(heap.len(), 0);
    /// heap.push(1);
    /// heap.push(2);
    /// assert_eq!(heap.len(), 2);
    /// ```
    #[must_use]
    #[inline]
    pub fn len(&self) -> usize {
        self.data.len()
    }
}

impl<T: Ord> MinHeap<T> {
    /// Returns a reference to the **minimum** element without removing it,
    /// or `None` if the heap is empty.
    ///
    /// # Examples
    ///
    /// ```
    /// use graph_collections::MinHeap;
    ///
    /// let mut heap: MinHeap<u32> = MinHeap::new();
    /// assert_eq!(heap.peek(), None);
    /// heap.push(5);
    /// heap.push(1);
    /// heap.push(3);
    /// assert_eq!(heap.peek(), Some(&1)); // minimum, not removed
    /// assert_eq!(heap.len(), 3);
    /// ```
    #[must_use]
    #[inline]
    pub fn peek(&self) -> Option<&T> {
        self.data.first()
    }
}

impl<T: Ord> MinHeap<T> {
    /// Inserts `value` into the heap in **O(log n)**.
    ///
    /// After each push, `peek` returns the overall minimum.
    ///
    /// # Examples
    ///
    /// ```
    /// use graph_collections::MinHeap;
    ///
    /// let mut heap: MinHeap<u32> = MinHeap::new();
    /// heap.push(3);
    /// heap.push(1);
    /// heap.push(2);
    /// assert_eq!(heap.peek(), Some(&1));
    /// ```
    pub fn push(&mut self, value: T) {
        self.data.push(value);
        let last = self.data.len() - 1;
        self.sift_up(last);
    }

    /// Restores the heap property upward from `idx`.
    ///
    /// Algorithm:
    /// 1. While `idx > 0` and `data[idx] < data[parent(idx)]`, swap upward.
    fn sift_up(&mut self, mut idx: usize) {
        while idx > 0 {
            let p = Self::parent(idx);
            if self.data[idx] < self.data[p] {
                self.data.swap(idx, p);
                idx = p;
            } else {
                break;
            }
        }
    }
}

impl<T: Ord> MinHeap<T> {
    /// Removes and returns the **minimum** element in **O(log n)**,
    /// or `None` if the heap is empty.
    ///
    /// # Examples
    ///
    /// ```
    /// use graph_collections::MinHeap;
    ///
    /// let mut heap: MinHeap<u32> = MinHeap::new();
    /// assert_eq!(heap.pop(), None);
    /// heap.push(5);
    /// heap.push(1);
    /// heap.push(3);
    /// assert_eq!(heap.pop(), Some(1));
    /// assert_eq!(heap.pop(), Some(3));
    /// assert_eq!(heap.pop(), Some(5));
    /// assert_eq!(heap.pop(), None);
    /// ```
    pub fn pop(&mut self) -> Option<T> {
        if self.data.is_empty() {
            return None;
        }
        let last = self.data.len() - 1;
        self.data.swap(0, last);
        let min = self.data.pop();
        if !self.data.is_empty() {
            self.sift_down(0);
        }
        min
    }

    /// Restores the heap property downward from `idx`.
    ///
    /// Algorithm:
    /// 1. Find the smallest among `data[idx]`, left child, and right child.
    /// 2. If a child is smaller, swap and recurse downward.
    fn sift_down(&mut self, mut idx: usize) {
        let len = self.data.len();
        loop {
            let left = Self::left(idx);
            let right = Self::right(idx);
            let mut smallest = idx;

            if left < len && self.data[left] < self.data[smallest] {
                smallest = left;
            }
            if right < len && self.data[right] < self.data[smallest] {
                smallest = right;
            }

            if smallest == idx {
                break;
            }
            self.data.swap(idx, smallest);
            idx = smallest;
        }
    }

    /// Removes all elements from the heap.
    ///
    /// # Examples
    ///
    /// ```
    /// use graph_collections::MinHeap;
    ///
    /// let mut heap: MinHeap<u32> = (1..=5).collect();
    /// heap.clear();
    /// assert!(heap.is_empty());
    /// ```
    #[inline]
    pub fn clear(&mut self) {
        self.data.clear();
    }
}

impl<T: Ord> FromIterator<T> for MinHeap<T> {
    /// Builds a `MinHeap` from any iterator.
    ///
    /// Uses the O(n) heapify algorithm (build-heap) rather than O(n log n)
    /// successive pushes.
    ///
    /// # Examples
    ///
    /// ```
    /// use graph_collections::MinHeap;
    ///
    /// let heap: MinHeap<u32> = vec![5, 3, 1, 4, 2].into_iter().collect();
    /// assert_eq!(heap.peek(), Some(&1));
    /// assert_eq!(heap.len(), 5);
    /// ```
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let data: Vec<T> = iter.into_iter().collect();
        let mut heap = Self { data };
        // Heapify: sift down every non-leaf node from bottom to top.
        if heap.data.len() > 1 {
            let last_parent = (heap.data.len() - 2) / 2;
            for i in (0..=last_parent).rev() {
                heap.sift_down(i);
            }
        }
        heap
    }
}

impl<T: Ord> Extend<T> for MinHeap<T> {
    /// Pushes all elements produced by the iterator into the heap.
    ///
    /// # Examples
    ///
    /// ```
    /// use graph_collections::MinHeap;
    ///
    /// let mut heap: MinHeap<u32> = MinHeap::new();
    /// heap.push(10);
    /// heap.extend([2, 7, 1]);
    /// assert_eq!(heap.peek(), Some(&1));
    /// assert_eq!(heap.len(), 4);
    /// ```
    fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
        for item in iter {
            self.push(item);
        }
    }
}

/// Consuming iterator that yields elements in **ascending (sorted) order**.
///
/// Each call to `next` performs one heap pop, so the overall traversal is O(n log n).
pub struct MinHeapIntoIter<T: Ord> {
    heap: MinHeap<T>,
}

impl<T: Ord> Iterator for MinHeapIntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        self.heap.pop()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let n = self.heap.len();
        (n, Some(n))
    }
}

impl<T: Ord> IntoIterator for MinHeap<T> {
    type Item = T;
    type IntoIter = MinHeapIntoIter<T>;

    /// Consumes the heap and yields elements in **ascending order**.
    ///
    /// # Examples
    ///
    /// ```
    /// use graph_collections::MinHeap;
    ///
    /// let heap: MinHeap<u32> = vec![4, 2, 5, 1, 3].into_iter().collect();
    /// let sorted: Vec<u32> = heap.into_iter().collect();
    /// assert_eq!(sorted, [1, 2, 3, 4, 5]);
    /// ```
    fn into_iter(self) -> Self::IntoIter {
        MinHeapIntoIter { heap: self }
    }
}
