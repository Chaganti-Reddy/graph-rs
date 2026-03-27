#[cfg(test)]
mod tests {
    use graph_collections::Deque;

    // ── Construction ──────────────────────────────────────────────────────────

    #[test]
    fn new_deque_is_empty() {
        let d: Deque<i32> = Deque::new();
        assert!(d.is_empty());
        assert_eq!(d.len(), 0);
    }

    #[test]
    fn default_deque_is_empty() {
        let d: Deque<i32> = Deque::default();
        assert!(d.is_empty());
    }

    #[test]
    fn with_capacity_starts_empty() {
        let d: Deque<i32> = Deque::with_capacity(64);
        assert!(d.is_empty());
        assert_eq!(d.len(), 0);
    }

    // ── push_back ─────────────────────────────────────────────────────────────

    #[test]
    fn push_back_single() {
        let mut d = Deque::new();
        d.push_back(1u32);
        assert_eq!(d.len(), 1);
        assert_eq!(d.front(), Some(&1));
        assert_eq!(d.back(), Some(&1));
    }

    #[test]
    fn push_back_multiple_preserves_order() {
        let mut d = Deque::new();
        d.push_back(1u32);
        d.push_back(2);
        d.push_back(3);
        assert_eq!(d.front(), Some(&1));
        assert_eq!(d.back(), Some(&3));
        assert_eq!(d.len(), 3);
    }

    // ── push_front ────────────────────────────────────────────────────────────

    #[test]
    fn push_front_single() {
        let mut d = Deque::new();
        d.push_front(1u32);
        assert_eq!(d.len(), 1);
        assert_eq!(d.front(), Some(&1));
        assert_eq!(d.back(), Some(&1));
    }

    #[test]
    fn push_front_multiple_reverses_order() {
        let mut d = Deque::new();
        d.push_front(3u32);
        d.push_front(2);
        d.push_front(1);
        // pushed 3, then 2 before it, then 1 before that → [1, 2, 3]
        assert_eq!(d.front(), Some(&1));
        assert_eq!(d.back(), Some(&3));
    }

    // ── mixed push_front / push_back ─────────────────────────────────────────

    #[test]
    fn interleaved_push_front_and_back() {
        let mut d = Deque::new();
        d.push_back(2u32);
        d.push_front(1);
        d.push_back(3);
        d.push_front(0);
        // [0, 1, 2, 3]
        let values: Vec<u32> = d.iter().copied().collect();
        assert_eq!(values, [0, 1, 2, 3]);
    }

    // ── pop_front ─────────────────────────────────────────────────────────────

    #[test]
    fn pop_front_empty_returns_none() {
        let mut d: Deque<i32> = Deque::new();
        assert_eq!(d.pop_front(), None);
    }

    #[test]
    fn pop_front_removes_front() {
        let mut d: Deque<u32> = (1..=3).collect();
        assert_eq!(d.pop_front(), Some(1));
        assert_eq!(d.pop_front(), Some(2));
        assert_eq!(d.pop_front(), Some(3));
        assert_eq!(d.pop_front(), None);
    }

    #[test]
    fn pop_front_decrements_len() {
        let mut d: Deque<u32> = (1..=3).collect();
        d.pop_front();
        assert_eq!(d.len(), 2);
    }

    // ── pop_back ──────────────────────────────────────────────────────────────

    #[test]
    fn pop_back_empty_returns_none() {
        let mut d: Deque<i32> = Deque::new();
        assert_eq!(d.pop_back(), None);
    }

    #[test]
    fn pop_back_removes_back() {
        let mut d: Deque<u32> = (1..=3).collect();
        assert_eq!(d.pop_back(), Some(3));
        assert_eq!(d.pop_back(), Some(2));
        assert_eq!(d.pop_back(), Some(1));
        assert_eq!(d.pop_back(), None);
    }

    #[test]
    fn pop_back_decrements_len() {
        let mut d: Deque<u32> = (1..=3).collect();
        d.pop_back();
        assert_eq!(d.len(), 2);
    }

