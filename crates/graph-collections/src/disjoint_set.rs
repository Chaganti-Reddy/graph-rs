/// Union-Find (disjoint set union) with **union-by-rank**.
///
/// Supports O(log n) `find` and `union` in this phase. Elements are represented as integer indices `0..n`. Create a `DisjointSet` of size `n`, then call `union` to merge components and `find` to discover which component an element belongs to.
///
/// # Examples
///
/// ```
/// use graph_collections::DisjointSet;
///
/// let mut ds = DisjointSet::new(5); // indices 0..5
/// assert_eq!(ds.count(), 5);
///
/// ds.union(0, 1);
/// ds.union(1, 2);
/// assert!(ds.connected(0, 2));
/// assert!(!ds.connected(0, 3));
/// assert_eq!(ds.count(), 3); // {0,1,2}, {3}, {4}
/// ```
#[derive(Debug, Clone)]
pub struct DisjointSet {
    parent: Vec<usize>,
    rank: Vec<usize>,
    count: usize, // number of disjoint sets
}

impl DisjointSet {
    /// Creates a `DisjointSet` with `n` singleton sets, one per index `0..n`.
    ///
    /// Each element is initially its own parent (root), and every rank starts
    /// at 0.
    ///
    /// # Panics
    ///
    /// Does not panic, but `n = 0` gives an empty structure where every
    /// operation on any index would panic at the index.
    ///
    /// # Examples
    ///
    /// ```
    /// use graph_collections::DisjointSet;
    ///
    /// let ds = DisjointSet::new(4);
    /// assert_eq!(ds.count(), 4);
    /// ```
    pub fn new(n: usize) -> Self {
        Self {
            parent: (0..n).collect(),
            rank: vec![0; n],
            count: n,
        }
    }
}

impl DisjointSet {
    /// Returns the **representative (root)** of the set containing `x`.
    ///
    /// Uses iterative root-following without path compression. Two elements are in the same component iff `find(a) == find(b)`.
    ///
    /// # Panics
    ///
    /// Panics if `x >= n` (out of bounds).
    ///
    /// # Examples
    ///
    /// ```
    /// use graph_collections::DisjointSet;
    ///
    /// let mut ds = DisjointSet::new(3);
    /// ds.union(0, 1);
    /// // Both 0 and 1 have the same root; 2 has its own.
    /// assert_eq!(ds.find(0), ds.find(1));
    /// assert_ne!(ds.find(0), ds.find(2));
    /// ```
    pub fn find(&mut self, mut x: usize) -> usize {
        // Walk to the root without modifying the tree.
        while self.parent[x] != x {
            x = self.parent[x];
        }
        x
    }

    /// Merges the sets containing `x` and `y`.
    ///
    /// Uses **union-by-rank**: the root with the lower rank is attached under
    /// the root with the higher rank, keeping trees shallow. When ranks are
    /// equal the second root is attached under the first and its rank increments.
    ///
    /// Returns `true` if the two elements were in **different** sets (a merge
    /// actually happened), or `false` if they were already in the same set.
    ///
    /// # Panics
    ///
    /// Panics if `x >= n` or `y >= n`.
    ///
    /// # Examples
    ///
    /// ```
    /// use graph_collections::DisjointSet;
    ///
    /// let mut ds = DisjointSet::new(4);
    /// assert!(ds.union(0, 1));  // new merge
    /// assert!(!ds.union(0, 1)); // already connected
    /// assert_eq!(ds.count(), 3);
    /// ```
    pub fn union(&mut self, x: usize, y: usize) -> bool {
        let rx = self.find(x);
        let ry = self.find(y);

        if rx == ry {
            return false; // already in the same component
        }

        // Attach smaller-rank tree under larger-rank tree.
        match self.rank[rx].cmp(&self.rank[ry]) {
            std::cmp::Ordering::Less => self.parent[rx] = ry,
            std::cmp::Ordering::Greater => self.parent[ry] = rx,
            std::cmp::Ordering::Equal => {
                self.parent[ry] = rx;
                self.rank[rx] += 1;
            }
        }

        self.count -= 1;
        true
    }

    /// Returns `true` if `x` and `y` belong to the same component.
    ///
    /// Equivalent to `find(x) == find(y)`.
    ///
    /// # Panics
    ///
    /// Panics if `x >= n` or `y >= n`.
    ///
    /// # Examples
    ///
    /// ```
    /// use graph_collections::DisjointSet;
    ///
    /// let mut ds = DisjointSet::new(5);
    /// ds.union(1, 3);
    /// assert!(ds.connected(1, 3));
    /// assert!(!ds.connected(1, 4));
    /// ```
    #[inline]
    pub fn connected(&mut self, x: usize, y: usize) -> bool {
        self.find(x) == self.find(y)
    }

    /// Returns the number of **disjoint sets** (connected components).
    ///
    /// Starts at `n` and decrements by one for each successful `union`.
    ///
    /// # Examples
    ///
    /// ```
    /// use graph_collections::DisjointSet;
    ///
    /// let mut ds = DisjointSet::new(4);
    /// assert_eq!(ds.count(), 4);
    /// ds.union(0, 1);
    /// assert_eq!(ds.count(), 3);
    /// ds.union(2, 3);
    /// assert_eq!(ds.count(), 2);
    /// ds.union(0, 3);
    /// assert_eq!(ds.count(), 1);
    /// ```
    #[must_use]
    #[inline]
    pub fn count(&self) -> usize {
        self.count
    }
}

impl DisjointSet {
    /// Returns the total number of elements (size passed to [`DisjointSet::new`]).
    ///
    /// This is the **capacity** of the structure, not the number of components.
    /// For the number of components use [`count`](DisjointSet::count).
    ///
    /// # Examples
    ///
    /// ```
    /// use graph_collections::DisjointSet;
    ///
    /// let ds = DisjointSet::new(10);
    /// assert_eq!(ds.size(), 10);
    /// ```
    #[must_use]
    #[inline]
    pub fn size(&self) -> usize {
        self.parent.len()
    }

    /// Returns `true` if all elements belong to a single component.
    ///
    /// # Examples
    ///
    /// ```
    /// use graph_collections::DisjointSet;
    ///
    /// let mut ds = DisjointSet::new(3);
    /// assert!(!ds.is_fully_connected());
    /// ds.union(0, 1);
    /// ds.union(1, 2);
    /// assert!(ds.is_fully_connected());
    /// ```
    #[inline]
    pub fn is_fully_connected(&self) -> bool {
        self.count == 1
    }
}
