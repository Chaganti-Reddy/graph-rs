use std::collections::VecDeque;

/// A FIFO queue backed by [`VecDeque`].
///
/// Provides O(1) enqueue and dequeue operations. Iteration order is
/// front-to-back (i.e. the next element to be dequeued comes first).
///
/// # Examples
///
/// ```
/// use graph_collections::Queue;
///
/// let mut queue: Queue<u32> = Queue::new();
/// queue.enqueue(1);
/// queue.enqueue(2);
/// queue.enqueue(3);
///
/// assert_eq!(queue.front(), Some(&1));
/// assert_eq!(queue.dequeue(), Some(1));
/// assert_eq!(queue.len(), 2);
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Queue<T> {
    data: VecDeque<T>,
}

impl<T> Default for Queue<T> {
    fn default() -> Self {
        Self {
            data: VecDeque::new(),
        }
    }
}

impl<T> Queue<T> {
    /// Creates a new, empty queue.
    ///
    /// # Examples
    ///
    /// ```
    /// use graph_collections::Queue;
    ///
    /// let queue: Queue<u32> = Queue::new();
    /// assert!(queue.is_empty());
    /// ```
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates a new, empty queue with at least the given capacity
    /// pre-allocated, avoiding reallocations for the first `capacity` pushes.
    ///
    /// # Examples
    ///
    /// ```
    /// use graph_collections::Queue;
    ///
    /// let mut queue: Queue<u32> = Queue::with_capacity(16);
    /// assert!(queue.is_empty());
    /// ```
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            data: VecDeque::with_capacity(capacity),
        }
    }
}

impl<T> Queue<T> {
    /// Returns `true` if the queue contains no elements.
    ///
    /// # Examples
    ///
    /// ```
    /// use graph_collections::Queue;
    ///
    /// let mut queue: Queue<u32> = Queue::new();
    /// assert!(queue.is_empty());
    /// queue.enqueue(1);
    /// assert!(!queue.is_empty());
    /// ```
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    /// Returns the number of elements in the queue.
    ///
    /// # Examples
    ///
    /// ```
    /// use graph_collections::Queue;
    ///
    /// let mut queue: Queue<u32> = Queue::new();
    /// assert_eq!(queue.len(), 0);
    /// queue.enqueue(42);
    /// assert_eq!(queue.len(), 1);
    /// ```
    #[must_use]
    #[inline]
    pub fn len(&self) -> usize {
        self.data.len()
    }

    /// Returns a reference to the element at the **front** of the queue
    /// (the next one to be dequeued), or `None` if the queue is empty.
    ///
    /// # Examples
    ///
    /// ```
    /// use graph_collections::Queue;
    ///
    /// let mut queue: Queue<u32> = Queue::new();
    /// assert_eq!(queue.front(), None);
    /// queue.enqueue(1);
    /// queue.enqueue(2);
    /// assert_eq!(queue.front(), Some(&1));
    /// ```
    #[must_use]
    #[inline]
    pub fn front(&self) -> Option<&T> {
        self.data.front()
    }

    /// Returns a reference to the element at the **back** of the queue
    /// (the most recently enqueued one), or `None` if the queue is empty.
    ///
    /// # Examples
    ///
    /// ```
    /// use graph_collections::Queue;
    ///
    /// let mut queue: Queue<u32> = Queue::new();
    /// assert_eq!(queue.back(), None);
    /// queue.enqueue(1);
    /// queue.enqueue(2);
    /// assert_eq!(queue.back(), Some(&2));
    /// ```
    #[must_use]
    #[inline]
    pub fn back(&self) -> Option<&T> {
        self.data.back()
    }

    /// Adds `element` to the **back** of the queue.
    ///
    /// # Examples
    ///
    /// ```
    /// use graph_collections::Queue;
    ///
    /// let mut queue: Queue<u32> = Queue::new();
    /// queue.enqueue(1);
    /// queue.enqueue(2);
    /// assert_eq!(queue.front(), Some(&1));
    /// assert_eq!(queue.back(),  Some(&2));
    /// ```
    #[inline]
    pub fn enqueue(&mut self, element: T) {
        self.data.push_back(element);
    }