    // ── double-ended symmetry ─────────────────────────────────────────────────

    #[test]
    fn used_as_stack_lifo_via_back() {
        // push_back + pop_back = LIFO stack
        let mut d = Deque::new();
        d.push_back(1u32);
        d.push_back(2);
        d.push_back(3);
        assert_eq!(d.pop_back(), Some(3));
        assert_eq!(d.pop_back(), Some(2));
        assert_eq!(d.pop_back(), Some(1));
    }

    #[test]
    fn used_as_queue_fifo_via_front() {
        // push_back + pop_front = FIFO queue
        let mut d = Deque::new();
        d.push_back(1u32);
        d.push_back(2);
        d.push_back(3);
        assert_eq!(d.pop_front(), Some(1));
        assert_eq!(d.pop_front(), Some(2));
        assert_eq!(d.pop_front(), Some(3));
    }

    #[test]
    fn drain_from_both_ends_alternating() {
        let mut d: Deque<u32> = (1..=6).collect();
        // [1, 2, 3, 4, 5, 6]
        assert_eq!(d.pop_front(), Some(1));
        assert_eq!(d.pop_back(), Some(6));
        assert_eq!(d.pop_front(), Some(2));
        assert_eq!(d.pop_back(), Some(5));
        assert_eq!(d.pop_front(), Some(3));
        assert_eq!(d.pop_back(), Some(4));
        assert!(d.is_empty());
    }

    // ── front / back peek ─────────────────────────────────────────────────────

    #[test]
    fn front_empty_is_none() {
        let d: Deque<i32> = Deque::new();
        assert_eq!(d.front(), None);
    }

    #[test]
    fn back_empty_is_none() {
        let d: Deque<i32> = Deque::new();
        assert_eq!(d.back(), None);
    }

    #[test]
    fn front_and_back_single_element() {
        let mut d = Deque::new();
        d.push_back(42u32);
        assert_eq!(d.front(), Some(&42));
        assert_eq!(d.back(), Some(&42));
    }

    #[test]
    fn front_does_not_consume() {
        let d: Deque<u32> = (1..=3).collect();
        let _ = d.front();
        assert_eq!(d.len(), 3);
    }

    #[test]
    fn back_does_not_consume() {
        let d: Deque<u32> = (1..=3).collect();
        let _ = d.back();
        assert_eq!(d.len(), 3);
    }

    // ── front_mut / back_mut ──────────────────────────────────────────────────

    #[test]
    fn front_mut_empty_is_none() {
        let mut d: Deque<i32> = Deque::new();
        assert_eq!(d.front_mut(), None);
    }

    #[test]
    fn back_mut_empty_is_none() {
        let mut d: Deque<i32> = Deque::new();
        assert_eq!(d.back_mut(), None);
    }

    #[test]
    fn front_mut_modifies_front() {
        let mut d: Deque<u32> = (1..=3).collect();
        *d.front_mut().unwrap() = 99;
        assert_eq!(d.front(), Some(&99));
        assert_eq!(d.len(), 3); // no element added/removed
    }

    #[test]
    fn back_mut_modifies_back() {
        let mut d: Deque<u32> = (1..=3).collect();
        *d.back_mut().unwrap() = 99;
        assert_eq!(d.back(), Some(&99));
        assert_eq!(d.len(), 3);
    }

    // ── get ───────────────────────────────────────────────────────────────────

    #[test]
    fn get_in_bounds() {
        let d: Deque<u32> = (10..=14).collect(); // [10, 11, 12, 13, 14]
        assert_eq!(d.get(0), Some(&10));
        assert_eq!(d.get(2), Some(&12));
        assert_eq!(d.get(4), Some(&14));
    }

    #[test]
    fn get_out_of_bounds() {
        let d: Deque<u32> = (10..=14).collect();
        assert_eq!(d.get(5), None);
        assert_eq!(d.get(100), None);
    }

    #[test]
    fn get_on_empty_deque() {
        let d: Deque<i32> = Deque::new();
        assert_eq!(d.get(0), None);
    }

