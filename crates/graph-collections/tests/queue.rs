#[cfg(test)]
mod tests {
    use graph_collections::Queue;

    // ── Construction ──────────────────────────────────────────────────────────

    #[test]
    fn new_queue_is_empty() {
        let q: Queue<i32> = Queue::new();
        assert!(q.is_empty());
        assert_eq!(q.len(), 0);
    }

    #[test]
    fn default_queue_is_empty() {
        let q: Queue<i32> = Queue::default();
        assert!(q.is_empty());
    }

    #[test]
    fn with_capacity_starts_empty() {
        let q: Queue<i32> = Queue::with_capacity(64);
        assert!(q.is_empty());
        assert_eq!(q.len(), 0);
    }

    // ── Enqueue / Dequeue ─────────────────────────────────────────────────────

    #[test]
    fn enqueue_single_element() {
        let mut q = Queue::new();
        q.enqueue(42u32);
        assert!(!q.is_empty());
        assert_eq!(q.len(), 1);
        assert_eq!(q.front(), Some(&42));
    }

    #[test]
    fn dequeue_empty_returns_none() {
        let mut q: Queue<i32> = Queue::new();
        assert_eq!(q.dequeue(), None);
    }

    #[test]
    fn enqueue_then_dequeue_roundtrip() {
        let mut q = Queue::new();
        q.enqueue(1u32);
        assert_eq!(q.dequeue(), Some(1));
        assert_eq!(q.dequeue(), None);
    }

    #[test]
    fn fifo_ordering_is_preserved() {
        let mut q = Queue::new();
        for i in 0..5u32 {
            q.enqueue(i);
        }
        for expected in 0..5u32 {
            assert_eq!(q.dequeue(), Some(expected));
        }
        assert!(q.is_empty());
    }

    #[test]
    fn interleaved_enqueue_dequeue() {
        let mut q = Queue::new();
        q.enqueue(1);
        q.enqueue(2);
        assert_eq!(q.dequeue(), Some(1));
        q.enqueue(3);
        assert_eq!(q.dequeue(), Some(2));
        assert_eq!(q.dequeue(), Some(3));
        assert_eq!(q.dequeue(), None);
    }

    // ── front / back ──────────────────────────────────────────────────────────

    #[test]
    fn front_empty_is_none() {
        let q: Queue<i32> = Queue::new();
        assert_eq!(q.front(), None);
    }

    #[test]
    fn back_empty_is_none() {
        let q: Queue<i32> = Queue::new();
        assert_eq!(q.back(), None);
    }

    #[test]
    fn front_and_back_single_element() {
        let mut q = Queue::new();
        q.enqueue(7u32);
        assert_eq!(q.front(), Some(&7));
        assert_eq!(q.back(), Some(&7));
    }

    #[test]
    fn front_and_back_multiple_elements() {
        let mut q = Queue::new();
        q.enqueue(1u32);
        q.enqueue(2);
        q.enqueue(3);
        assert_eq!(q.front(), Some(&1));
        assert_eq!(q.back(), Some(&3));
    }

    #[test]
    fn front_does_not_consume_element() {
        let mut q = Queue::new();
        q.enqueue(99u32);
        let _ = q.front();
        assert_eq!(q.len(), 1);
        assert_eq!(q.dequeue(), Some(99));
    }

    // ── len / is_empty ────────────────────────────────────────────────────────

    #[test]
    fn len_tracks_enqueue_and_dequeue() {
        let mut q = Queue::new();
        assert_eq!(q.len(), 0);
        q.enqueue(1);
        assert_eq!(q.len(), 1);
        q.enqueue(2);
        assert_eq!(q.len(), 2);
        q.dequeue();
        assert_eq!(q.len(), 1);
        q.dequeue();
        assert_eq!(q.len(), 0);
    }

    // ── clear ─────────────────────────────────────────────────────────────────

    #[test]
    fn clear_empties_queue() {
        let mut q: Queue<i32> = (1..=10).collect();
        assert_eq!(q.len(), 10);
        q.clear();
        assert!(q.is_empty());
        assert_eq!(q.len(), 0);
    }

    #[test]
    fn clear_on_empty_queue_is_a_noop() {
        let mut q: Queue<i32> = Queue::new();
        q.clear(); // should not panic
        assert!(q.is_empty());
    }

    // ── iter ──────────────────────────────────────────────────────────────────

    #[test]
    fn iter_yields_elements_front_to_back() {
        let q: Queue<u32> = (1..=5).collect();
        let values: Vec<u32> = q.iter().copied().collect();
        assert_eq!(values, [1, 2, 3, 4, 5]);
    }

    #[test]
    fn iter_does_not_consume_queue() {
        let q: Queue<u32> = (1..=3).collect();
        let _ = q.iter().count();
        assert_eq!(q.len(), 3);
    }

    #[test]
    fn iter_on_empty_queue_yields_nothing() {
        let q: Queue<i32> = Queue::new();
        assert_eq!(q.iter().count(), 0);
    }

