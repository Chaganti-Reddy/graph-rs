#[cfg(test)]
mod tests {
    use graph_collections::MinHeap;

    // ── Construction ──────────────────────────────────────────────────────────

    #[test]
    fn new_heap_is_empty() {
        let h: MinHeap<i32> = MinHeap::new();
        assert!(h.is_empty());
        assert_eq!(h.len(), 0);
    }

    #[test]
    fn default_heap_is_empty() {
        let h: MinHeap<i32> = MinHeap::default();
        assert!(h.is_empty());
    }

    #[test]
    fn with_capacity_starts_empty() {
        let h: MinHeap<i32> = MinHeap::with_capacity(64);
        assert!(h.is_empty());
        assert_eq!(h.len(), 0);
    }

    // ── peek ─────────────────────────────────────────────────────────────────

    #[test]
    fn peek_empty_is_none() {
        let h: MinHeap<i32> = MinHeap::new();
        assert_eq!(h.peek(), None);
    }

    #[test]
    fn peek_returns_minimum() {
        let mut h = MinHeap::new();
        h.push(5u32);
        h.push(1);
        h.push(3);
        assert_eq!(h.peek(), Some(&1));
    }

    #[test]
    fn peek_does_not_consume() {
        let mut h = MinHeap::new();
        h.push(42u32);
        let _ = h.peek();
        assert_eq!(h.len(), 1);
    }

    // ── push ─────────────────────────────────────────────────────────────────

    #[test]
    fn push_single_element() {
        let mut h = MinHeap::new();
        h.push(7u32);
        assert_eq!(h.len(), 1);
        assert_eq!(h.peek(), Some(&7));
    }

    #[test]
    fn push_ascending_order_heap_invariant() {
        let mut h = MinHeap::new();
        for i in [5u32, 3, 8, 1, 4, 2, 7, 6] {
            h.push(i);
            // invariant: peek is always the current minimum
            let min = *h.peek().unwrap();
            assert!(h.clone().into_iter().all(|x| x >= min));
        }
    }

    #[test]
    fn push_descending_preserves_invariant() {
        let mut h = MinHeap::new();
        for i in (1u32..=10).rev() {
            h.push(i);
        }
        assert_eq!(h.peek(), Some(&1));
    }

    // ── pop ──────────────────────────────────────────────────────────────────

    #[test]
    fn pop_empty_returns_none() {
        let mut h: MinHeap<i32> = MinHeap::new();
        assert_eq!(h.pop(), None);
    }

    #[test]
    fn pop_returns_ascending_order() {
        let mut h: MinHeap<u32> = [3, 1, 4, 1, 5, 9, 2, 6].iter().copied().collect();
        let mut out = Vec::new();
        while let Some(v) = h.pop() {
            out.push(v);
        }
        let mut expected = out.clone();
        expected.sort();
        assert_eq!(out, expected);
    }

    #[test]
    fn pop_single_element() {
        let mut h = MinHeap::new();
        h.push(99u32);
        assert_eq!(h.pop(), Some(99));
        assert!(h.is_empty());
        assert_eq!(h.pop(), None);
    }

    #[test]
    fn pop_decrements_len() {
        let mut h: MinHeap<u32> = (1..=5).collect();
        h.pop();
        assert_eq!(h.len(), 4);
    }

    // ── len / is_empty ────────────────────────────────────────────────────────

    #[test]
    fn len_tracks_push_and_pop() {
        let mut h = MinHeap::new();
        assert_eq!(h.len(), 0);
        h.push(1u32);
        assert_eq!(h.len(), 1);
        h.push(2);
        assert_eq!(h.len(), 2);
        h.pop();
        assert_eq!(h.len(), 1);
        h.pop();
        assert_eq!(h.len(), 0);
        assert!(h.is_empty());
    }

    // ── clear ─────────────────────────────────────────────────────────────────

    #[test]
    fn clear_empties_heap() {
        let mut h: MinHeap<u32> = (1..=10).collect();
        h.clear();
        assert!(h.is_empty());
        assert_eq!(h.len(), 0);
    }

    #[test]
    fn clear_on_empty_is_noop() {
        let mut h: MinHeap<i32> = MinHeap::new();
        h.clear();
        assert!(h.is_empty());
    }

    // ── heap invariant after many mixed operations ─────────────────────────────