    // ── len / is_empty ────────────────────────────────────────────────────────

    #[test]
    fn len_tracks_all_operations() {
        let mut d = Deque::new();
        assert_eq!(d.len(), 0);
        d.push_back(1u32);
        assert_eq!(d.len(), 1);
        d.push_front(0);
        assert_eq!(d.len(), 2);
        d.pop_back();
        assert_eq!(d.len(), 1);
        d.pop_front();
        assert_eq!(d.len(), 0);
    }

    // ── clear ─────────────────────────────────────────────────────────────────

    #[test]
    fn clear_empties_deque() {
        let mut d: Deque<i32> = (1..=10).collect();
        assert_eq!(d.len(), 10);
        d.clear();
        assert!(d.is_empty());
        assert_eq!(d.len(), 0);
    }

    #[test]
    fn clear_on_empty_is_noop() {
        let mut d: Deque<i32> = Deque::new();
        d.clear();
        assert!(d.is_empty());
    }

    // ── rotate_left ───────────────────────────────────────────────────────────

    #[test]
    fn rotate_left_by_zero_is_noop() {
        let mut d: Deque<u32> = (1..=5).collect();
        d.rotate_left(0);
        let values: Vec<u32> = d.iter().copied().collect();
        assert_eq!(values, [1, 2, 3, 4, 5]);
    }

    #[test]
    fn rotate_left_by_one() {
        let mut d: Deque<u32> = (1..=5).collect();
        d.rotate_left(1);
        // [1,2,3,4,5] → [2,3,4,5,1]
        let values: Vec<u32> = d.iter().copied().collect();
        assert_eq!(values, [2, 3, 4, 5, 1]);
    }

    #[test]
    fn rotate_left_by_n() {
        let mut d: Deque<u32> = (1..=5).collect();
        d.rotate_left(2);
        // [1,2,3,4,5] → [3,4,5,1,2]
        let values: Vec<u32> = d.iter().copied().collect();
        assert_eq!(values, [3, 4, 5, 1, 2]);
    }

    #[test]
    fn rotate_left_by_len_is_noop() {
        let mut d: Deque<u32> = (1..=5).collect();
        d.rotate_left(5);
        let values: Vec<u32> = d.iter().copied().collect();
        assert_eq!(values, [1, 2, 3, 4, 5]);
    }

    // ── rotate_right ──────────────────────────────────────────────────────────

    #[test]
    fn rotate_right_by_zero_is_noop() {
        let mut d: Deque<u32> = (1..=5).collect();
        d.rotate_right(0);
        let values: Vec<u32> = d.iter().copied().collect();
        assert_eq!(values, [1, 2, 3, 4, 5]);
    }

    #[test]
    fn rotate_right_by_one() {
        let mut d: Deque<u32> = (1..=5).collect();
        d.rotate_right(1);
        // [1,2,3,4,5] → [5,1,2,3,4]
        let values: Vec<u32> = d.iter().copied().collect();
        assert_eq!(values, [5, 1, 2, 3, 4]);
    }

    #[test]
    fn rotate_right_by_n() {
        let mut d: Deque<u32> = (1..=5).collect();
        d.rotate_right(2);
        // [1,2,3,4,5] → [4,5,1,2,3]
        let values: Vec<u32> = d.iter().copied().collect();
        assert_eq!(values, [4, 5, 1, 2, 3]);
    }

    #[test]
    fn rotate_left_then_right_is_identity() {
        let mut d: Deque<u32> = (1..=5).collect();
        d.rotate_left(3);
        d.rotate_right(3);
        let values: Vec<u32> = d.iter().copied().collect();
        assert_eq!(values, [1, 2, 3, 4, 5]);
    }

    // ── iter (front-to-back) ──────────────────────────────────────────────────

    #[test]
    fn iter_yields_front_to_back() {
        let d: Deque<u32> = (1..=5).collect();
        let values: Vec<u32> = d.iter().copied().collect();
        assert_eq!(values, [1, 2, 3, 4, 5]);
    }

