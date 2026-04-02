//! Hopcroft-Karp maximum bipartite matching.
//!
//! Finds the maximum matching in a bipartite graph using alternating BFS and
//! DFS phases, achieving O(E · √V) — significantly faster than the naive
//! O(V · E) augmenting-path approach for large graphs.

use std::collections::VecDeque;

/// The result of a Hopcroft-Karp bipartite matching computation.
#[derive(Debug, Clone)]
pub struct BipartiteMatching {
    /// For each node in the **left** partition, the matched node in the right
    /// partition (`Some(right_node)`) or `None` if unmatched.
    ///
    /// Indexed `0..left_size`.
    pub match_left: Vec<Option<usize>>,
    /// For each node in the **right** partition, the matched node in the left
    /// partition (`Some(left_node)`) or `None` if unmatched.
    ///
    /// Indexed `0..right_size`.
    pub match_right: Vec<Option<usize>>,
    /// The size of the maximum matching (number of matched pairs).
    pub matching_size: usize,
}

impl BipartiteMatching {
    /// Returns all matched pairs as `(left_node, right_node)` tuples.
    ///
    /// # Examples
    ///
    /// ```
    /// use graph_flow::hopcroft_karp;
    ///
    /// // Left: {0, 1}, Right: {0, 1}
    /// // Edges: 0→0, 0→1, 1→1
    /// let adj = vec![vec![0usize, 1], vec![1]];
    /// let matching = hopcroft_karp(&adj, 2);
    /// let pairs = matching.pairs();
    /// assert_eq!(pairs.len(), 2);
    /// ```
    pub fn pairs(&self) -> Vec<(usize, usize)> {
        self.match_left
            .iter()
            .enumerate()
            .filter_map(|(l, &r)| r.map(|r| (l, r)))
            .collect()
    }
}

const UNMATCHED: usize = usize::MAX;
const INF: usize = usize::MAX;

/// Computes the **maximum bipartite matching** using the Hopcroft-Karp
/// algorithm.
///
/// The bipartite graph is described by an adjacency list for the **left**
/// partition only: `adj[u]` contains the indices of right-partition nodes that
/// left-node `u` is connected to.
///
/// # Arguments
///
/// - `adj` — adjacency list for left nodes; `adj[u]` = list of right nodes
///   adjacent to left node `u`. Left nodes are indexed `0..adj.len()`.
/// - `right_size` — the number of nodes in the right partition, indexed
///   `0..right_size`.
///
/// # Returns
///
/// A [`BipartiteMatching`] with the match assignments and total matching size.
///
/// # Algorithm
///
/// Hopcroft-Karp alternates between two phases:
///
/// 1. **BFS phase** — finds the shortest augmenting paths layer by layer,
///    producing a layered graph.
/// 2. **DFS phase** — finds a maximal set of vertex-disjoint augmenting paths
///    within the layered graph and augments all of them simultaneously.
///
/// Each round of BFS + DFS increases the length of the shortest remaining
/// augmenting path. Since augmenting path lengths are odd integers bounded by
/// V, there are at most √V rounds, each taking O(E) — giving O(E · √V) total.
///
/// # Complexity
///
/// O(E · √V).
///
/// # Examples
///
/// ```
/// use graph_flow::hopcroft_karp;
///
/// // Bipartite graph:
/// //   Left 0 — Right 0
/// //   Left 0 — Right 1
/// //   Left 1 — Right 1
/// //   Left 2 — Right 2
/// //
/// // Maximum matching: (0,0), (1,1), (2,2) — size 3.
/// let adj = vec![
///     vec![0usize, 1], // left 0 connects to right 0 and 1
///     vec![1],         // left 1 connects to right 1
///     vec![2],         // left 2 connects to right 2
/// ];
/// let matching = hopcroft_karp(&adj, 3);
/// assert_eq!(matching.matching_size, 3);
/// ```
pub fn hopcroft_karp(adj: &[Vec<usize>], right_size: usize) -> BipartiteMatching {
    let left_size = adj.len();

    // match_left[u] = right node matched to left u (UNMATCHED if none).
    let mut match_left = vec![UNMATCHED; left_size];
    // match_right[v] = left node matched to right v (UNMATCHED if none).
    let mut match_right = vec![UNMATCHED; right_size];

    let mut matching_size = 0;

    loop {
        // BFS phase: build layered graph and compute dist[] for left nodes.
        let dist = bfs_phase(adj, &match_left, &match_right, left_size);

        if dist[left_size] == INF {
            // No augmenting path exists — maximum matching found.
            break;
        }

        // DFS phase: find maximal set of augmenting paths in the layered graph.
        // We need dist to be mutable for the DFS to mark used layers.
        let mut dist = dist;

        for u in 0..left_size {
            if match_left[u] == UNMATCHED
                && dfs_phase(
                    adj,
                    &mut match_left,
                    &mut match_right,
                    &mut dist,
                    u,
                    left_size,
                )
            {
                matching_size += 1;
            }
        }
    }

    // Convert UNMATCHED sentinels to Option.
    let match_left_opt: Vec<Option<usize>> = match_left
        .iter()
        .map(|&m| if m == UNMATCHED { None } else { Some(m) })
        .collect();
    let match_right_opt: Vec<Option<usize>> = match_right
        .iter()
        .map(|&m| if m == UNMATCHED { None } else { Some(m) })
        .collect();

    BipartiteMatching {
        match_left: match_left_opt,
        match_right: match_right_opt,
        matching_size,
    }
}

