# graph-rs

A complete graph algorithm library built from scratch in Rust — every data
structure, every algorithm, written by hand with no third-party graph
dependencies. Built as a structured curriculum project covering 40+ algorithms
across 8 library crates.

[![CI](https://github.com/Chaganti-Reddy/graph-rs/actions/workflows/ci.yml/badge.svg)](https://github.com/Chaganti-Reddy/graph-rs/actions)
[![Docs](https://img.shields.io/badge/docs-gh--pages-blue)](https://chaganti-reddy.github.io/graph-rs)
[![License: MIT OR Apache-2.0](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue.svg)](#license)

---

## Table of contents

- [Quick start](#quick-start)
- [Workspace layout](#workspace-layout)
- [Core abstractions](#core-abstractions)
  - [NodeId and EdgeId](#nodeid-and-edgeid)
  - [The Graph trait](#the-graph-trait)
  - [AdjacencyList vs AdjacencyMatrix](#adjacencylist-vs-adjacencymatrix)
  - [GraphBuilder](#graphbuilder)
  - [GraphError](#grapherror)
- [Algorithms](#algorithms)
  - [Traversal](#traversal)
  - [Shortest paths](#shortest-paths)
  - [Spanning trees](#spanning-trees)
  - [Maximum flow](#maximum-flow)
  - [Advanced](#advanced)
- [Collections](#collections)
- [Property-based tests](#property-based-tests)
- [Benchmarks](#benchmarks)
- [Development commands](#development-commands)
- [Algorithm complexity reference](#algorithm-complexity-reference)
- [License](#license)

---

## Quick start

Clone the repo and add any crate as a path dependency:

```toml
# Cargo.toml
[dependencies]
# meta-crate: re-exports everything via prelude::*
graph = { path = "../graph-rs/crates/graph" }

# or pull individual crates
graph-core          = { path = "../graph-rs/crates/graph-core" }
graph-traversal     = { path = "../graph-rs/crates/graph-traversal" }
graph-shortest-path = { path = "../graph-rs/crates/graph-shortest-path" }
```

```rust
use graph::prelude::*;

// Build a weighted directed graph
let mut g: AdjacencyList<&str> = AdjacencyList::directed();
let a = g.add_node("A");
let b = g.add_node("B");
let c = g.add_node("C");
g.add_edge(a, b, 1.0).unwrap();
g.add_edge(b, c, 2.0).unwrap();
g.add_edge(a, c, 10.0).unwrap();

// A→B→C (cost 3) beats A→C direct (cost 10)
let result = dijkstra(&g, a).unwrap();
assert_eq!(result.distances[&c], 3.0);
assert_eq!(result.parents[&c], b); // reconstruct the path

// Traverse
let dist = bfs(&g, a);
assert_eq!(dist[&b], 1);

// Detect structure
assert!(!has_cycle_directed(&g));
```

---

## Workspace layout

```
graph-rs/
├── crates/
│   ├── graph-collections/   # Stack, Queue, Deque, MinHeap, PriorityQueue, DisjointSet
│   ├── graph-core/          # Graph trait, NodeId, AdjacencyList, AdjacencyMatrix, GraphBuilder
│   ├── graph-traversal/     # DFS, BFS, topological sort, cycle detection, components, bipartite
│   ├── graph-shortest-path/ # Dijkstra, Bellman-Ford, Floyd-Warshall, A*
│   ├── graph-spanning/      # Kruskal, Prim, bridges, articulation points
│   ├── graph-flow/          # Ford-Fulkerson, Edmonds-Karp, min-cut, Hopcroft-Karp
│   ├── graph-advanced/      # Tarjan SCC, Kosaraju SCC, condensation, Euler, Hamiltonian, TSP
│   └── graph/               # Meta-crate: re-exports everything via prelude::*
├── .github/workflows/ci.yml # fmt → clippy → test → doc → deploy to gh-pages
├── CHANGELOG.md
└── RELEASE_CHECKLIST.md
```

Dependency order (each crate only depends on those above it):

```
graph-collections
       ↓
  graph-core
       ↓
graph-traversal   graph-shortest-path   graph-spanning   graph-flow   graph-advanced
       ↓                  ↓                   ↓               ↓             ↓
                        graph  (meta-crate: re-exports all)
```

---

## Core abstractions

### NodeId and EdgeId

All nodes are identified by `NodeId`, a zero-cost `usize` newtype. Using a
newtype prevents accidentally passing a raw index where a `NodeId` is expected
and vice versa.

```rust
use graph_core::NodeId;

let id = NodeId::new(0);
assert_eq!(id.index(), 0);

// From<usize> is implemented for ergonomics
let id: NodeId = 5usize.into();
```

`EdgeId` is the corresponding newtype for edge identifiers, with the same API.

### The Graph trait

Every algorithm in the library is generic over the `Graph` trait. This means
all algorithms work with both `AdjacencyList` and `AdjacencyMatrix` without any
code duplication.

```rust
pub trait Graph {
    type NodeData;
    type Weight;
    type NodeIter<'a>: Iterator<Item = NodeId> where Self: 'a;
    type NeighborIter<'a>: Iterator<Item = (NodeId, &'a Self::Weight)> where Self: 'a;

    fn add_node(&mut self, data: Self::NodeData) -> NodeId;
    fn add_edge(&mut self, from: NodeId, to: NodeId, weight: Self::Weight)
        -> Result<(), GraphError>;
    fn remove_node(&mut self, id: NodeId) -> Option<Self::NodeData>;

    fn node_count(&self) -> usize;
    fn edge_count(&self) -> usize;
    fn contains_node(&self, id: NodeId) -> bool;
    fn contains_edge(&self, from: NodeId, to: NodeId) -> bool;
    fn degree(&self, id: NodeId) -> usize;
    fn nodes(&self) -> Self::NodeIter<'_>;
    fn neighbors(&self, id: NodeId) -> Self::NeighborIter<'_>;

    // Provided helpers
    fn all_edges(&self) -> Vec<Edge<Self::Weight>> where Self::Weight: Clone { ... }
    fn is_empty(&self) -> bool { ... }
}
```

The iterator associated types use **Generic Associated Types** (GATs, stable
since Rust 1.65). This allows `NodeIter` and `NeighborIter` to borrow directly
from `&self` without any heap allocation, giving zero-copy iteration over graph
internals.

To implement `Graph` for your own representation, provide the two iterator types
and the seven required methods. The trait's provided methods (`all_edges`,
`is_empty`) are free.

### AdjacencyList vs AdjacencyMatrix

|                   | `AdjacencyList<N, W>`   | `AdjacencyMatrix<N, W>` |
|-------------------|-------------------------|-------------------------|
| Memory            | O(V + E)                | O(V²)                   |
| `add_node`        | O(1) amortised          | O(V) (extends all rows) |
| `add_edge`        | O(1) amortised          | O(1)                    |
| `contains_edge`   | O(out-degree)           | **O(1)**                |
| `neighbors`       | O(out-degree) — slice   | O(V) — scans row        |
| Best for          | Sparse graphs, most algorithms | Dense graphs, Floyd-Warshall |

Both support `directed()` and `undirected()` constructors. For undirected
graphs, `add_edge(u, v, w)` automatically inserts both directions.

```rust
use graph_core::{AdjacencyList, AdjacencyMatrix, Graph};

// Directed sparse graph
let mut sparse: AdjacencyList<&str> = AdjacencyList::directed();
let a = sparse.add_node("A");
let b = sparse.add_node("B");
sparse.add_edge(a, b, 2.5).unwrap();
println!("{:?}", sparse.node_data(a)); // Some("A")

// Undirected dense graph with u32 weights
let mut dense: AdjacencyMatrix<(), u32> = AdjacencyMatrix::undirected();
let x = dense.add_node(());
let y = dense.add_node(());
dense.add_edge(x, y, 5).unwrap();
assert!(dense.contains_edge(y, x)); // symmetric
println!("{:?}", dense.edge_weight(x, y)); // Some(5)
```

`AdjacencyList` also provides `node_data(&self, id) -> Option<&N>` and
`node_data_mut(&mut self, id) -> Option<&mut N>` for direct data access.
`AdjacencyMatrix` provides `edge_weight(&self, from, to) -> Option<&W>` for
O(1) weight lookup.

### GraphBuilder

`GraphBuilder` is a fluent API for constructing graphs from static data,
particularly useful in tests and benchmarks.

```rust
use graph_core::{GraphBuilder, Graph};

let g = GraphBuilder::<&str, f64>::new()
    .directed()
    .node("A")  // index 0
    .node("B")  // index 1
    .node("C")  // index 2
    .edge(0, 1, 1.5)
    .edge(1, 2, 2.0)
    .edge(0, 2, 10.0)
    .build_adjacency_list();

assert_eq!(g.node_count(), 3);
assert_eq!(g.edge_count(), 3);
```

Call `.build_adjacency_matrix()` instead to get a dense representation. Both
consume the builder. Passing an out-of-range node index to `.edge()` panics at
build time with a descriptive message.

### GraphError

All fallible operations return `Result<T, GraphError>`:

```rust
use graph_core::GraphError;

// Possible variants:
GraphError::NodeNotFound(id)          // node id doesn't exist
GraphError::EdgeAlreadyExists(u, v)   // duplicate edge
GraphError::SelfLoop(id)              // self-loop disallowed
GraphError::NegativeCycle             // detected by Bellman-Ford / Floyd-Warshall
GraphError::NotConnected              // required by MST, Euler circuit
GraphError::InvalidOperation("…")    // context-specific message
```

`GraphError` implements both `Display` and `std::error::Error`, so it
integrates cleanly with `?` and any error-handling crate.

---

## Algorithms

All algorithms are generic over any type implementing `Graph` (or
`Graph<Weight = f64>` where weights are required). They work identically on
`AdjacencyList` and `AdjacencyMatrix`.

### Traversal

```rust
use graph_core::{AdjacencyList, Graph};
use graph_traversal::*;

let mut g: AdjacencyList<()> = AdjacencyList::directed();
let a = g.add_node(()); let b = g.add_node(()); let c = g.add_node(());
g.add_edge(a, b, 1.0).unwrap();
g.add_edge(b, c, 1.0).unwrap();

// BFS — returns HashMap<NodeId, usize> of hop distances
let dist = bfs(&g, a);
assert_eq!(dist[&c], 2);

// BFS tree — distances + parent map for path reconstruction
let tree = bfs_tree(&g, a);
assert_eq!(tree.parent[&c], Some(b));

// DFS — recursive (post-order finish) and iterative
let mut visited = Vec::new();
let finish = dfs_recursive(&g, a, &mut |id| visited.push(id));
let order  = dfs_iterative(&g, a);

// Path reconstruction from any parent map
let path = reconstruct_path(&tree.parent, a, c); // Some([a, b, c])

// Topological sort (two implementations)
let order_dfs  = topological_sort_dfs(&g).unwrap();     // DFS finish-order reverse
let order_kahn = topological_sort_kahn(&g).unwrap();    // Kahn's BFS-based

// Cycle detection
assert!(!has_cycle_directed(&g));
assert!(!has_cycle_undirected(&g));

// Connected components (undirected)
let mut ug: AdjacencyList<()> = AdjacencyList::undirected();
// … add nodes/edges …
let components: Vec<Vec<NodeId>> = connected_components(&ug);

// Bipartite check
let (bipartite, colouring) = is_bipartite(&ug);
// colouring: HashMap<NodeId, bool> — the 2-colouring if bipartite
```

**`dfs_recursive`** returns nodes in finish order (post-order). Use it when you
need post-order processing (e.g. SCC). **`dfs_iterative`** avoids call-stack
overflow on deep graphs. Both accept a visitor closure called on first
discovery.

### Shortest paths

All shortest-path functions require `Graph<Weight = f64>`.

#### Dijkstra

Single-source shortest paths for non-negative weights. O((V + E) log V) with a
binary min-heap and lazy deletion.

```rust
use graph_shortest_path::{dijkstra, dijkstra::reconstruct_path};

let result = dijkstra(&g, source).unwrap();
// result.distances: HashMap<NodeId, f64>  — distance from source
// result.parents:   HashMap<NodeId, NodeId> — shortest-path tree

let (path, cost) = reconstruct_path(&result, source, goal).unwrap();
// path: Vec<NodeId>, cost: f64
```

#### Bellman-Ford

Handles negative-weight edges. Detects negative cycles and returns
`Err(GraphError::NegativeCycle)`. O(V · E).

```rust
use graph_shortest_path::bellman_ford;

match bellman_ford(&g, source) {
    Ok(result) => {
        // result.distances: HashMap<NodeId, f64>
        // result.parents:   HashMap<NodeId, NodeId>
    }
    Err(GraphError::NegativeCycle) => { /* cycle detected */ }
    _ => unreachable!(),
}
```

#### Floyd-Warshall

All-pairs shortest paths. O(V³) time, O(V²) space. Best used with
`AdjacencyMatrix` since the algorithm naturally maps to a 2-D array.

```rust
use graph_shortest_path::{floyd_warshall, floyd_warshall_with_paths, reconstruct_fw_path};

// Distances only
let dist: Vec<Vec<f64>> = floyd_warshall(&g).unwrap();

// Distances + next-hop table for O(V) path reconstruction
let (dist, next) = floyd_warshall_with_paths(&g).unwrap();
let path = reconstruct_fw_path(&next, u, v); // Option<Vec<NodeId>>
```

#### A\*

Goal-directed search with a caller-supplied heuristic. Falls back to Dijkstra
when `h` always returns `0.0`. O(E log V) in the best case.

```rust
use graph_shortest_path::astar;

// Heuristic: straight-line distance to goal (must be admissible)
let h = |node: NodeId| -> f64 { euclidean_dist(node, goal) };

let (path, cost) = astar(&g, source, goal, h).unwrap();
```

### Spanning trees

Both `kruskal` and `prim` return `Option<SpanningTree>` — `None` if the graph
is disconnected or empty.

```rust
use graph_spanning::{kruskal, prim, SpanningTree};

let mst: Option<SpanningTree> = kruskal(&g);  // O(E log E)
let mst: Option<SpanningTree> = prim(&g);     // O((V+E) log V)

if let Some(tree) = mst {
    println!("MST weight: {}", tree.total_weight);
    for edge in &tree.edges {
        println!("  {:?} → {:?}  w={}", edge.source, edge.target, edge.weight);
    }
}
```

`SpanningTree` contains `edges: Vec<Edge<f64>>` and `total_weight: f64`. On
the same connected graph, `kruskal` and `prim` always agree on `total_weight`
(though the edge sets may differ for equal-weight edges).

#### Bridges and articulation points

Tarjan's O(V + E) DFS-based algorithms for finding cut edges and cut vertices:

```rust
use graph_spanning::{bridges, articulation_points};

let cut_edges:    Vec<(NodeId, NodeId)> = bridges(&g);
let cut_vertices: Vec<NodeId>           = articulation_points(&g);
```

Removing any bridge disconnects the graph. Removing any articulation point
increases the number of connected components.

### Maximum flow

Flow algorithms use `FlowGraph` rather than the generic `Graph` trait. See the
[design note in `graph-flow`](#design-note-flowgraph) for why.

```rust
use graph_flow::{FlowGraph, edmonds_karp, ford_fulkerson, min_cut};

let mut fg = FlowGraph::new(6); // 6 nodes, indexed 0..5
fg.add_edge(0, 1, 10.0);
fg.add_edge(0, 2, 8.0);
fg.add_edge(1, 3, 5.0);
fg.add_edge(2, 3, 7.0);
fg.add_edge(3, 5, 15.0);

// Edmonds-Karp: O(V·E²) — BFS augmenting paths, preferred over Ford-Fulkerson
let max_flow = edmonds_karp(&mut fg, 0, 5);

// Ford-Fulkerson: O(E · max_flow) — DFS augmenting paths
fg.reset_flow(); // reuse graph, zero all flows
let max_flow = ford_fulkerson(&mut fg, 0, 5);

// Min s-t cut from a completed max-flow (Max-Flow Min-Cut theorem)
let MinCut { source_side, sink_side, cut_edges } = min_cut(&fg, 0);
```

`fg.add_edge(u, v, cap)` automatically inserts the reverse residual edge.
`fg.reset_flow()` zeroes all flows so the same graph can be reused.

#### Hopcroft-Karp bipartite matching

Finds a **maximum cardinality matching** in a bipartite graph in O(E√V):

```rust
use graph_flow::hopcroft_karp;

// left nodes: 0..L, right nodes: 0..R
let result = hopcroft_karp(L, R, &edges); // edges: &[(usize, usize)]
// result.matching_size: usize
// result.match_left:  Vec<Option<usize>> — which right node each left node is matched to
// result.match_right: Vec<Option<usize>> — which left node each right node is matched to
```

#### Design note: FlowGraph

The residual graph needs to update both a forward edge and its reverse edge in
a single pass. Rust's borrow checker disallows two simultaneous mutable
references into the same `Vec`. `FlowGraph` solves this with index-based access:
each `FlowEdge` stores the index of its reverse edge in the target's adjacency
list (`rev: usize`), enabling O(1) residual updates with no unsafe code.

### Advanced

#### Strongly connected components

Both algorithms solve SCC in O(V + E). Tarjan's is a single DFS pass using a
stack and low-link values. Kosaraju's uses two DFS passes (forward + transposed
graph). Both return `Vec<Vec<NodeId>>` — a list of components, each a list of
nodes.

```rust
use graph_advanced::{tarjan_scc, kosaraju_scc};

let sccs: Vec<Vec<NodeId>> = tarjan_scc(&g);
let sccs: Vec<Vec<NodeId>> = kosaraju_scc(&g);

// Verify they agree
assert_eq!(sccs.len(), kosaraju_scc(&g).len());
```

#### DAG condensation

Collapse each SCC into a single super-node, producing a DAG:

```rust
use graph_advanced::{condensation, CondensedGraph};

let dag: CondensedGraph = condensation(&g);
// dag.components[i]: Vec<NodeId> — original nodes in super-node i
// dag.edges[i]:      Vec<usize>  — super-nodes that i has edges to
// dag.node_count():  usize
```

#### Euler path and circuit

Hierholzer's algorithm. O(E).

```rust
use graph_advanced::{euler_circuit, euler_path, EulerError};

match euler_circuit(&g) {
    Ok((circuit, start)) => { /* circuit: Vec<NodeId> visiting every edge once */ }
    Err(EulerError::NotConnected)   => { /* graph isn't connected */ }
    Err(EulerError::NoEulerCircuit) => { /* not all vertices have even degree */ }
    Err(EulerError::EmptyGraph)     => {}
}

// Euler path (visits every edge once, start ≠ end)
match euler_path(&g) {
    Ok((path, start, end)) => { /* exactly two odd-degree vertices */ }
    Err(e) => { /* … */ }
}
```

#### Hamiltonian path

Exact backtracking. O(V!). Use only for small graphs (V ≤ 12 or so).

```rust
use graph_advanced::hamiltonian_path;

let path: Option<Vec<NodeId>> = hamiltonian_path(&g, start);
```

#### Travelling Salesman — Held-Karp DP

Exact bitmask DP. O(2^V · V²) time, O(2^V · V) space. Practical for V ≤ 20.

```rust
use graph_advanced::tsp_held_karp;

// dist[i][j]: cost from i to j. Use f64::INFINITY for missing edges.
let dist = vec![
    vec![0.0, 1.0, 15.0, 6.0],
    vec![2.0, 0.0, 7.0, 3.0],
    vec![9.0, 6.0, 0.0, 12.0],
    vec![10.0, 4.0, 8.0, 0.0],
];

let (cost, tour) = tsp_held_karp(&dist).unwrap();
// tour: Vec<usize> — node visit order, starts and ends at 0
```

---

## Collections

`graph-collections` provides the data structures used internally by the
algorithms. They are also available as standalone types.

| Type | Description |
|---|---|
| `Stack<T>` | LIFO stack backed by `Vec` |
| `Queue<T>` | FIFO queue backed by two `Vec`s (amortised O(1) dequeue) |
| `Deque<T>` | Double-ended queue |
| `MinHeap<T>` | Binary min-heap (raw array, no key update) |
| `PriorityQueue<T, P>` | Priority queue with explicit priority type |
| `DisjointSet` | Union-Find with union-by-rank and path compression |

```rust
use graph_collections::{Stack, Queue, MinHeap, DisjointSet};

let mut s: Stack<i32> = Stack::new();
s.push(1); s.push(2);
assert_eq!(s.pop(), Some(2));

let mut uf = DisjointSet::new(5);
uf.union(0, 1); uf.union(1, 2);
assert!(uf.connected(0, 2));
assert!(!uf.connected(0, 3));
assert_eq!(uf.count(), 3); // three disjoint sets remain
```

`DisjointSet` uses **union-by-rank** and **path compression**, giving an
amortised O(α(n)) cost per operation where α is the inverse Ackermann function
— effectively constant for all practical inputs.

---

## Property-based tests

The `graph` meta-crate includes a `proptest` suite that generates random graphs
and verifies algorithm invariants across thousands of inputs:

```bash
# Default number of cases (256)
cargo test -p graph proptest

# Stress with 5000 cases
PROPTEST_CASES=5000 cargo test -p graph proptest
```

Some invariants checked:

- Dijkstra and Bellman-Ford agree on all distances for non-negative graphs.
- `tarjan_scc` and `kosaraju_scc` return the same component count and sizes.
- Kruskal and Prim return the same `total_weight`.
- Every edge in a min-cut result crosses the partition (source-side → sink-side).
- BFS distances are non-decreasing along any path.

---

## Benchmarks

Criterion benchmarks are in `crates/graph/benches/` and cover BFS, Dijkstra,
Kruskal, and Floyd-Warshall on random graphs of increasing size.

```bash
# Run all benchmarks (HTML report in target/criterion/)
cargo bench -p graph

# Run one benchmark suite
cargo bench -p graph --bench dijkstra
```

Open `target/criterion/report/index.html` in a browser to see the full Criterion
report with per-benchmark timing, regression detection, and violin plots.

---

## Development commands

```bash
# Run the full test suite across all crates
cargo test --all

# Lint (warnings promoted to errors, matching CI)
cargo clippy --all -- -D warnings

# Format check
cargo fmt --all -- --check

# Build and open docs locally
cargo doc --workspace --no-deps --open

# Run property-based tests with extra cases
PROPTEST_CASES=5000 cargo test -p graph proptest

# Run benchmarks
cargo bench -p graph
```

---

## Algorithm complexity reference

| Algorithm | Crate | Time | Space |
|---|---|---|---|
| BFS / DFS | graph-traversal | O(V+E) | O(V) |
| Topological sort (DFS / Kahn) | graph-traversal | O(V+E) | O(V) |
| Cycle detection | graph-traversal | O(V+E) | O(V) |
| Connected components | graph-traversal | O(V+E) | O(V) |
| Bipartite check | graph-traversal | O(V+E) | O(V) |
| Dijkstra | graph-shortest-path | O((V+E) log V) | O(V) |
| Bellman-Ford | graph-shortest-path | O(V·E) | O(V) |
| Floyd-Warshall | graph-shortest-path | O(V³) | O(V²) |
| A\* | graph-shortest-path | O(E log V) | O(V) |
| Kruskal MST | graph-spanning | O(E log E) | O(V) |
| Prim MST | graph-spanning | O((V+E) log V) | O(V) |
| Bridges / articulation points | graph-spanning | O(V+E) | O(V) |
| Ford-Fulkerson max flow | graph-flow | O(E · max\_flow) | O(V+E) |
| Edmonds-Karp max flow | graph-flow | O(V·E²) | O(V+E) |
| Min s-t cut | graph-flow | O(V+E) | O(V) |
| Hopcroft-Karp matching | graph-flow | O(E√V) | O(V) |
| Tarjan / Kosaraju SCC | graph-advanced | O(V+E) | O(V) |
| DAG condensation | graph-advanced | O(V+E) | O(V) |
| Hierholzer Euler path/circuit | graph-advanced | O(E) | O(E) |
| Hamiltonian path (backtrack) | graph-advanced | O(V!) | O(V) |
| TSP Held-Karp DP | graph-advanced | O(2^V · V²) | O(2^V · V) |
| DisjointSet (union / find) | graph-collections | O(α(n)) amortised | O(n) |

---

## License

Licensed under either of [Apache License 2.0](LICENSE-APACHE) or
[MIT License](LICENSE-MIT) at your option.
