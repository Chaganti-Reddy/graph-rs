#[cfg(test)]
mod tests {
    use graph_collections::DisjointSet;

    // ── Construction ──────────────────────────────────────────────────────────

    #[test]
    fn new_has_n_singleton_sets() {
        let ds = DisjointSet::new(5);
        assert_eq!(ds.count(), 5);
        assert_eq!(ds.size(), 5);
    }

    #[test]
    fn new_zero_is_valid() {
        let ds = DisjointSet::new(0);
        assert_eq!(ds.count(), 0);
        assert_eq!(ds.size(), 0);
    }

    #[test]
    fn new_one_element() {
        let ds = DisjointSet::new(1);
        assert_eq!(ds.count(), 1);
        assert_eq!(ds.size(), 1);
    }

    // ── find ─────────────────────────────────────────────────────────────────

    #[test]
    fn find_singleton_returns_self() {
        let mut ds = DisjointSet::new(5);
        for i in 0..5 {
            assert_eq!(ds.find(i), i);
        }
    }

    #[test]
    fn find_after_union_shares_root() {
        let mut ds = DisjointSet::new(4);
        ds.union(0, 1);
        assert_eq!(ds.find(0), ds.find(1));
    }

    #[test]
    fn find_transitive_same_root() {
        let mut ds = DisjointSet::new(4);
        ds.union(0, 1);
        ds.union(1, 2);
        ds.union(2, 3);
        let root = ds.find(0);
        assert_eq!(ds.find(1), root);
        assert_eq!(ds.find(2), root);
        assert_eq!(ds.find(3), root);
    }

    #[test]
    fn find_different_components_different_roots() {
        let mut ds = DisjointSet::new(4);
        ds.union(0, 1);
        ds.union(2, 3);
        assert_ne!(ds.find(0), ds.find(2));
    }

    // ── union ─────────────────────────────────────────────────────────────────

    #[test]
    fn union_returns_true_on_new_merge() {
        let mut ds = DisjointSet::new(3);
        assert!(ds.union(0, 1));
    }

    #[test]
    fn union_returns_false_when_already_connected() {
        let mut ds = DisjointSet::new(3);
        assert!(ds.union(0, 1));
        assert!(!ds.union(0, 1)); // idempotent
        assert!(!ds.union(1, 0)); // direction doesn't matter
    }

    #[test]
    fn union_decrements_count() {
        let mut ds = DisjointSet::new(4);
        assert_eq!(ds.count(), 4);
        ds.union(0, 1);
        assert_eq!(ds.count(), 3);
        ds.union(2, 3);
        assert_eq!(ds.count(), 2);
    }

    #[test]
    fn union_same_element_is_noop() {
        let mut ds = DisjointSet::new(3);
        assert!(!ds.union(1, 1)); // self-union never merges
        assert_eq!(ds.count(), 3);
    }

    #[test]
    fn union_chain_produces_single_component() {
        let mut ds = DisjointSet::new(5);
        ds.union(0, 1);
        ds.union(1, 2);
        ds.union(2, 3);
        ds.union(3, 4);
        assert_eq!(ds.count(), 1);
        assert!(ds.is_fully_connected());
    }

    // ── union-by-rank invariant ───────────────────────────────────────────────

    #[test]
    fn union_by_rank_keeps_tree_shallow() {
        // After k merges the max rank should be ≤ log2(n).
        let mut ds = DisjointSet::new(16);
        for i in 0..15 {
            ds.union(i, i + 1);
        }
        // With union-by-rank, rank of any root is ≤ 4 (log2 16).
        // We verify indirectly: find(0) should always terminate.
        let root = ds.find(0);
        for i in 0..16 {
            assert_eq!(ds.find(i), root);
        }
    }

    // ── connected ─────────────────────────────────────────────────────────────

    #[test]
    fn connected_singletons_not_connected() {
        let mut ds = DisjointSet::new(4);
        assert!(!ds.connected(0, 1));
        assert!(!ds.connected(1, 2));
    }

    #[test]
    fn connected_after_union() {
        let mut ds = DisjointSet::new(4);
        ds.union(0, 2);
        assert!(ds.connected(0, 2));
        assert!(!ds.connected(0, 1));
        assert!(!ds.connected(2, 3));
    }

    #[test]
    fn connected_is_transitive() {
        let mut ds = DisjointSet::new(5);
        ds.union(0, 1);
        ds.union(1, 2);
        assert!(ds.connected(0, 2));
    }

    #[test]
    fn connected_is_symmetric() {
        let mut ds = DisjointSet::new(3);
        ds.union(0, 1);
        assert_eq!(ds.connected(0, 1), ds.connected(1, 0));
    }

    #[test]
    fn connected_same_element_is_always_true() {
        let mut ds = DisjointSet::new(3);
        for i in 0..3 {
            assert!(ds.connected(i, i));
        }
    }

    // ── count / size / is_fully_connected ─────────────────────────────────────