    #[test]
    fn heap_invariant_after_interleaved_push_pop() {
        let mut h = MinHeap::new();
        let ops = [
            (true, 5u32),
            (true, 2),
            (false, 0),
            (true, 8),
            (true, 1),
            (false, 0),
            (true, 4),
        ];
        let mut last_popped = 0u32;
        for (is_push, val) in ops {
            if is_push {
                h.push(val);
            } else if let Some(v) = h.pop() {
                last_popped = v;
            }
        }
        // drain remainder and verify ascending
        let mut prev = last_popped;
        while let Some(v) = h.pop() {
            assert!(v >= prev);
            prev = v;
        }
    }

    // ── FromIterator / Extend ─────────────────────────────────────────────────

    #[test]
    fn collect_preserves_heap_property() {
        let h: MinHeap<u32> = vec![9, 4, 7, 1, 3].into_iter().collect();
        assert_eq!(h.peek(), Some(&1));
        assert_eq!(h.len(), 5);
    }

    #[test]
    fn collect_from_range() {
        let h: MinHeap<u32> = (1..=100).rev().collect();
        assert_eq!(h.peek(), Some(&1));
    }

    #[test]
    fn extend_maintains_heap_property() {
        let mut h = MinHeap::new();
        h.push(10u32);
        h.extend([5, 3, 8, 1]);
        assert_eq!(h.peek(), Some(&1));
        assert_eq!(h.len(), 5);
    }

    #[test]
    fn extend_empty_is_noop() {
        let mut h = MinHeap::new();
        h.push(1u32);
        h.extend(std::iter::empty::<u32>());
        assert_eq!(h.len(), 1);
    }

    // ── IntoIterator (consuming, sorted) ─────────────────────────────────────

    #[test]
    fn into_iter_yields_sorted_ascending() {
        let h: MinHeap<u32> = vec![4, 2, 5, 1, 3].into_iter().collect();
        let sorted: Vec<u32> = h.into_iter().collect();
        assert_eq!(sorted, [1, 2, 3, 4, 5]);
    }

    #[test]
    fn for_loop_consuming_sorted() {
        let h: MinHeap<u32> = (1..=5).rev().collect();
        let mut out = Vec::new();
        for x in h {
            out.push(x);
        }
        assert_eq!(out, [1, 2, 3, 4, 5]);
    }

    // ── Derived traits ────────────────────────────────────────────────────────

    #[test]
    fn clone_is_independent() {
        let mut original: MinHeap<u32> = (1..=4).collect();
        let mut cloned = original.clone();
        original.pop();
        cloned.push(0);
        assert_eq!(original.len(), 3);
        assert_eq!(cloned.len(), 5);
        assert_eq!(cloned.peek(), Some(&0));
    }

    #[test]
    fn debug_format_non_empty() {
        let mut h: MinHeap<u32> = MinHeap::new();
        h.push(42);
        let s = format!("{:?}", h);
        assert!(s.contains("42"));
    }

    // ── Generic / type coverage ───────────────────────────────────────────────

    #[test]
    fn works_with_strings() {
        let mut h: MinHeap<String> = MinHeap::new();
        h.push("banana".to_string());
        h.push("apple".to_string());
        h.push("cherry".to_string());
        // lexicographic min is "apple"
        assert_eq!(h.pop(), Some("apple".to_string()));
    }

    #[test]
    fn works_with_tuples() {
        // tuples order lexicographically: (priority, value)
        let mut h: MinHeap<(u32, &str)> = MinHeap::new();
        h.push((3, "low"));
        h.push((1, "high"));
        h.push((2, "medium"));
        assert_eq!(h.pop(), Some((1, "high")));
        assert_eq!(h.pop(), Some((2, "medium")));
        assert_eq!(h.pop(), Some((3, "low")));
    }

    // ── Stress test ───────────────────────────────────────────────────────────

    #[test]
    fn large_heap_sorted_output() {
        const N: u32 = 100_000;
        let h: MinHeap<u32> = (0..N).rev().collect();
        let sorted: Vec<u32> = h.into_iter().collect();
        let expected: Vec<u32> = (0..N).collect();
        assert_eq!(sorted, expected);
    }

    #[test]
    fn large_heap_peek_always_minimum() {
        const N: u32 = 10_000;
        let mut h: MinHeap<u32> = MinHeap::with_capacity(N as usize);
        for i in (0..N).rev() {
            h.push(i);
            assert_eq!(h.peek(), Some(&i)); // each push is new minimum
        }
    }
}
