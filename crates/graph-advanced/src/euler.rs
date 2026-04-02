use graph_core::{Graph, NodeId};
use std::collections::HashMap;

/// Errors returned by Euler path/circuit algorithms.
#[derive(Debug, Clone, PartialEq)]
pub enum EulerError {
    /// The graph has no nodes.
    EmptyGraph,
    /// The graph is disconnected (ignoring isolated nodes), so no Euler
    /// path or circuit can traverse all edges.
    Disconnected,
    /// The degree conditions for an Euler circuit are not met.
    ///
    /// For undirected graphs: all nodes must have even degree.
    /// For directed graphs: every node must have equal in-degree and out-degree.
    NoCircuit,
    /// The degree conditions for an Euler path are not met.
    ///
    /// For undirected graphs: exactly two nodes must have odd degree.
    /// For directed graphs: exactly one node must have `out - in = 1` (start)
    /// and one must have `in - out = 1` (end).
    NoPath,
}

impl std::fmt::Display for EulerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EulerError::EmptyGraph => write!(f, "graph is empty"),
            EulerError::Disconnected => write!(f, "graph is disconnected"),
            EulerError::NoCircuit => write!(f, "Euler circuit does not exist (odd-degree nodes)"),
            EulerError::NoPath => write!(f, "Euler path does not exist (wrong degree sequence)"),
        }
    }
}

impl std::error::Error for EulerError {}

/// Finds an **Euler circuit** in an undirected graph using Hierholzer's algorithm.
///
/// An Euler circuit visits every edge **exactly once** and returns to the
/// starting node. It exists iff the graph is connected (ignoring isolated
/// nodes) and every node has even degree.
///
/// # Returns
///
/// `Ok(Vec<NodeId>)` — the sequence of nodes visited, with the first node
/// repeated at the end (first == last).
///
/// # Errors
///
/// - [`EulerError::EmptyGraph`] — no nodes in the graph.
/// - [`EulerError::Disconnected`] — graph is not connected.
/// - [`EulerError::NoCircuit`] — some node has odd degree.
///
/// # Complexity
///
/// O(E) — Hierholzer's algorithm visits each edge exactly twice
/// (once per direction in the undirected adjacency list).
///
/// # Examples
///
/// ```
/// use graph_core::{AdjacencyList, Graph};
/// use graph_advanced::euler_circuit;
///
/// // Complete graph K3: triangle — Euler circuit exists.
/// let mut g: AdjacencyList<()> = AdjacencyList::undirected();
/// let n: Vec<_> = (0..3).map(|_| g.add_node(())).collect();
/// g.add_edge(n[0], n[1], 1.0).unwrap();
/// g.add_edge(n[1], n[2], 1.0).unwrap();
/// g.add_edge(n[2], n[0], 1.0).unwrap();
///
/// let circuit = euler_circuit(&g).unwrap();
/// assert_eq!(circuit.len(), 4); // 3 edges + return to start
/// assert_eq!(circuit.first(), circuit.last());
/// ```
pub fn euler_circuit<G>(graph: &G) -> Result<Vec<NodeId>, EulerError>
where
    G: Graph<Weight = f64>,
{
    let start = first_non_isolated_node(graph).ok_or(EulerError::EmptyGraph)?;

    // Check: all nodes with edges must have even degree.
    for node in graph.nodes() {
        if graph.degree(node) % 2 != 0 {
            return Err(EulerError::NoCircuit);
        }
    }

    let circuit = hierholzer(graph, start)?;
    Ok(circuit)
}

/// Finds an **Euler path** in an undirected graph using Hierholzer's algorithm.
///
/// An Euler path visits every edge **exactly once** but need not return to the
/// start. It exists iff the graph is connected (ignoring isolated nodes) and
/// **exactly two** nodes have odd degree (these are the path endpoints).
///
/// # Returns
///
/// `Ok(Vec<NodeId>)` — the sequence of nodes visited, from the odd-degree
/// start node to the other odd-degree end node.
///
/// # Errors
///
/// - [`EulerError::EmptyGraph`] — no nodes in the graph.
/// - [`EulerError::Disconnected`] — graph is not connected.
/// - [`EulerError::NoPath`] — the graph does not have exactly two odd-degree nodes.
///
/// # Complexity
///
/// O(E).
///
/// # Examples
///
/// ```
/// use graph_core::{AdjacencyList, Graph};
/// use graph_advanced::euler_path;
///
/// // Path graph 0-1-2-3: nodes 0 and 3 have odd degree (1 each).
/// let mut g: AdjacencyList<()> = AdjacencyList::undirected();
/// let n: Vec<_> = (0..4).map(|_| g.add_node(())).collect();
/// g.add_edge(n[0], n[1], 1.0).unwrap();
/// g.add_edge(n[1], n[2], 1.0).unwrap();
/// g.add_edge(n[2], n[3], 1.0).unwrap();
///
/// let path = euler_path(&g).unwrap();
/// assert_eq!(path.len(), 4); // 3 edges → 4 nodes
/// ```
pub fn euler_path<G>(graph: &G) -> Result<Vec<NodeId>, EulerError>
where
    G: Graph<Weight = f64>,
{
    if graph.node_count() == 0 {
        return Err(EulerError::EmptyGraph);
    }

    let odd_nodes: Vec<NodeId> = graph
        .nodes()
        .filter(|&n| graph.degree(n) % 2 != 0)
        .collect();

    if odd_nodes.len() != 2 {
        return Err(EulerError::NoPath);
    }

    // Start from one of the two odd-degree nodes.
    let start = odd_nodes[0];
    let circuit = hierholzer(graph, start)?;
    Ok(circuit)
}

// ── Internal helpers ──────────────────────────────────────────────────────────

