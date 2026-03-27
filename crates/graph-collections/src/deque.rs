use std::collections::VecDeque;

/// A double-ended queue (deque) backed by [`VecDeque`].
///
/// Provides O(1) push and pop from **both** ends. Iteration order is
/// front-to-back by default; use `.iter().rev()` or [`Deque::iter_back`]
/// for back-to-front traversal.
///
/// # Differences from [`crate::Queue`]
///
/// [`crate::Queue`] is FIFO — elements enter at the back and leave at the front only.
/// `Deque` relaxes that: you can push and pop from either end independently,
/// making it suitable for sliding-window algorithms, palindrome checks,
/// work-stealing schedulers, and undo/redo stacks.
///
/// # Examples
///
/// ```
/// use graph_collections::Deque;
///
/// let mut deque: Deque<u32> = Deque::new();
///
/// deque.push_back(2);
/// deque.push_front(1);
/// deque.push_back(3);
///
/// // front → back: [1, 2, 3]
/// assert_eq!(deque.front(), Some(&1));
/// assert_eq!(deque.back(),  Some(&3));
///
/// assert_eq!(deque.pop_front(), Some(1));
/// assert_eq!(deque.pop_back(),  Some(3));
/// assert_eq!(deque.len(), 1);
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Deque<T> {
    data: VecDeque<T>,
}

// ── Construction ──────────────────────────────────────────────────────────────

impl<T> Default for Deque<T> {
    fn default() -> Self {
        Self {
            data: VecDeque::new(),
        }
    }
}

impl<T> Deque<T> {
    /// Creates a new, empty deque.
    ///
    /// # Examples
    ///
    /// ```
    /// use graph_collections::Deque;
    ///
    /// let deque: Deque<u32> = Deque::new();
    /// assert!(deque.is_empty());
    /// ```
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates a new, empty deque with at least the given capacity
    /// pre-allocated, avoiding reallocations for the first `capacity` pushes.
    ///
    /// # Examples
    ///
    /// ```
    /// use graph_collections::Deque;
    ///
    /// let deque: Deque<u32> = Deque::with_capacity(32);
    /// assert!(deque.is_empty());
    /// ```
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            data: VecDeque::with_capacity(capacity),
        }
    }
}

// ── State queries ─────────────────────────────────────────────────────────────

impl<T> Deque<T> {
    /// Returns `true` if the deque contains no elements.
    ///
    /// # Examples
    ///
    /// ```
    /// use graph_collections::Deque;
    ///
    /// let mut deque: Deque<u32> = Deque::new();
    /// assert!(deque.is_empty());
    /// deque.push_back(1);
    /// assert!(!deque.is_empty());
    /// ```
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    /// Returns the number of elements in the deque.
    ///
    /// # Examples
    ///
    /// ```
    /// use graph_collections::Deque;
    ///
    /// let mut deque: Deque<u32> = Deque::new();
    /// assert_eq!(deque.len(), 0);
    /// deque.push_back(1);
    /// deque.push_front(0);
    /// assert_eq!(deque.len(), 2);
    /// ```
    #[must_use]
    #[inline]
    pub fn len(&self) -> usize {
        self.data.len()
    }
}

// ── Peek (non-consuming access) ───────────────────────────────────────────────

impl<T> Deque<T> {
    /// Returns a reference to the **front** element, or `None` if empty.
    ///
    /// Does not remove the element.
    ///
    /// # Examples
    ///
    /// ```
    /// use graph_collections::Deque;
    ///
    /// let mut deque: Deque<u32> = Deque::new();
    /// assert_eq!(deque.front(), None);
    /// deque.push_back(1);
    /// deque.push_back(2);
    /// assert_eq!(deque.front(), Some(&1));
    /// assert_eq!(deque.len(), 2); // unchanged
    /// ```
    #[must_use]
    #[inline]
    pub fn front(&self) -> Option<&T> {
        self.data.front()
    }

    /// Returns a mutable reference to the **front** element, or `None` if empty.
    ///
    /// # Examples
    ///
    /// ```
    /// use graph_collections::Deque;
    ///
    /// let mut deque: Deque<u32> = Deque::new();
    /// deque.push_back(1);
    /// if let Some(v) = deque.front_mut() {
    ///     *v = 99;
    /// }
    /// assert_eq!(deque.front(), Some(&99));
    /// ```
    #[must_use]
    #[inline]
    pub fn front_mut(&mut self) -> Option<&mut T> {
        self.data.front_mut()
    }

    /// Returns a reference to the **back** element, or `None` if empty.
    ///
    /// Does not remove the element.
    ///
    /// # Examples
    ///
    /// ```
    /// use graph_collections::Deque;
    ///
    /// let mut deque: Deque<u32> = Deque::new();
    /// assert_eq!(deque.back(), None);
    /// deque.push_back(1);
    /// deque.push_back(2);
    /// assert_eq!(deque.back(), Some(&2));
    /// assert_eq!(deque.len(), 2); // unchanged
    /// ```
    #[must_use]
    #[inline]
    pub fn back(&self) -> Option<&T> {
        self.data.back()
    }

