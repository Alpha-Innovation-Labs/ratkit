//! Get visible paths with filtering support.

use crate::helpers::get_visible_paths;
use crate::tree_node::TreeNode;
use crate::tree_view_state::TreeViewState;

/// Returns visible paths filtered by a matcher function.
///
/// A path is included if:
/// - The node matches the filter (via matcher function), OR
/// - It's an expanded directory that contains a matching descendant
///
/// When no filter is set (filter is None or empty), falls back to
/// the standard `get_visible_paths` for efficiency.
///
/// # Type Parameters
///
/// * `T` - The node data type
/// * `F` - The filter matcher function type
///
/// # Arguments
///
/// * `nodes` - The tree nodes to traverse
/// * `state` - The tree view state (contains filter and expansion state)
/// * `matcher` - A function that takes node data and filter text, returns true if matches
///
/// # Returns
///
/// A vector of paths (each path is a `Vec<usize>`) that are visible and match the filter.
///
/// # Example
///
/// ```ignore
/// let paths = get_visible_paths_filtered(&nodes, &state, |data, filter| {
///     matches_filter(&data.name, filter)
/// });
/// ```
pub fn get_visible_paths_filtered<T, F>(
    nodes: &[TreeNode<T>],
    state: &TreeViewState,
    matcher: F,
) -> Vec<Vec<usize>>
where
    F: Fn(&T, &Option<String>) -> bool,
{
    // If no filter is active, use the standard get_visible_paths for efficiency
    if state.filter.is_none() || state.filter.as_ref().map_or(true, |f| f.is_empty()) {
        return get_visible_paths(nodes, state);
    }

    let mut paths = Vec::new();

    fn traverse<T, F>(
        nodes: &[TreeNode<T>],
        current_path: Vec<usize>,
        state: &TreeViewState,
        matcher: &F,
        paths: &mut Vec<Vec<usize>>,
    ) -> bool
    where
        F: Fn(&T, &Option<String>) -> bool,
    {
        let mut any_match = false;

        for (idx, node) in nodes.iter().enumerate() {
            let mut path = current_path.clone();
            path.push(idx);

            let node_matches = matcher(&node.data, &state.filter);

            // For expanded directories, check if any children match
            let children_match = if !node.children.is_empty() && state.is_expanded(&path) {
                traverse(&node.children, path.clone(), state, matcher, paths)
            } else {
                false
            };

            // Include this node if it matches or has matching children
            if node_matches || children_match {
                paths.push(path);
                any_match = true;
            }
        }

        any_match
    }

    traverse(nodes, Vec::new(), state, &matcher, &mut paths);

    // Sort paths to maintain tree order
    paths.sort();
    paths.dedup();

    paths
}