/// Hierholzer's algorithm: finds an Euler circuit/path starting at `start`.
///
/// Builds a local adjacency list from the graph and tracks which neighbour
/// slots have been consumed via a per-node pointer.  For undirected graphs
/// the `Graph` trait stores each edge in **both** directions; to avoid
/// traversing the same logical edge twice we assign each adjacency-list entry
/// a unique global edge id and mark it used when traversed.
///
/// Uses an explicit stack instead of recursion to avoid stack overflow.
fn hierholzer<G>(graph: &G, start: NodeId) -> Result<Vec<NodeId>, EulerError>
where
    G: Graph<Weight = f64>,
{
    let nodes: Vec<NodeId> = graph.nodes().collect();
    let n = nodes.len();
    let index_of: HashMap<NodeId, usize> =
        nodes.iter().enumerate().map(|(i, &id)| (id, i)).collect();

    // Build adjacency as (neighbour_node_index, edge_id) pairs.
    // For an undirected graph each logical edge (u,v) appears as two
    // adjacency-list entries. We assign consecutive ids to neighbours of each
    // node in order, then identify the paired entry by matching neighbours
    // across both directions. Instead we use a simpler scheme: give each
    // directed adjacency-list slot a unique id, and store both directions with
    // matching ids so we can mark the reverse as used too.
    //
    // Scheme: for each node u enumerate its neighbours in order. The forward
    // entry (u, i) gets edge_id = base_offset[u] + i.  We then build a
    // reverse lookup: reverse[v][edge_id_of_u_to_v] = edge_id_of_v_to_u.

    // adj[i] = list of (neighbour_index, edge_id)
    let mut adj: Vec<Vec<(usize, usize)>> = vec![Vec::new(); n];
    let mut global_id = 0usize;
    let mut edge_id_base: Vec<usize> = vec![0; n]; // starting edge_id for each node's adj list

    for (ui, &u) in nodes.iter().enumerate() {
        edge_id_base[ui] = global_id;
        for (v, _) in graph.neighbors(u) {
            let vi = index_of[&v];
            adj[ui].push((vi, global_id));
            global_id += 1;
        }
    }

    // Total directed adjacency entries = global_id.
    let mut used = vec![false; global_id];

    // For each directed entry (u→v with edge_id e_uv), find the corresponding
    // reverse entry (v→u with edge_id e_vu) so we can mark both used at once.
    // We do this by building a map: (u_idx, v_idx) → list of edge_ids from u to v.
    // Then for each (u, v, e_uv) the reverse is any unused edge_id in (v, u).
    // To keep it O(E), pre-build reverse_id[edge_id] = paired reverse edge_id.
    //
    // For a directed graph there is no paired reverse entry — we only mark the
    // forward direction.  We detect undirected mode by checking whether
    // every adjacency entry has a matching reverse.

    // reverse_of[e] = edge_id of the reverse half-edge, or usize::MAX if none.
    let mut reverse_of = vec![usize::MAX; global_id];
    {
        // For each node v, build a map neighbour_idx → queue of edge_ids coming IN to v (i.e. the v→u edges).
        // We use these to pair up undirected half-edges.
        let mut in_edges: Vec<HashMap<usize, std::collections::VecDeque<usize>>> =
            vec![HashMap::new(); n];
        for (ui, adj_ui) in adj.iter().enumerate() {
            for &(vi, eid) in adj_ui {
                in_edges[vi].entry(ui).or_default().push_back(eid);
            }
        }
        // For each directed entry u→v (edge_id e_uv), look in in_edges[u] for a v→u edge.
        for (ui, adj_ui) in adj.iter().enumerate() {
            for &(vi, e_uv) in adj_ui {
                if let Some(queue) = in_edges[ui].get_mut(&vi) {
                    if let Some(e_vu) = queue.pop_front() {
                        reverse_of[e_uv] = e_vu;
                    }
                }
            }
        }
    }

    // Count logical edges to use for the final length check.
    // For undirected graphs (where reverse_of[e] != MAX), each logical edge
    // is counted twice in global_id, so we divide by 2 for the check.
    // We infer undirected mode if at least one reverse pair exists.
    let has_reverse = reverse_of.iter().any(|&r| r != usize::MAX);
    let logical_edge_count = if has_reverse {
        global_id / 2
    } else {
        global_id
    };

    // Hierholzer's with per-node pointer (advance past used edges).
    let mut ptr: Vec<usize> = vec![0; n];
    let start_idx = index_of[&start];
    let mut stack: Vec<usize> = vec![start_idx];
    let mut circuit: Vec<usize> = Vec::new();

    while let Some(&curr) = stack.last() {
        // Advance ptr past any already-used edges.
        while ptr[curr] < adj[curr].len() && used[adj[curr][ptr[curr]].1] {
            ptr[curr] += 1;
        }

        if ptr[curr] < adj[curr].len() {
            let (next, eid) = adj[curr][ptr[curr]];
            ptr[curr] += 1;
            // Mark this edge (and its reverse) as used.
            used[eid] = true;
            if reverse_of[eid] != usize::MAX {
                used[reverse_of[eid]] = true;
            }
            stack.push(next);
        } else {
            circuit.push(stack.pop().unwrap());
        }
    }

    circuit.reverse();

    // Check that all edges were consumed (connectivity check).
    if circuit.len() != logical_edge_count + 1 {
        return Err(EulerError::Disconnected);
    }

    // Convert indices back to NodeIds.
    Ok(circuit.into_iter().map(|i| nodes[i]).collect())
}

/// Returns the first node that has at least one edge, or `None` if the graph
/// is empty or all nodes are isolated.
fn first_non_isolated_node<G>(graph: &G) -> Option<NodeId>
where
    G: Graph,
{
    graph.nodes().find(|&n| graph.degree(n) > 0)
}
