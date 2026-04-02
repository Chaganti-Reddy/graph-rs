/// Solves the **Travelling Salesman Problem** exactly using the Held-Karp
/// bitmask dynamic programming algorithm.
///
/// Given a complete weighted directed graph represented as a distance matrix,
/// finds the minimum-cost Hamiltonian circuit starting and ending at node `0`.
///
/// # Algorithm
///
/// Let `dp[mask][v]` be the minimum cost to reach node `v` having visited
/// exactly the nodes encoded in `mask` (bit `i` set ⟹ node `i` visited),
/// starting from node `0`.
///
/// **Recurrence:**  
/// `dp[mask | (1 << u)][u] = min(dp[mask][v] + dist[v][u])`  
/// for all `v` in `mask` and `u` not in `mask`.
///
/// **Answer:**  
/// `min over v of (dp[all_visited][v] + dist[v][0])`
///
/// # Arguments
///
/// - `dist` — `n × n` distance matrix. `dist[i][j]` is the cost of going
///   from node `i` to node `j`. Use `f64::INFINITY` for missing edges.
///
/// # Returns
///
/// `Some((cost, path))` — the minimum tour cost and the node visit order
/// (starts and ends at node `0`).  
/// `None` — no Hamiltonian circuit exists (some required edge is `INFINITY`),
/// or `dist` is empty.
///
/// # Complexity
///
/// O(2^n · n²) time, O(2^n · n) space. Practical for n ≤ 20.
///
/// # Panics
///
/// Panics if `dist` is not square (all rows must have the same length as the
/// number of rows).
///
/// # Examples
///
/// ```
/// use graph_advanced::tsp_held_karp;
///
/// // 4-node complete graph with known optimal tour cost 35.
/// //        0   1   2   3
/// let dist = vec![
///     vec![0.0, 10.0, 15.0, 20.0], // from 0
///     vec![10.0, 0.0, 35.0, 25.0], // from 1
///     vec![15.0, 35.0, 0.0, 30.0], // from 2
///     vec![20.0, 25.0, 30.0, 0.0], // from 3
/// ];
///
/// let (cost, path) = tsp_held_karp(&dist).unwrap();
/// assert_eq!(cost, 80.0); // 0→1(10)+1→3(25)+3→2(30)+2→0(15)
/// assert_eq!(path.first(), Some(&0));
/// assert_eq!(path.last(), Some(&0));
/// assert_eq!(path.len(), 5); // 4 nodes + return to 0
/// ```
pub fn tsp_held_karp(dist: &[Vec<f64>]) -> Option<(f64, Vec<usize>)> {
    let n = dist.len();
    if n == 0 {
        return None;
    }
    assert!(
        dist.iter().all(|row| row.len() == n),
        "distance matrix must be square"
    );
    if n == 1 {
        return Some((0.0, vec![0, 0]));
    }

    let full_mask = (1usize << n) - 1;

    // dp[mask][v] = min cost to reach v having visited exactly nodes in mask.
    // parent[mask][v] = the node we came from to achieve dp[mask][v].
    let mut dp = vec![vec![f64::INFINITY; n]; 1 << n];
    let mut parent: Vec<Vec<usize>> = vec![vec![usize::MAX; n]; 1 << n];

    // Start at node 0.
    dp[1][0] = 0.0;

    for mask in 1..=(full_mask) {
        // Only process masks that include node 0 (bit 0 always set).
        if mask & 1 == 0 {
            continue;
        }
        for v in 0..n {
            if mask & (1 << v) == 0 {
                continue; // v not in mask
            }
            if dp[mask][v] == f64::INFINITY {
                continue;
            }
            // Try extending to each unvisited node u.
            for u in 0..n {
                if mask & (1 << u) != 0 {
                    continue; // u already visited
                }
                let cost = dp[mask][v] + dist[v][u];
                let next_mask = mask | (1 << u);
                if cost < dp[next_mask][u] {
                    dp[next_mask][u] = cost;
                    parent[next_mask][u] = v;
                }
            }
        }
    }

    // Find the minimum cost to complete the circuit: return to node 0.
    let mut best_cost = f64::INFINITY;
    let mut last_node = usize::MAX;

    for v in 1..n {
        if dp[full_mask][v] == f64::INFINITY {
            continue;
        }
        let total = dp[full_mask][v] + dist[v][0];
        if total < best_cost {
            best_cost = total;
            last_node = v;
        }
    }

    if best_cost == f64::INFINITY {
        return None; // No Hamiltonian circuit exists.
    }

    // Reconstruct path by following parent pointers.
    let mut path = Vec::with_capacity(n + 1);
    let mut mask = full_mask;
    let mut curr = last_node;

    while curr != usize::MAX && mask != 0 {
        path.push(curr);
        let prev = parent[mask][curr];
        mask ^= 1 << curr;
        curr = prev;
    }

    path.reverse();
    path.push(0); // Return to start.

    Some((best_cost, path))
}