    /// Returns a mutable reference to the **back** element, or `None` if empty.
    ///
    /// # Examples
    ///
    /// ```
    /// use graph_collections::Deque;
    ///
    /// let mut deque: Deque<u32> = Deque::new();
    /// deque.push_back(1);
    /// if let Some(v) = deque.back_mut() {
    ///     *v = 42;
    /// }
    /// assert_eq!(deque.back(), Some(&42));
    /// ```
    #[must_use]
    #[inline]
    pub fn back_mut(&mut self) -> Option<&mut T> {
        self.data.back_mut()
    }

    /// Returns a reference to the element at `index` (front = 0), or `None`
    /// if `index` is out of bounds.
    ///
    /// # Examples
    ///
    /// ```
    /// use graph_collections::Deque;
    ///
    /// let deque: Deque<u32> = (10..=14).collect();
    /// assert_eq!(deque.get(0), Some(&10));
    /// assert_eq!(deque.get(2), Some(&12));
    /// assert_eq!(deque.get(9), None);
    /// ```
    #[must_use]
    #[inline]
    pub fn get(&self, index: usize) -> Option<&T> {
        self.data.get(index)
    }
}

// ── Push (inserting elements) ─────────────────────────────────────────────────

impl<T> Deque<T> {
    /// Appends `element` to the **back** of the deque.
    ///
    /// # Examples
    ///
    /// ```
    /// use graph_collections::Deque;
    ///
    /// let mut deque: Deque<u32> = Deque::new();
    /// deque.push_back(1);
    /// deque.push_back(2);
    /// assert_eq!(deque.back(), Some(&2));
    /// ```
    #[inline]
    pub fn push_back(&mut self, element: T) {
        self.data.push_back(element);
    }

    /// Prepends `element` to the **front** of the deque.
    ///
    /// # Examples
    ///
    /// ```
    /// use graph_collections::Deque;
    ///
    /// let mut deque: Deque<u32> = Deque::new();
    /// deque.push_front(2);
    /// deque.push_front(1);
    /// assert_eq!(deque.front(), Some(&1));
    /// ```
    #[inline]
    pub fn push_front(&mut self, element: T) {
        self.data.push_front(element);
    }
}

// ── Pop (removing elements) ───────────────────────────────────────────────────

impl<T> Deque<T> {
    /// Removes and returns the **front** element, or `None` if empty.
    ///
    /// # Examples
    ///
    /// ```
    /// use graph_collections::Deque;
    ///
    /// let mut deque: Deque<u32> = Deque::new();
    /// assert_eq!(deque.pop_front(), None);
    /// deque.push_back(1);
    /// deque.push_back(2);
    /// assert_eq!(deque.pop_front(), Some(1));
    /// assert_eq!(deque.pop_front(), Some(2));
    /// assert_eq!(deque.pop_front(), None);
    /// ```
    #[inline]
    pub fn pop_front(&mut self) -> Option<T> {
        self.data.pop_front()
    }

    /// Removes and returns the **back** element, or `None` if empty.
    ///
    /// # Examples
    ///
    /// ```
    /// use graph_collections::Deque;
    ///
    /// let mut deque: Deque<u32> = Deque::new();
    /// assert_eq!(deque.pop_back(), None);
    /// deque.push_back(1);
    /// deque.push_back(2);
    /// assert_eq!(deque.pop_back(), Some(2));
    /// assert_eq!(deque.pop_back(), Some(1));
    /// assert_eq!(deque.pop_back(), None);
    /// ```
    #[inline]
    pub fn pop_back(&mut self) -> Option<T> {
        self.data.pop_back()
    }

    /// Removes all elements from the deque.
    ///
    /// # Examples
    ///
    /// ```
    /// use graph_collections::Deque;
    ///
    /// let mut deque: Deque<u32> = (1..=5).collect();
    /// assert_eq!(deque.len(), 5);
    /// deque.clear();
    /// assert!(deque.is_empty());
    /// ```
    #[inline]
    pub fn clear(&mut self) {
        self.data.clear();
    }
}

// ── Rotation ──────────────────────────────────────────────────────────────────

impl<T> Deque<T> {
    /// Rotates the deque `n` steps to the **left**: moves the front `n` elements
    /// to the back.
    ///
    /// Equivalent to calling `push_back(pop_front())` n times, but O(n) not O(n²).
    ///
    /// # Panics
    ///
    /// Panics if `n > self.len()`.
    ///
    /// # Examples
    ///
    /// ```
    /// use graph_collections::Deque;
    ///
    /// let mut deque: Deque<u32> = (1..=5).collect();
    /// deque.rotate_left(2);
    /// // was [1,2,3,4,5], now [3,4,5,1,2]
    /// assert_eq!(deque.front(), Some(&3));
    /// assert_eq!(deque.back(),  Some(&2));
    /// ```
    #[inline]
    pub fn rotate_left(&mut self, n: usize) {
        self.data.rotate_left(n);
    }