    #[test]
    fn iter_does_not_consume() {
        let d: Deque<u32> = (1..=3).collect();
        let _ = d.iter().count();
        assert_eq!(d.len(), 3);
    }

    #[test]
    fn iter_on_empty_yields_nothing() {
        let d: Deque<i32> = Deque::new();
        assert_eq!(d.iter().count(), 0);
    }

    // ── iter_back (back-to-front) ─────────────────────────────────────────────

    #[test]
    fn iter_back_yields_back_to_front() {
        let d: Deque<u32> = (1..=5).collect();
        let values: Vec<u32> = d.iter_back().copied().collect();
        assert_eq!(values, [5, 4, 3, 2, 1]);
    }

    #[test]
    fn iter_back_does_not_consume() {
        let d: Deque<u32> = (1..=3).collect();
        let _ = d.iter_back().count();
        assert_eq!(d.len(), 3);
    }

    #[test]
    fn iter_and_iter_back_are_reverses_of_each_other() {
        let d: Deque<u32> = (1..=6).collect();
        let fwd: Vec<u32> = d.iter().copied().collect();
        let mut bwd: Vec<u32> = d.iter_back().copied().collect();
        bwd.reverse();
        assert_eq!(fwd, bwd);
    }

    // ── IntoIterator (consuming) ──────────────────────────────────────────────

    #[test]
    fn into_iter_consuming_front_to_back() {
        let d: Deque<u32> = (1..=3).collect();
        let values: Vec<u32> = d.into_iter().collect();
        assert_eq!(values, [1, 2, 3]);
    }

    #[test]
    fn for_loop_consuming() {
        let d: Deque<u32> = (0..4).collect();
        let mut out = Vec::new();
        for x in d {
            out.push(x);
        }
        assert_eq!(out, [0, 1, 2, 3]);
    }

    // ── IntoIterator (borrowing) ──────────────────────────────────────────────

    #[test]
    fn into_iter_borrow_front_to_back() {
        let d: Deque<u32> = (1..=3).collect();
        let values: Vec<u32> = (&d).into_iter().copied().collect();
        assert_eq!(values, [1, 2, 3]);
        assert_eq!(d.len(), 3); // still alive
    }

    #[test]
    fn for_loop_borrowing() {
        let d: Deque<u32> = (1..=4).collect();
        let mut sum = 0u32;
        for x in &d {
            sum += x;
        }
        assert_eq!(sum, 10);
        assert_eq!(d.len(), 4);
    }

    // ── Iterator adapters ─────────────────────────────────────────────────────

    #[test]
    fn map_filter_via_iter() {
        let d: Deque<u32> = (1..=6).collect();
        let result: Vec<u32> = d.iter().filter(|&&x| x % 2 == 0).map(|&x| x * 10).collect();
        assert_eq!(result, [20, 40, 60]);
    }

    #[test]
    fn sum_via_into_iter() {
        let d: Deque<u32> = (1..=100).collect();
        let total: u32 = d.into_iter().sum();
        assert_eq!(total, 5050);
    }

    #[test]
    fn rev_via_iter_back_adapter() {
        let d: Deque<u32> = (1..=5).collect();
        // iter_back gives DoubleEndedIterator, so .rev() on it gives forward again
        let values: Vec<u32> = d.iter_back().rev().copied().collect();
        assert_eq!(values, [1, 2, 3, 4, 5]);
    }

    // ── FromIterator / Extend ─────────────────────────────────────────────────

    #[test]
    fn collect_preserves_order() {
        let d: Deque<u32> = vec![10, 20, 30].into_iter().collect();
        assert_eq!(d.front(), Some(&10));
        assert_eq!(d.back(), Some(&30));
        assert_eq!(d.len(), 3);
    }

    #[test]
    fn extend_appends_to_back() {
        let mut d: Deque<u32> = Deque::new();
        d.push_back(1);
        d.extend([2, 3, 4]);
        assert_eq!(d.len(), 4);
        assert_eq!(d.front(), Some(&1));
        assert_eq!(d.back(), Some(&4));
    }

