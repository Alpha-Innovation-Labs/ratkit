//! Get all visible paths in the tree.

use crate::tree_view::{TreeNode, TreeViewState};

/// Gets all visible paths (flattened tree with expansion state).
///
/// Returns paths for all nodes that are currently visible, respecting
/// the expansion state of parent nodes.
///
/// # Arguments
///
/// * `nodes` - The tree nodes
/// * `state` - The tree view state (for expansion info)
///
/// # Returns
///
/// A vector of paths (each path is a `Vec<usize>`) for all visible nodes.
pub fn get_visible_paths<T>(nodes: &[TreeNode<T>], state: &TreeViewState) -> Vec<Vec<usize>> {
    let mut paths = Vec::new();

    fn traverse<T>(
        nodes: &[TreeNode<T>],
        current_path: Vec<usize>,
        state: &TreeViewState,
        paths: &mut Vec<Vec<usize>>,
    ) {
        for (idx, node) in nodes.iter().enumerate() {
            let mut path = current_path.clone();
            path.push(idx);
            paths.push(path.clone());

            // If expanded, recurse into children
            if state.is_expanded(&path) && !node.children.is_empty() {
                traverse(&node.children, path, state, paths);
            }
        }
    }

    traverse(nodes, Vec::new(), state, &mut paths);
    paths
}
