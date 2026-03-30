#[cfg(test)]
mod tests {
    use graph_collections::PriorityQueue;

    // ── Construction ──────────────────────────────────────────────────────────

    #[test]
    fn new_pq_is_empty() {
        let pq: PriorityQueue<&str, u32> = PriorityQueue::new();
        assert!(pq.is_empty());
        assert_eq!(pq.len(), 0);
    }

    #[test]
    fn default_pq_is_empty() {
        let pq: PriorityQueue<i32, i32> = PriorityQueue::default();
        assert!(pq.is_empty());
    }

    #[test]
    fn with_capacity_starts_empty() {
        let pq: PriorityQueue<u32, u32> = PriorityQueue::with_capacity(64);
        assert!(pq.is_empty());
        assert_eq!(pq.len(), 0);
    }

    // ── peek_priority / peek_value ────────────────────────────────────────────

    #[test]
    fn peek_priority_empty_is_none() {
        let pq: PriorityQueue<i32, i32> = PriorityQueue::new();
        assert_eq!(pq.peek_priority(), None);
    }

    #[test]
    fn peek_value_empty_is_none() {
        let pq: PriorityQueue<i32, i32> = PriorityQueue::new();
        assert_eq!(pq.peek_value(), None);
    }

    #[test]
    fn peek_priority_returns_lowest() {
        let mut pq = PriorityQueue::new();
        pq.push("c", 3u32);
        pq.push("a", 1);
        pq.push("b", 2);
        assert_eq!(pq.peek_priority(), Some(&1));
    }

    #[test]
    fn peek_value_returns_highest_priority_value() {
        let mut pq = PriorityQueue::new();
        pq.push("low-pri", 9u32);
        pq.push("high-pri", 1);
        assert_eq!(pq.peek_value(), Some(&"high-pri"));
    }

    #[test]
    fn peek_does_not_consume() {
        let mut pq = PriorityQueue::new();
        pq.push("x", 5u32);
        let _ = pq.peek_priority();
        let _ = pq.peek_value();
        assert_eq!(pq.len(), 1);
    }

    // ── push ─────────────────────────────────────────────────────────────────

    #[test]
    fn push_single_element() {
        let mut pq = PriorityQueue::new();
        pq.push("only", 42u32);
        assert_eq!(pq.len(), 1);
        assert_eq!(pq.peek_priority(), Some(&42));
    }

    #[test]
    fn push_updates_minimum_priority() {
        let mut pq = PriorityQueue::new();
        pq.push("a", 10u32);
        assert_eq!(pq.peek_priority(), Some(&10));
        pq.push("b", 5);
        assert_eq!(pq.peek_priority(), Some(&5));
        pq.push("c", 1);
        assert_eq!(pq.peek_priority(), Some(&1));
    }

    // ── pop ──────────────────────────────────────────────────────────────────

    #[test]
    fn pop_empty_returns_none() {
        let mut pq: PriorityQueue<i32, i32> = PriorityQueue::new();
        assert_eq!(pq.pop(), None);
    }

    #[test]
    fn pop_returns_lowest_priority_first() {
        let mut pq = PriorityQueue::new();
        pq.push("low", 10u32);
        pq.push("high", 1);
        pq.push("medium", 5);

        assert_eq!(pq.pop(), Some(("high", 1)));
        assert_eq!(pq.pop(), Some(("medium", 5)));
        assert_eq!(pq.pop(), Some(("low", 10)));
        assert_eq!(pq.pop(), None);
    }

    #[test]
    fn pop_decrements_len() {
        let mut pq = PriorityQueue::new();
        pq.push(1u32, 1u32);
        pq.push(2, 2);
        pq.pop();
        assert_eq!(pq.len(), 1);
    }

    #[test]
    fn pop_single_element() {
        let mut pq = PriorityQueue::new();
        pq.push("only", 7u32);
        assert_eq!(pq.pop(), Some(("only", 7)));
        assert!(pq.is_empty());
    }

    // ── duplicate priorities ──────────────────────────────────────────────────

    #[test]
    fn duplicate_priorities_all_returned() {
        let mut pq = PriorityQueue::new();
        pq.push("a", 2u32);
        pq.push("b", 2);
        pq.push("c", 2);
        // All three have priority 2 — all should come back.
        let results = vec![pq.pop(), pq.pop(), pq.pop()];
        assert_eq!(pq.pop(), None);
        // Each result has priority 2.
        for r in &results {
            assert_eq!(r.as_ref().map(|(_, p)| p), Some(&2));
        }
    }

    // ── len / is_empty ────────────────────────────────────────────────────────