    #[test]
    fn extend_empty_is_noop() {
        let mut d: Deque<u32> = Deque::new();
        d.push_back(1);
        d.extend(std::iter::empty::<u32>());
        assert_eq!(d.len(), 1);
    }

    // ── Derived traits ────────────────────────────────────────────────────────

    #[test]
    fn clone_is_independent() {
        let mut original: Deque<u32> = (1..=4).collect();
        let mut cloned = original.clone();

        original.pop_front();
        cloned.push_back(99);

        assert_eq!(original.len(), 3);
        assert_eq!(cloned.len(), 5);
    }

    #[test]
    fn equality_same_order() {
        let a: Deque<u32> = (1..=4).collect();
        let b: Deque<u32> = (1..=4).collect();
        assert_eq!(a, b);
    }

    #[test]
    fn equality_different_order() {
        let a: Deque<u32> = vec![1, 2, 3].into_iter().collect();
        let b: Deque<u32> = vec![3, 2, 1].into_iter().collect();
        assert_ne!(a, b);
    }

    #[test]
    fn equality_different_lengths() {
        let a: Deque<u32> = (1..=3).collect();
        let b: Deque<u32> = (1..=4).collect();
        assert_ne!(a, b);
    }

    #[test]
    fn debug_format_non_empty() {
        let mut d: Deque<u32> = Deque::new();
        d.push_back(42);
        let s = format!("{:?}", d);
        assert!(s.contains("42"));
    }

    // ── Generic / type coverage ───────────────────────────────────────────────

    #[test]
    fn works_with_strings() {
        let mut d: Deque<String> = Deque::new();
        d.push_back("hello".to_string());
        d.push_front("world".to_string());
        // ["world", "hello"]
        assert_eq!(d.pop_front(), Some("world".to_string()));
        assert_eq!(d.pop_back(), Some("hello".to_string()));
    }

    #[test]
    fn works_with_option_type() {
        let mut d: Deque<Option<i32>> = Deque::new();
        d.push_back(Some(1));
        d.push_back(None);
        d.push_front(Some(-1));
        assert_eq!(d.pop_front(), Some(Some(-1)));
        assert_eq!(d.pop_back(), Some(None));
    }

    // ── Palindrome check — classic deque algorithm ────────────────────────────

    #[test]
    fn palindrome_check_even_length() {
        let word = "racecar";
        let mut d: Deque<char> = word.chars().collect();
        let mut is_palindrome = true;
        while d.len() > 1 {
            if d.pop_front() != d.pop_back() {
                is_palindrome = false;
                break;
            }
        }
        assert!(is_palindrome);
    }

    #[test]
    fn palindrome_check_not_palindrome() {
        let word = "hello";
        let mut d: Deque<char> = word.chars().collect();
        let mut is_palindrome = true;
        while d.len() > 1 {
            if d.pop_front() != d.pop_back() {
                is_palindrome = false;
                break;
            }
        }
        assert!(!is_palindrome);
    }

    // ── Stress test ───────────────────────────────────────────────────────────

    #[test]
    fn large_deque_push_back_pop_front() {
        const N: u32 = 100_000;
        let mut d: Deque<u32> = Deque::with_capacity(N as usize);
        for i in 0..N {
            d.push_back(i);
        }
        assert_eq!(d.len(), N as usize);
        for i in 0..N {
            assert_eq!(d.pop_front(), Some(i));
        }
        assert!(d.is_empty());
    }

    #[test]
    fn large_deque_push_front_pop_back() {
        const N: u32 = 100_000;
        let mut d: Deque<u32> = Deque::with_capacity(N as usize);
        for i in 0..N {
            d.push_front(i);
        }
        // pushed 0,1,2,...N-1 each to the front → front is N-1, back is 0
        assert_eq!(d.front(), Some(&(N - 1)));
        assert_eq!(d.back(), Some(&0));
        for i in 0..N {
            assert_eq!(d.pop_back(), Some(i));
        }
        assert!(d.is_empty());
    }
}
