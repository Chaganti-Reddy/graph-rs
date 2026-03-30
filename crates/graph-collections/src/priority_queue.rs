use crate::MinHeap;

/// A keyed priority queue that pops by **lowest priority first**.
///
/// Backed by [`MinHeap`] with `(P, T)` pairs. Because Rust's tuple ordering
/// is lexicographic, storing `(priority, value)` gives us priority ordering
/// for free.
///
/// # Use case
///
/// This is the queue Dijkstra's algorithm uses:
/// push a `(node, cost)` pair and always pop the cheapest node next.
///
/// # Examples
///
/// ```
/// use graph_collections::PriorityQueue;
///
/// let mut pq: PriorityQueue<&str, u32> = PriorityQueue::new();
///
/// pq.push("low",    10);
/// pq.push("high",    1);
/// pq.push("medium",  5);
///
/// assert_eq!(pq.peek_priority(), Some(&1));
///
/// assert_eq!(pq.pop(), Some(("high",   1)));
/// assert_eq!(pq.pop(), Some(("medium", 5)));
/// assert_eq!(pq.pop(), Some(("low",   10)));
/// assert!(pq.is_empty());
/// ```
#[derive(Debug, Clone)]
pub struct PriorityQueue<T: Ord, P: Ord> {
    /// Internally stores `(priority, value)` so [`MinHeap`]'s lexicographic
    /// ordering promotes the smallest priority to the top.
    heap: MinHeap<(P, T)>,
}

impl<T: Ord, P: Ord> Default for PriorityQueue<T, P> {
    fn default() -> Self {
        Self {
            heap: MinHeap::new(),
        }
    }
}

impl<T: Ord, P: Ord> PriorityQueue<T, P> {
    /// Creates a new, empty priority queue.
    ///
    /// # Examples
    ///
    /// ```
    /// use graph_collections::PriorityQueue;
    ///
    /// let pq: PriorityQueue<&str, u32> = PriorityQueue::new();
    /// assert!(pq.is_empty());
    /// ```
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates a new, empty priority queue with at least the given capacity
    /// pre-allocated.
    ///
    /// # Examples
    ///
    /// ```
    /// use graph_collections::PriorityQueue;
    ///
    /// let pq: PriorityQueue<u32, u32> = PriorityQueue::with_capacity(128);
    /// assert!(pq.is_empty());
    /// ```
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            heap: MinHeap::with_capacity(capacity),
        }
    }
}

impl<T: Ord, P: Ord> PriorityQueue<T, P> {
    /// Returns `true` if the queue contains no elements.
    ///
    /// # Examples
    ///
    /// ```
    /// use graph_collections::PriorityQueue;
    ///
    /// let mut pq: PriorityQueue<u32, u32> = PriorityQueue::new();
    /// assert!(pq.is_empty());
    /// pq.push(1, 10);
    /// assert!(!pq.is_empty());
    /// ```
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.heap.is_empty()
    }

    /// Returns the number of elements in the queue.
    ///
    /// # Examples
    ///
    /// ```
    /// use graph_collections::PriorityQueue;
    ///
    /// let mut pq: PriorityQueue<u32, u32> = PriorityQueue::new();
    /// pq.push(1, 5);
    /// pq.push(2, 3);
    /// assert_eq!(pq.len(), 2);
    /// ```
    #[must_use]
    #[inline]
    pub fn len(&self) -> usize {
        self.heap.len()
    }
}

impl<T: Ord, P: Ord> PriorityQueue<T, P> {
    /// Returns a reference to the **lowest priority** value without removing it,
    /// or `None` if empty.
    ///
    /// # Examples
    ///
    /// ```
    /// use graph_collections::PriorityQueue;
    ///
    /// let mut pq: PriorityQueue<&str, u32> = PriorityQueue::new();
    /// assert_eq!(pq.peek_priority(), None);
    /// pq.push("a", 3);
    /// pq.push("b", 1);
    /// assert_eq!(pq.peek_priority(), Some(&1));
    /// assert_eq!(pq.len(), 2); // unchanged
    /// ```
    #[must_use]
    #[inline]
    pub fn peek_priority(&self) -> Option<&P> {
        self.heap.peek().map(|(p, _)| p)
    }