    /// Removes and returns the element at the **front** of the queue,
    /// or `None` if the queue is empty.
    ///
    /// # Examples
    ///
    /// ```
    /// use graph_collections::Queue;
    ///
    /// let mut queue: Queue<u32> = Queue::new();
    /// assert_eq!(queue.dequeue(), None);
    /// queue.enqueue(1);
    /// queue.enqueue(2);
    /// assert_eq!(queue.dequeue(), Some(1));
    /// assert_eq!(queue.dequeue(), Some(2));
    /// assert_eq!(queue.dequeue(), None);
    /// ```
    #[inline]
    pub fn dequeue(&mut self) -> Option<T> {
        self.data.pop_front()
    }

    /// Removes all elements from the queue.
    ///
    /// # Examples
    ///
    /// ```
    /// use graph_collections::Queue;
    ///
    /// let mut queue: Queue<u32> = Queue::new();
    /// queue.enqueue(1);
    /// queue.enqueue(2);
    /// queue.clear();
    /// assert!(queue.is_empty());
    /// ```
    #[inline]
    pub fn clear(&mut self) {
        self.data.clear();
    }

    /// Returns a front-to-back iterator over references to the elements.
    ///
    /// The queue is not modified.
    ///
    /// # Examples
    ///
    /// ```
    /// use graph_collections::Queue;
    ///
    /// let mut queue: Queue<u32> = Queue::new();
    /// queue.enqueue(1);
    /// queue.enqueue(2);
    /// queue.enqueue(3);
    ///
    /// let values: Vec<u32> = queue.iter().copied().collect();
    /// assert_eq!(values, [1, 2, 3]);
    /// ```
    #[inline]
    pub fn iter(&self) -> std::collections::vec_deque::Iter<'_, T> {
        self.data.iter()
    }
}

impl<T> FromIterator<T> for Queue<T> {
    /// Builds a `Queue` from any iterator. Elements are enqueued in iteration order.
    ///
    /// # Examples
    ///
    /// ```
    /// use graph_collections::Queue;
    ///
    /// let queue: Queue<u32> = (1..=3).collect();
    /// assert_eq!(queue.front(), Some(&1));
    /// assert_eq!(queue.len(),   3);
    /// ```
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        Self {
            data: VecDeque::from_iter(iter),
        }
    }
}

impl<T> Extend<T> for Queue<T> {
    /// Enqueues all elements produced by the iterator.
    ///
    /// # Examples
    ///
    /// ```
    /// use graph_collections::Queue;
    ///
    /// let mut queue: Queue<u32> = Queue::new();
    /// queue.extend([1, 2, 3]);
    /// assert_eq!(queue.len(), 3);
    /// ```
    fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
        self.data.extend(iter);
    }
}

/// Consuming iterator: `for x in queue`
impl<T> IntoIterator for Queue<T> {
    type Item = T;
    type IntoIter = std::collections::vec_deque::IntoIter<T>;

    /// Consumes the queue and yields elements front-to-back.
    ///
    /// # Examples
    ///
    /// ```
    /// use graph_collections::Queue;
    ///
    /// let queue: Queue<u32> = (1..=3).collect();
    /// let values: Vec<u32> = queue.into_iter().collect();
    /// assert_eq!(values, [1, 2, 3]);
    /// ```
    fn into_iter(self) -> Self::IntoIter {
        self.data.into_iter()
    }
}

/// Borrowing iterator: `for x in &queue`
impl<'a, T> IntoIterator for &'a Queue<T> {
    type Item = &'a T;
    type IntoIter = std::collections::vec_deque::Iter<'a, T>;

    /// Iterates over references to elements front-to-back without consuming the queue.
    ///
    /// # Examples
    ///
    /// ```
    /// use graph_collections::Queue;
    ///
    /// let queue: Queue<u32> = (1..=3).collect();
    /// let sum: u32 = (&queue).into_iter().sum();
    /// assert_eq!(sum, 6);
    /// assert_eq!(queue.len(), 3); // queue is still alive
    /// ```
    fn into_iter(self) -> Self::IntoIter {
        self.data.iter()
    }
}