    /// Rotates the deque `n` steps to the **right**: moves the back `n` elements
    /// to the front.
    ///
    /// Equivalent to calling `push_front(pop_back())` n times, but O(n) not O(n²).
    ///
    /// # Panics
    ///
    /// Panics if `n > self.len()`.
    ///
    /// # Examples
    ///
    /// ```
    /// use graph_collections::Deque;
    ///
    /// let mut deque: Deque<u32> = (1..=5).collect();
    /// deque.rotate_right(2);
    /// // was [1,2,3,4,5], now [4,5,1,2,3]
    /// assert_eq!(deque.front(), Some(&4));
    /// assert_eq!(deque.back(),  Some(&3));
    /// ```
    #[inline]
    pub fn rotate_right(&mut self, n: usize) {
        self.data.rotate_right(n);
    }
}

// ── Iteration ─────────────────────────────────────────────────────────────────

impl<T> Deque<T> {
    /// Returns a front-to-back iterator over references to the elements.
    ///
    /// The deque is not modified.
    ///
    /// # Examples
    ///
    /// ```
    /// use graph_collections::Deque;
    ///
    /// let deque: Deque<u32> = (1..=4).collect();
    /// let values: Vec<u32> = deque.iter().copied().collect();
    /// assert_eq!(values, [1, 2, 3, 4]);
    /// ```
    #[inline]
    pub fn iter(&self) -> std::collections::vec_deque::Iter<'_, T> {
        self.data.iter()
    }

    /// Returns a **back-to-front** iterator over references to the elements.
    ///
    /// Convenience wrapper around `self.iter().rev()`. Because [`std::collections::vec_deque::Iter`]
    /// implements [`DoubleEndedIterator`], reversing is O(1) setup with O(1)
    /// per element — no allocation.
    ///
    /// # Examples
    ///
    /// ```
    /// use graph_collections::Deque;
    ///
    /// let deque: Deque<u32> = (1..=4).collect();
    /// let values: Vec<u32> = deque.iter_back().copied().collect();
    /// assert_eq!(values, [4, 3, 2, 1]);
    /// ```
    #[inline]
    pub fn iter_back(&self) -> std::iter::Rev<std::collections::vec_deque::Iter<'_, T>> {
        self.data.iter().rev()
    }
}

// ── FromIterator / Extend ─────────────────────────────────────────────────────

impl<T> FromIterator<T> for Deque<T> {
    /// Builds a `Deque` from any iterator. Elements are pushed to the back
    /// in iteration order — front of the iterator becomes front of the deque.
    ///
    /// # Examples
    ///
    /// ```
    /// use graph_collections::Deque;
    ///
    /// let deque: Deque<u32> = (1..=4).collect();
    /// assert_eq!(deque.front(), Some(&1));
    /// assert_eq!(deque.back(),  Some(&4));
    /// assert_eq!(deque.len(),   4);
    /// ```
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        Self {
            data: VecDeque::from_iter(iter),
        }
    }
}

impl<T> Extend<T> for Deque<T> {
    /// Pushes all elements produced by the iterator to the **back** of the deque.
    ///
    /// # Examples
    ///
    /// ```
    /// use graph_collections::Deque;
    ///
    /// let mut deque: Deque<u32> = Deque::new();
    /// deque.push_back(1);
    /// deque.extend([2, 3, 4]);
    /// assert_eq!(deque.len(),  4);
    /// assert_eq!(deque.back(), Some(&4));
    /// ```
    fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
        self.data.extend(iter);
    }
}

// ── IntoIterator ──────────────────────────────────────────────────────────────

/// Consuming iterator: `for x in deque`
impl<T> IntoIterator for Deque<T> {
    type Item = T;
    type IntoIter = std::collections::vec_deque::IntoIter<T>;

    /// Consumes the deque and yields elements front-to-back.
    ///
    /// # Examples
    ///
    /// ```
    /// use graph_collections::Deque;
    ///
    /// let deque: Deque<u32> = (1..=3).collect();
    /// let values: Vec<u32> = deque.into_iter().collect();
    /// assert_eq!(values, [1, 2, 3]);
    /// ```
    fn into_iter(self) -> Self::IntoIter {
        self.data.into_iter()
    }
}

/// Borrowing iterator: `for x in &deque`
impl<'a, T> IntoIterator for &'a Deque<T> {
    type Item = &'a T;
    type IntoIter = std::collections::vec_deque::Iter<'a, T>;

    /// Iterates over references to elements front-to-back without consuming the deque.
    ///
    /// # Examples
    ///
    /// ```
    /// use graph_collections::Deque;
    ///
    /// let deque: Deque<u32> = (1..=3).collect();
    /// let sum: u32 = (&deque).into_iter().sum();
    /// assert_eq!(sum, 6);
    /// assert_eq!(deque.len(), 3); // deque is still alive
    /// ```
    fn into_iter(self) -> Self::IntoIter {
        self.data.iter()
    }
}