    /// Returns a reference to the **value** with the lowest priority without
    /// removing it, or `None` if empty.
    ///
    /// # Examples
    ///
    /// ```
    /// use graph_collections::PriorityQueue;
    ///
    /// let mut pq: PriorityQueue<&str, u32> = PriorityQueue::new();
    /// pq.push("high", 1);
    /// pq.push("low",  9);
    /// assert_eq!(pq.peek_value(), Some(&"high"));
    /// ```
    #[must_use]
    #[inline]
    pub fn peek_value(&self) -> Option<&T> {
        self.heap.peek().map(|(_, v)| v)
    }
}

impl<T: Ord, P: Ord> PriorityQueue<T, P> {
    /// Inserts `value` with the given `priority` in **O(log n)**.
    ///
    /// Duplicate priorities are allowed; ties are broken by the natural order
    /// of `T` (because tuples are compared lexicographically).
    ///
    /// # Examples
    ///
    /// ```
    /// use graph_collections::PriorityQueue;
    ///
    /// let mut pq: PriorityQueue<&str, u32> = PriorityQueue::new();
    /// pq.push("task-a", 2);
    /// pq.push("task-b", 1);
    /// assert_eq!(pq.peek_priority(), Some(&1));
    /// ```
    #[inline]
    pub fn push(&mut self, value: T, priority: P) {
        self.heap.push((priority, value));
    }

    /// Removes and returns `(value, priority)` for the element with the
    /// **lowest priority**, or `None` if empty.
    ///
    /// # Examples
    ///
    /// ```
    /// use graph_collections::PriorityQueue;
    ///
    /// let mut pq: PriorityQueue<&str, u32> = PriorityQueue::new();
    /// assert_eq!(pq.pop(), None);
    /// pq.push("a", 3);
    /// pq.push("b", 1);
    /// pq.push("c", 2);
    /// assert_eq!(pq.pop(), Some(("b", 1)));
    /// assert_eq!(pq.pop(), Some(("c", 2)));
    /// assert_eq!(pq.pop(), Some(("a", 3)));
    /// ```
    #[inline]
    pub fn pop(&mut self) -> Option<(T, P)> {
        self.heap.pop().map(|(p, v)| (v, p))
    }

    /// Removes all elements from the queue.
    ///
    /// # Examples
    ///
    /// ```
    /// use graph_collections::PriorityQueue;
    ///
    /// let mut pq: PriorityQueue<u32, u32> = PriorityQueue::new();
    /// pq.push(1, 10);
    /// pq.push(2, 5);
    /// pq.clear();
    /// assert!(pq.is_empty());
    /// ```
    #[inline]
    pub fn clear(&mut self) {
        self.heap.clear();
    }
}

impl<T: Ord, P: Ord> FromIterator<(T, P)> for PriorityQueue<T, P> {
    /// Builds a `PriorityQueue` from an iterator of `(value, priority)` pairs.
    ///
    /// # Examples
    ///
    /// ```
    /// use graph_collections::PriorityQueue;
    ///
    /// let pq: PriorityQueue<&str, u32> = vec![
    ///     ("slow",   10u32),
    ///     ("fast",    1),
    ///     ("medium",  5),
    /// ]
    /// .into_iter()
    /// .collect();
    ///
    /// assert_eq!(pq.peek_priority(), Some(&1));
    /// assert_eq!(pq.len(), 3);
    /// ```
    fn from_iter<I: IntoIterator<Item = (T, P)>>(iter: I) -> Self {
        // Swap to (P, T) for MinHeap's ordering, then wrap.
        let heap: MinHeap<(P, T)> = iter.into_iter().map(|(v, p)| (p, v)).collect();
        Self { heap }
    }
}

impl<T: Ord, P: Ord> Extend<(T, P)> for PriorityQueue<T, P> {
    /// Pushes all `(value, priority)` pairs from the iterator into the queue.
    ///
    /// # Examples
    ///
    /// ```
    /// use graph_collections::PriorityQueue;
    ///
    /// let mut pq: PriorityQueue<&str, u32> = PriorityQueue::new();
    /// pq.push("seed", 5);
    /// pq.extend([("a", 2u32), ("b", 8), ("c", 1)]);
    /// assert_eq!(pq.peek_priority(), Some(&1));
    /// assert_eq!(pq.len(), 4);
    /// ```
    fn extend<I: IntoIterator<Item = (T, P)>>(&mut self, iter: I) {
        for (v, p) in iter {
            self.push(v, p);
        }
    }
}
