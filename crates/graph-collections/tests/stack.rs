use graph_collections::Stack;

// ── Construction ─────────────────────────────────────────────────────────────

#[test]
fn new_is_empty() {
    let s: Stack<i32> = Stack::new();
    assert!(s.is_empty());
    assert_eq!(s.len(), 0);
}

#[test]
fn default_is_empty() {
    let s: Stack<i32> = Stack::default();
    assert!(s.is_empty());
    assert_eq!(s.len(), 0);
}

#[test]
fn from_vec_empty() {
    let s: Stack<i32> = Stack::from(vec![]);
    assert!(s.is_empty());
}

#[test]
fn from_vec_preserves_lifo_order() {
    let mut s = Stack::from(vec![1, 2, 3]);
    // last element of vec becomes top of stack
    assert_eq!(s.pop(), Some(3));
    assert_eq!(s.pop(), Some(2));
    assert_eq!(s.pop(), Some(1));
    assert_eq!(s.pop(), None);
}

// ── Push ─────────────────────────────────────────────────────────────────────

#[test]
fn push_single_element() {
    let mut s = Stack::new();
    s.push(1);
    assert!(!s.is_empty());
    assert_eq!(s.len(), 1);
}

#[test]
fn push_increases_len() {
    let mut s = Stack::new();
    s.push(1);
    s.push(2);
    s.push(3);
    assert_eq!(s.len(), 3);
}

#[test]
fn push_many_elements() {
    let mut s = Stack::new();
    for i in 0..1000 {
        s.push(i);
    }
    assert_eq!(s.len(), 1000);
}

// ── Pop ──────────────────────────────────────────────────────────────────────

#[test]
fn pop_empty_returns_none() {
    let mut s: Stack<i32> = Stack::new();
    assert_eq!(s.pop(), None);
}

#[test]
fn pop_returns_lifo_order() {
    let mut s = Stack::new();
    s.push(1);
    s.push(2);
    s.push(3);
    assert_eq!(s.pop(), Some(3));
    assert_eq!(s.pop(), Some(2));
    assert_eq!(s.pop(), Some(1));
}

#[test]
fn pop_decreases_len() {
    let mut s = Stack::new();
    s.push(1);
    s.push(2);
    let _ = s.pop();
    assert_eq!(s.len(), 1);
}

#[test]
fn pop_until_empty() {
    let mut s = Stack::new();
    s.push(1);
    s.push(2);
    let _ = s.pop();
    let _ = s.pop();
    assert!(s.is_empty());
    assert_eq!(s.pop(), None); // extra pop on empty is safe
}

// ── Peek ─────────────────────────────────────────────────────────────────────

#[test]
fn peek_empty_returns_none() {
    let s: Stack<i32> = Stack::new();
    assert_eq!(s.peek(), None);
}

#[test]
fn peek_returns_top() {
    let mut s = Stack::new();
    s.push(1);
    s.push(2);
    assert_eq!(s.peek(), Some(&2));
}

#[test]
fn peek_does_not_remove() {
    let mut s = Stack::new();
    s.push(42);
    assert_eq!(s.peek(), Some(&42));
    assert_eq!(s.peek(), Some(&42)); // still there
    assert_eq!(s.len(), 1);
}

#[test]
fn peek_reflects_latest_push() {
    let mut s = Stack::new();
    s.push(1);
    assert_eq!(s.peek(), Some(&1));
    s.push(2);
    assert_eq!(s.peek(), Some(&2)); // updated after new push
}

// ── is_empty / len ───────────────────────────────────────────────────────────

#[test]
fn is_empty_true_on_new() {
    let s: Stack<i32> = Stack::new();
    assert!(s.is_empty());
}

#[test]
fn is_empty_false_after_push() {
    let mut s = Stack::new();
    s.push(1);
    assert!(!s.is_empty());
}

#[test]
fn is_empty_true_after_full_drain() {
    let mut s = Stack::new();
    s.push(1);
    let _ = s.pop();
    assert!(s.is_empty());
}

// ── Clone / PartialEq ────────────────────────────────────────────────────────

#[test]
fn clone_is_independent() {
    let mut original = Stack::new();
    original.push(1);
    original.push(2);

    let mut cloned = original.clone();
    cloned.push(3);

    // original is unaffected
    assert_eq!(original.len(), 2);
    assert_eq!(cloned.len(), 3);
}

#[test]
fn equality_same_elements() {
    let mut a = Stack::new();
    a.push(1);
    a.push(2);

    let mut b = Stack::new();
    b.push(1);
    b.push(2);

    assert_eq!(a, b);
}

#[test]
fn inequality_different_elements() {
    let mut a = Stack::new();
    a.push(1);

    let mut b = Stack::new();
    b.push(2);

    assert_ne!(a, b);
}

#[test]
fn inequality_different_lengths() {
    let mut a = Stack::new();
    a.push(1);
    a.push(2);

    let mut b = Stack::new();
    b.push(1);

    assert_ne!(a, b);
}

// ── Edge Cases ───────────────────────────────────────────────────────────────

#[test]
fn push_pop_interleaved() {
    let mut s = Stack::new();
    s.push(1);
    s.push(2);
    assert_eq!(s.pop(), Some(2));
    s.push(3);
    assert_eq!(s.pop(), Some(3));
    assert_eq!(s.pop(), Some(1));
    assert_eq!(s.pop(), None);
}

#[test]
fn works_with_strings() {
    let mut s = Stack::new();
    s.push("hello");
    s.push("world");
    assert_eq!(s.pop(), Some("world"));
    assert_eq!(s.pop(), Some("hello"));
}

#[test]
fn works_with_option_type() {
    let mut s: Stack<Option<i32>> = Stack::new();
    s.push(Some(1));
    s.push(None);
    assert_eq!(s.pop(), Some(None));
    assert_eq!(s.pop(), Some(Some(1)));
}
