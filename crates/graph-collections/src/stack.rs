/// A LIFO stack backed by a [`Vec`].
///
/// Elements are pushed and popped from the top of the stack.
/// All operations are O(1) amortized.
///
/// # Examples
/// ```
/// use graph_collections::Stack;
/// let mut s: Stack<i32> = Stack::new();
/// s.push(1);
/// s.push(2);
/// assert_eq!(s.pop(), Some(2));
/// assert_eq!(s.len(), 1);
/// assert!(!s.is_empty());
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct Stack<T> {
    data: Vec<T>,
}

impl<T> Default for Stack<T> {
    fn default() -> Self {
        Self { data: Vec::new() }
    }
}

impl<T> From<Vec<T>> for Stack<T> {
    fn from(data: Vec<T>) -> Self {
        Self { data }
    }
}

impl<T> Stack<T> {
    /// Creates a new empty stack.
    ///
    /// # Examples
    /// ```
    /// use graph_collections::Stack;
    /// let s: Stack<i32> = Stack::new();
    /// assert!(s.is_empty());
    /// ```
    pub fn new() -> Self {
        Self::default()
    }

    /// Pushes a value onto the top of the stack.
    ///
    /// # Examples
    /// ```
    /// use graph_collections::Stack;
    /// let mut s: Stack<i32> = Stack::new();
    /// s.push(10);
    /// assert_eq!(s.peek(), Some(&10));
    /// ```
    pub fn push(&mut self, value: T) {
        self.data.push(value);
    }

    /// Removes and returns the top value, or `None` if the stack is empty.
    ///
    /// # Examples
    /// ```
    /// use graph_collections::Stack;
    /// let mut s: Stack<i32> = Stack::new();
    /// s.push(1);
    /// s.push(2);
    /// assert_eq!(s.pop(), Some(2));
    /// assert_eq!(s.pop(), Some(1));
    /// assert_eq!(s.pop(), None);
    /// ```
    #[must_use]
    pub fn pop(&mut self) -> Option<T> {
        self.data.pop()
    }

    /// Returns a reference to the top value without removing it,
    /// or `None` if the stack is empty.
    ///
    /// # Examples
    /// ```
    /// use graph_collections::Stack;
    /// let mut s: Stack<i32> = Stack::new();
    /// s.push(5);
    /// assert_eq!(s.peek(), Some(&5));
    /// assert_eq!(s.len(), 1); // peek does not remove
    /// ```
    #[must_use]
    pub fn peek(&self) -> Option<&T> {
        self.data.last()
    }

    /// Returns `true` if the stack contains no elements.
    ///
    /// # Examples
    /// ```
    /// use graph_collections::Stack;
    /// let mut s: Stack<i32> = Stack::new();
    /// assert!(s.is_empty());
    /// s.push(1);
    /// assert!(!s.is_empty());
    /// ```
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    /// Returns the number of elements in the stack.
    ///
    /// # Examples
    /// ```
    /// use graph_collections::Stack;
    /// let mut s: Stack<i32> = Stack::new();
    /// s.push(1);
    /// s.push(2);
    /// assert_eq!(s.len(), 2);
    /// ```
    #[must_use]
    pub fn len(&self) -> usize {
        self.data.len()
    }
}