    #[test]
    fn len_tracks_push_and_pop() {
        let mut pq: PriorityQueue<u32, u32> = PriorityQueue::new();
        assert_eq!(pq.len(), 0);
        pq.push(1, 1);
        assert_eq!(pq.len(), 1);
        pq.push(2, 2);
        assert_eq!(pq.len(), 2);
        pq.pop();
        assert_eq!(pq.len(), 1);
        pq.pop();
        assert_eq!(pq.len(), 0);
        assert!(pq.is_empty());
    }

    // ── clear ─────────────────────────────────────────────────────────────────

    #[test]
    fn clear_empties_queue() {
        let mut pq: PriorityQueue<u32, u32> = PriorityQueue::new();
        for i in 0..10 {
            pq.push(i, i);
        }
        pq.clear();
        assert!(pq.is_empty());
        assert_eq!(pq.len(), 0);
    }

    #[test]
    fn clear_on_empty_is_noop() {
        let mut pq: PriorityQueue<i32, i32> = PriorityQueue::new();
        pq.clear();
        assert!(pq.is_empty());
    }

    // ── FromIterator / Extend ─────────────────────────────────────────────────

    #[test]
    fn collect_from_vec_of_pairs() {
        let pq: PriorityQueue<&str, u32> = vec![("slow", 10u32), ("fast", 1), ("medium", 5)]
            .into_iter()
            .collect();

        assert_eq!(pq.peek_priority(), Some(&1));
        assert_eq!(pq.len(), 3);
    }

    #[test]
    fn extend_appends_and_maintains_order() {
        let mut pq: PriorityQueue<u32, u32> = PriorityQueue::new();
        pq.push(100, 10);
        pq.extend([(200u32, 2u32), (300, 5), (400, 1)]);
        assert_eq!(pq.peek_priority(), Some(&1));
        assert_eq!(pq.len(), 4);
    }

    #[test]
    fn extend_empty_is_noop() {
        let mut pq: PriorityQueue<u32, u32> = PriorityQueue::new();
        pq.push(1, 1);
        pq.extend(std::iter::empty::<(u32, u32)>());
        assert_eq!(pq.len(), 1);
    }

    // ── Derived traits ────────────────────────────────────────────────────────

    #[test]
    fn clone_is_independent() {
        let mut original: PriorityQueue<u32, u32> = PriorityQueue::new();
        original.push(1, 5);
        original.push(2, 3);
        let mut cloned = original.clone();

        original.pop();
        cloned.push(3, 1);

        assert_eq!(original.len(), 1);
        assert_eq!(cloned.len(), 3);
        assert_eq!(cloned.peek_priority(), Some(&1));
    }

    #[test]
    fn debug_format_non_empty() {
        let mut pq: PriorityQueue<u32, u32> = PriorityQueue::new();
        pq.push(42, 7);
        let s = format!("{:?}", pq);
        assert!(s.contains("42") || s.contains("7")); // internal repr
    }

    // ── Generic / type coverage ───────────────────────────────────────────────

    #[test]
    fn works_with_string_values() {
        let mut pq: PriorityQueue<String, u32> = PriorityQueue::new();
        pq.push("world".to_string(), 2);
        pq.push("hello".to_string(), 1);
        assert_eq!(pq.pop().map(|(v, _)| v), Some("hello".to_string()));
    }

    #[test]
    fn works_with_negative_priorities() {
        let mut pq: PriorityQueue<&str, i32> = PriorityQueue::new();
        pq.push("most urgent", -10);
        pq.push("urgent", -5);
        pq.push("normal", 0);
        assert_eq!(pq.pop(), Some(("most urgent", -10)));
    }

    #[test]
    fn dijkstra_like_usage() {
        // Simulate popping nodes by shortest tentative distance.
        let mut pq: PriorityQueue<u32, u32> = PriorityQueue::new();
        // (node_id, dist)
        pq.push(0, 0);
        pq.push(1, 4);
        pq.push(2, 2);
        pq.push(3, 7);
        pq.push(4, 1);

        let mut order = Vec::new();
        while let Some((node, dist)) = pq.pop() {
            order.push((node, dist));
        }
        // Should come out in ascending distance order
        let dists: Vec<u32> = order.iter().map(|(_, d)| *d).collect();
        let mut sorted = dists.clone();
        sorted.sort();
        assert_eq!(dists, sorted);
    }

    // ── Stress test ───────────────────────────────────────────────────────────

    #[test]
    fn large_priority_queue_sorted_output() {
        const N: u32 = 50_000;
        let mut pq: PriorityQueue<u32, u32> = PriorityQueue::with_capacity(N as usize);
        for i in (0..N).rev() {
            pq.push(i, i);
        }
        let mut prev_p = 0u32;
        while let Some((_, p)) = pq.pop() {
            assert!(p >= prev_p);
            prev_p = p;
        }
    }
}