/// BFS phase: computes shortest distances from free left nodes to the virtual
/// sink node (`dist[left_size]`).
///
/// Left nodes are `0..left_size`. We use `left_size` as a virtual sink
/// sentinel representing "reached an unmatched right node".
///
/// Returns `dist` where `dist[u]` is the BFS layer of left node `u`.
fn bfs_phase(
    adj: &[Vec<usize>],
    match_left: &[usize],
    match_right: &[usize],
    left_size: usize,
) -> Vec<usize> {
    let mut dist = vec![INF; left_size + 1]; // +1 for the virtual sink
    let mut queue = VecDeque::new();

    // Seed BFS with all free (unmatched) left nodes at layer 0.
    for u in 0..left_size {
        if match_left[u] == UNMATCHED {
            dist[u] = 0;
            queue.push_back(u);
        }
    }

    // dist[left_size] represents reaching a free right node (augmenting path found).
    dist[left_size] = INF;

    while let Some(u) = queue.pop_front() {
        if dist[u] < dist[left_size] {
            for &v in &adj[u] {
                // The next left node along the alternating path is the one
                // currently matched to right node v.
                let next_left = match_right[v];
                if next_left == UNMATCHED {
                    // v is a free right node: we've found an augmenting path.
                    dist[left_size] = dist[u] + 1;
                } else if dist[next_left] == INF {
                    dist[next_left] = dist[u] + 1;
                    queue.push_back(next_left);
                }
            }
        }
    }

    dist
}

/// DFS phase: finds and augments one alternating path from left node `u` to a
/// free right node, using only edges consistent with the layered graph.
///
/// Returns `true` if an augmenting path was found and flow was pushed.
fn dfs_phase(
    adj: &[Vec<usize>],
    match_left: &mut Vec<usize>,
    match_right: &mut Vec<usize>,
    dist: &mut Vec<usize>,
    u: usize,
    left_size: usize,
) -> bool {
    if u == left_size {
        return true; // Reached the virtual sink — augmenting path complete.
    }

    for &v in &adj[u] {
        let next_left = match_right[v];
        let next_left_idx = if next_left == UNMATCHED {
            left_size
        } else {
            next_left
        };

        // Only follow edges that advance us in the layered graph.
        if dist[next_left_idx] == dist[u] + 1
            && dfs_phase(adj, match_left, match_right, dist, next_left_idx, left_size)
        {
            match_left[u] = v;
            match_right[v] = u;
            return true;
        }
    }

    // No augmenting path from u — block this node from future DFS visits
    // this round by setting its distance to INF.
    dist[u] = INF;
    false
}