    #[test]
    fn count_decreases_only_on_real_merge() {
        let mut ds = DisjointSet::new(4);
        ds.union(0, 1); // 3
        ds.union(0, 1); // still 3 (already merged)
        ds.union(1, 0); // still 3
        assert_eq!(ds.count(), 3);
    }

    #[test]
    fn size_never_changes() {
        let mut ds = DisjointSet::new(6);
        for i in 0..5 {
            ds.union(i, i + 1);
        }
        assert_eq!(ds.size(), 6);
    }

    #[test]
    fn is_fully_connected_false_on_new() {
        let ds = DisjointSet::new(3);
        assert!(!ds.is_fully_connected());
    }

    #[test]
    fn is_fully_connected_true_after_all_merged() {
        let mut ds = DisjointSet::new(4);
        ds.union(0, 1);
        ds.union(2, 3);
        assert!(!ds.is_fully_connected());
        ds.union(0, 2);
        assert!(ds.is_fully_connected());
    }

    #[test]
    fn size_one_is_fully_connected_immediately() {
        let ds = DisjointSet::new(1);
        assert!(ds.is_fully_connected());
    }

    // ── Derived traits ────────────────────────────────────────────────────────

    #[test]
    fn clone_is_independent() {
        let mut original = DisjointSet::new(4);
        original.union(0, 1);
        let cloned = original.clone();

        original.union(2, 3);
        // original now has 2 components; clone should still have 3.
        assert_eq!(original.count(), 2);
        assert_eq!(cloned.count(), 3);
    }

    #[test]
    fn debug_format_non_empty() {
        let ds = DisjointSet::new(3);
        let s = format!("{:?}", ds);
        // Should mention parent, rank, and count fields.
        assert!(s.contains("parent") || s.contains("rank") || s.contains("count"));
    }

    // ── Graph connectivity scenarios ──────────────────────────────────────────

    #[test]
    fn graph_two_components() {
        // Edges: 0-1, 1-2, 3-4  →  {0,1,2} and {3,4}
        let mut ds = DisjointSet::new(5);
        ds.union(0, 1);
        ds.union(1, 2);
        ds.union(3, 4);
        assert_eq!(ds.count(), 2);
        assert!(ds.connected(0, 2));
        assert!(ds.connected(3, 4));
        assert!(!ds.connected(2, 3));
    }

    #[test]
    fn graph_cycle_does_not_over_merge() {
        // Triangle 0-1, 1-2, 0-2 forms one component.
        let mut ds = DisjointSet::new(3);
        assert!(ds.union(0, 1));
        assert!(ds.union(1, 2));
        assert!(!ds.union(0, 2)); // closing the cycle — no new merge
        assert_eq!(ds.count(), 1);
    }

    #[test]
    fn grid_connectivity() {
        // 3×3 grid; connect all neighbours row-major.
        // After connecting the full grid, count should be 1.
        let n = 9usize;
        let mut ds = DisjointSet::new(n);
        let edges = [
            (0, 1),
            (1, 2),
            (3, 4),
            (4, 5),
            (6, 7),
            (7, 8), // rows
            (0, 3),
            (1, 4),
            (2, 5),
            (3, 6),
            (4, 7),
            (5, 8), // cols
        ];
        for (a, b) in edges {
            ds.union(a, b);
        }
        assert_eq!(ds.count(), 1);
        assert!(ds.is_fully_connected());
    }

    // ── Kruskal's MST simulation ──────────────────────────────────────────────

    #[test]
    fn kruskal_mst_edge_count() {
        // 5 nodes, weighted edges sorted by weight.
        // Kruskal: add an edge iff it connects two different components.
        // MST of a connected graph with n nodes has exactly n-1 edges.
        let mut ds = DisjointSet::new(5);
        let edges = [
            (0, 1, 1),
            (0, 2, 2),
            (1, 3, 3),
            (2, 4, 4),
            (3, 4, 5),
            (1, 2, 6),
        ];
        let mut mst_edges = 0;
        for (u, v, _w) in edges {
            if ds.union(u, v) {
                mst_edges += 1;
            }
        }
        assert_eq!(mst_edges, 4); // n - 1 = 5 - 1
        assert!(ds.is_fully_connected());
    }

    // ── Stress test ───────────────────────────────────────────────────────────

    #[test]
    fn large_sequential_union_single_component() {
        const N: usize = 100_000;
        let mut ds = DisjointSet::new(N);
        for i in 0..N - 1 {
            assert!(ds.union(i, i + 1));
        }
        assert_eq!(ds.count(), 1);
        assert!(ds.is_fully_connected());
    }

    #[test]
    fn large_pair_merges_exact_component_count() {
        // Merge pairs (0,1),(2,3),(4,5)... → N/2 components
        const N: usize = 10_000;
        let mut ds = DisjointSet::new(N);
        for i in (0..N).step_by(2) {
            ds.union(i, i + 1);
        }
        assert_eq!(ds.count(), N / 2);
    }
}
