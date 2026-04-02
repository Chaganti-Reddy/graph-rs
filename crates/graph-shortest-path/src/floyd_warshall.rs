use graph_core::{Graph, GraphError, NodeId};

type FwResult = (Vec<Vec<f64>>, Vec<Vec<Option<NodeId>>>);

/// Runs the Floyd-Warshall all-pairs shortest-path algorithm.
///
/// Returns a `V × V` distance matrix indexed by node position (the index of
/// the [`NodeId`] as returned by [`NodeId::index`]). Entry `[i][j]` is the
/// shortest distance from node `i` to node `j`, or `f64::INFINITY` if no
/// path exists.
///
/// # Errors
///
/// Returns [`GraphError::NegativeCycle`] if a negative-weight cycle is
/// detected (i.e., `dist[i][i] < 0` for any `i` after the algorithm).
///
/// # Complexity
///
/// O(V³) time, O(V²) space.
///
/// # Examples
///
/// ```
/// use graph_core::{AdjacencyList, Graph};
/// use graph_shortest_path::floyd_warshall;
///
/// let mut g: AdjacencyList<()> = AdjacencyList::directed();
/// let a = g.add_node(());  // index 0
/// let b = g.add_node(());  // index 1
/// let c = g.add_node(());  // index 2
/// g.add_edge(a, b, 3.0).unwrap();
/// g.add_edge(b, c, 2.0).unwrap();
/// g.add_edge(a, c, 10.0).unwrap();
///
/// let dist = floyd_warshall(&g).unwrap();
/// assert_eq!(dist[0][2], 5.0); // a→b→c costs 5, cheaper than a→c at 10
/// assert_eq!(dist[1][0], f64::INFINITY); // no path back
/// ```
///
/// [`NodeId::index`]: graph_core::NodeId::index
#[allow(clippy::needless_range_loop)] // 2-D matrix: dist[i][k], dist[k][j] require numeric indices
pub fn floyd_warshall<G>(graph: &G) -> Result<Vec<Vec<f64>>, GraphError>
where
    G: Graph<Weight = f64>,
{
    let n = graph.node_count();

    // Initialise the distance matrix.
    let mut dist = vec![vec![f64::INFINITY; n]; n];

    // Diagonal: distance from a node to itself is 0.
    for i in 0..n {
        dist[i][i] = 0.0;
    }

    // Seed from the graph's edges.
    for u in graph.nodes() {
        for (v, &w) in graph.neighbors(u) {
            let i = u.index();
            let j = v.index();
            // Keep minimum if there are parallel edges.
            if w < dist[i][j] {
                dist[i][j] = w;
            }
        }
    }

    // Triple loop: k is the intermediate node being considered.
    for k in 0..n {
        for i in 0..n {
            // Skip if there is no path from i to k.
            if dist[i][k] == f64::INFINITY {
                continue;
            }
            for j in 0..n {
                if dist[k][j] == f64::INFINITY {
                    continue;
                }
                let through_k = dist[i][k] + dist[k][j];
                if through_k < dist[i][j] {
                    dist[i][j] = through_k;
                }
            }
        }
    }

    // Negative-cycle detection: any negative diagonal entry means a cycle.
    for i in 0..n {
        if dist[i][i] < 0.0 {
            return Err(GraphError::NegativeCycle);
        }
    }

    Ok(dist)
}

/// Returns the shortest path (as a sequence of [`NodeId`]s) between two nodes
/// using a pre-computed Floyd-Warshall distance matrix and next-hop table.
///
/// Call [`floyd_warshall_with_paths`] instead of [`floyd_warshall`] if you
/// need path reconstruction.
///
/// # Examples
///
/// ```
/// use graph_core::{AdjacencyList, Graph};
/// use graph_shortest_path::floyd_warshall::{floyd_warshall_with_paths, reconstruct_fw_path};
///
/// let mut g: AdjacencyList<()> = AdjacencyList::directed();
/// let a = g.add_node(());
/// let b = g.add_node(());
/// let c = g.add_node(());
/// g.add_edge(a, b, 1.0).unwrap();
/// g.add_edge(b, c, 1.0).unwrap();
///
/// let (dist, next) = floyd_warshall_with_paths(&g).unwrap();
/// assert_eq!(dist[0][2], 2.0);
///
/// // Reconstruct path a → c
/// let path = reconstruct_fw_path(&next, a, c);
/// assert_eq!(path, Some(vec![a, b, c]));
/// ```
#[allow(clippy::needless_range_loop)] // 2-D matrix: dist[i][k], next[i][j] require numeric indices
pub fn floyd_warshall_with_paths<G>(graph: &G) -> Result<FwResult, GraphError>
where
    G: Graph<Weight = f64>,
{
    let n = graph.node_count();

    let mut dist = vec![vec![f64::INFINITY; n]; n];
    // next[i][j] = first hop from i toward j (None if no path).
    let mut next: Vec<Vec<Option<NodeId>>> = vec![vec![None; n]; n];

    for i in 0..n {
        dist[i][i] = 0.0;
    }

    // Seed from edges.
    for u in graph.nodes() {
        for (v, &w) in graph.neighbors(u) {
            let i = u.index();
            let j = v.index();
            if w < dist[i][j] {
                dist[i][j] = w;
                next[i][j] = Some(v);
            }
        }
    }
    // Self-hops.
    for u in graph.nodes() {
        let i = u.index();
        next[i][i] = Some(u);
    }

    for k in 0..n {
        for i in 0..n {
            if dist[i][k] == f64::INFINITY {
                continue;
            }
            for j in 0..n {
                if dist[k][j] == f64::INFINITY {
                    continue;
                }
                let through_k = dist[i][k] + dist[k][j];
                if through_k < dist[i][j] {
                    dist[i][j] = through_k;
                    next[i][j] = next[i][k]; // route i→j via k
                }
            }
        }
    }

    for i in 0..n {
        if dist[i][i] < 0.0 {
            return Err(GraphError::NegativeCycle);
        }
    }

    Ok((dist, next))
}

/// Reconstructs a shortest path using the `next` table from
/// [`floyd_warshall_with_paths`].
///
/// Returns `None` if no path exists from `start` to `end`.
pub fn reconstruct_fw_path(
    next: &[Vec<Option<NodeId>>],
    start: NodeId,
    end: NodeId,
) -> Option<Vec<NodeId>> {
    next[start.index()][end.index()]?;

    let mut path = vec![start];
    let mut current = start;

    while current != end {
        current = next[current.index()][end.index()]?;
        path.push(current);
        // Guard against infinite loops from negative cycles.
        if path.len() > next.len() + 1 {
            return None;
        }
    }

    Some(path)
}