    // ── IntoIterator (consuming) ──────────────────────────────────────────────

    #[test]
    fn into_iter_consuming_yields_front_to_back() {
        let q: Queue<u32> = (1..=3).collect();
        let values: Vec<u32> = q.into_iter().collect();
        assert_eq!(values, [1, 2, 3]);
    }

    #[test]
    fn for_loop_consuming() {
        let q: Queue<u32> = (0..4).collect();
        let mut out = Vec::new();
        for x in q {
            out.push(x);
        }
        assert_eq!(out, [0, 1, 2, 3]);
    }

    // ── IntoIterator (borrowing) ──────────────────────────────────────────────

    #[test]
    fn into_iter_borrow_yields_front_to_back() {
        let q: Queue<u32> = (1..=3).collect();
        let values: Vec<u32> = (&q).into_iter().copied().collect();
        assert_eq!(values, [1, 2, 3]);
        assert_eq!(q.len(), 3); // still alive
    }

    #[test]
    fn for_loop_borrowing() {
        let q: Queue<u32> = (0..4).collect();
        let mut sum = 0u32;
        for x in &q {
            sum += x;
        }
        assert_eq!(sum, 6);
        assert_eq!(q.len(), 4); // queue untouched
    }

    // ── Iterator adapters work ────────────────────────────────────────────────

    #[test]
    fn map_and_filter_via_iter() {
        let q: Queue<u32> = (1..=6).collect();
        let evens_doubled: Vec<u32> = q.iter().filter(|&&x| x % 2 == 0).map(|&x| x * 2).collect();
        assert_eq!(evens_doubled, [4, 8, 12]);
    }

    #[test]
    fn sum_via_into_iter() {
        let q: Queue<u32> = (1..=100).collect();
        let total: u32 = q.into_iter().sum();
        assert_eq!(total, 5050);
    }

    // ── FromIterator / Extend ─────────────────────────────────────────────────

    #[test]
    fn collect_into_queue_preserves_order() {
        let q: Queue<u32> = vec![10, 20, 30].into_iter().collect();
        assert_eq!(q.front(), Some(&10));
        assert_eq!(q.back(), Some(&30));
        assert_eq!(q.len(), 3);
    }

    #[test]
    fn extend_appends_to_back() {
        let mut q: Queue<u32> = Queue::new();
        q.enqueue(1);
        q.extend([2, 3, 4]);
        assert_eq!(q.len(), 4);
        assert_eq!(q.front(), Some(&1));
        assert_eq!(q.back(), Some(&4));
    }

    #[test]
    fn extend_empty_iterator_is_a_noop() {
        let mut q: Queue<u32> = Queue::new();
        q.enqueue(1);
        q.extend(std::iter::empty());
        assert_eq!(q.len(), 1);
    }

    // ── Derived traits ────────────────────────────────────────────────────────

    #[test]
    fn clone_is_independent() {
        let mut original: Queue<u32> = (1..=3).collect();
        let mut cloned = original.clone();

        original.dequeue();
        cloned.enqueue(99);

        // original is shorter; clone has extra element
        assert_eq!(original.len(), 2);
        assert_eq!(cloned.len(), 4);
    }

    #[test]
    fn equality_same_order() {
        let a: Queue<u32> = (1..=3).collect();
        let b: Queue<u32> = (1..=3).collect();
        assert_eq!(a, b);
    }

    #[test]
    fn equality_different_order() {
        let a: Queue<u32> = vec![1, 2, 3].into_iter().collect();
        let b: Queue<u32> = vec![3, 2, 1].into_iter().collect();
        assert_ne!(a, b);
    }

    #[test]
    fn debug_format_is_non_empty() {
        let mut q: Queue<u32> = Queue::new();
        q.enqueue(1);
        let s = format!("{:?}", q);
        assert!(s.contains('1'));
    }

    // ── Generic / type coverage ───────────────────────────────────────────────

    #[test]
    fn works_with_strings() {
        let mut q: Queue<String> = Queue::new();
        q.enqueue("hello".to_string());
        q.enqueue("world".to_string());
        assert_eq!(q.dequeue(), Some("hello".to_string()));
    }

    #[test]
    fn works_with_option_type() {
        let mut q: Queue<Option<i32>> = Queue::new();
        q.enqueue(Some(1));
        q.enqueue(None);
        q.enqueue(Some(3));
        assert_eq!(q.dequeue(), Some(Some(1)));
        assert_eq!(q.dequeue(), Some(None));
    }

    #[test]
    fn large_queue_stress() {
        const N: u32 = 100_000;
        let mut q: Queue<u32> = Queue::with_capacity(N as usize);
        for i in 0..N {
            q.enqueue(i);
        }
        assert_eq!(q.len(), N as usize);
        for i in 0..N {
            assert_eq!(q.dequeue(), Some(i));
        }
        assert!(q.is_empty());
    }
}
