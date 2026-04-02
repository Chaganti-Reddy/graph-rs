use graph_core::NodeId;
use std::collections::HashMap;

/// Reconstructs the path from `start` to `end` using a parent map produced
/// by BFS or DFS.
///
/// Returns `Some(path)` where `path[0] == start` and `path[last] == end`,
/// or `None` if `end` is not reachable from `start` in the parent map.
///
/// # Examples
///
/// ```
/// use graph_core::NodeId;
/// use graph_traversal::reconstruct_path;
/// use std::collections::HashMap;
///
/// let a = NodeId::new(0);
/// let b = NodeId::new(1);
/// let c = NodeId::new(2);
///
/// let mut parent = HashMap::new();
/// parent.insert(b, a);
/// parent.insert(c, b);
///
/// let path = reconstruct_path(&parent, a, c).unwrap();
/// assert_eq!(path, vec![a, b, c]);
/// ```
pub fn reconstruct_path(
    parent: &HashMap<NodeId, NodeId>,
    start: NodeId,
    end: NodeId,
) -> Option<Vec<NodeId>> {
    if start == end {
        return Some(vec![start]);
    }

    let mut path = vec![end];
    let mut current = end;

    loop {
        let prev = *parent.get(&current)?;
        path.push(prev);
        if prev == start {
            break;
        }
        current = prev;
    }

    path.reverse();
    Some(path)
}
