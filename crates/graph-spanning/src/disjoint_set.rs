/// Union-Find (Disjoint Set Union) with **union by rank** and **path
/// compression**.
///
/// This is the complete version of the skeleton from `graph-collections`.
/// Path compression flattens the tree during every [`Self::find`] call, giving
/// amortised O(α(n)) per operation where α is the inverse Ackermann function
/// — effectively constant for all practical inputs.
///
/// # Examples
///
/// ```
/// use graph_spanning::DisjointSet;
///
/// let mut ds = DisjointSet::new(5);
/// assert_eq!(ds.count(), 5);
///
/// ds.union(0, 1);
/// ds.union(2, 3);
/// assert!(ds.connected(0, 1));
/// assert!(ds.connected(2, 3));
/// assert!(!ds.connected(0, 2));
/// assert_eq!(ds.count(), 3);
///
/// ds.union(1, 3); // merges the two groups
/// assert!(ds.connected(0, 3));
/// assert_eq!(ds.count(), 2);
/// ```
#[derive(Debug, Clone)]
pub struct DisjointSet {
    parent: Vec<usize>,
    rank: Vec<usize>,
    count: usize,
}

impl DisjointSet {
    /// Creates a new Union-Find with `n` elements, each in its own set.
    ///
    /// Elements are indexed `0..n`.
    ///
    /// # Examples
    ///
    /// ```
    /// use graph_spanning::DisjointSet;
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

    /// Finds the representative (root) of the set containing `x`.
    ///
    /// Applies **path compression**: every node on the path to the root is
    /// re-pointed directly to the root, flattening the tree for future calls.
    ///
    /// # Panics
    ///
    /// Panics if `x >= n` (out of bounds).
    ///
    /// # Examples
    ///
    /// ```
    /// use graph_spanning::DisjointSet;
    ///
    /// let mut ds = DisjointSet::new(3);
    /// ds.union(0, 1);
    /// // After union, both 0 and 1 share the same representative.
    /// assert_eq!(ds.find(0), ds.find(1));
    /// ```
    pub fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            // Recursive path compression: point x directly to the root.
            self.parent[x] = self.find(self.parent[x]);
        }
        self.parent[x]
    }

    /// Merges the sets containing `x` and `y`.
    ///
    /// Uses **union by rank**: the root of the smaller-rank tree is attached
    /// to the root of the larger-rank tree, keeping the tree shallow.
    ///
    /// Returns `true` if `x` and `y` were in different sets (a merge
    /// occurred), or `false` if they were already in the same set.
    ///
    /// # Panics
    ///
    /// Panics if `x` or `y` is out of bounds.
    ///
    /// # Examples
    ///
    /// ```
    /// use graph_spanning::DisjointSet;
    ///
    /// let mut ds = DisjointSet::new(3);
    /// assert!(ds.union(0, 1));  // merged
    /// assert!(!ds.union(0, 1)); // already connected
    /// ```
    pub fn union(&mut self, x: usize, y: usize) -> bool {
        let rx = self.find(x);
        let ry = self.find(y);

        if rx == ry {
            return false; // already in the same set
        }

        // Attach smaller-rank tree under larger-rank root.
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

    /// Returns `true` if `x` and `y` are in the same set.
    ///
    /// # Panics
    ///
    /// Panics if `x` or `y` is out of bounds.
    ///
    /// # Examples
    ///
    /// ```
    /// use graph_spanning::DisjointSet;
    ///
    /// let mut ds = DisjointSet::new(4);
    /// ds.union(0, 1);
    /// ds.union(1, 2);
    /// assert!(ds.connected(0, 2)); // transitive
    /// assert!(!ds.connected(0, 3));
    /// ```
    pub fn connected(&mut self, x: usize, y: usize) -> bool {
        self.find(x) == self.find(y)
    }

    /// Returns the current number of disjoint sets.
    ///
    /// Starts at `n` and decreases by 1 for every successful [`Self::union`] call.
    ///
    /// # Examples
    ///
    /// ```
    /// use graph_spanning::DisjointSet;
    ///
    /// let mut ds = DisjointSet::new(4);
    /// assert_eq!(ds.count(), 4);
    /// ds.union(0, 1);
    /// assert_eq!(ds.count(), 3);
    /// ```
    #[inline]
    pub fn count(&self) -> usize {
        self.count
    }
}
