//! TreeNavigator::get_node_at_path helper method.

use crate::primitives::tree_view::tree_navigator::TreeNavigator;
use crate::primitives::tree_view::tree_node::TreeNode;

impl TreeNavigator {
    /// Gets a node at a specific path.
    ///
    /// # Arguments
    ///
    /// * `nodes` - The tree nodes.
    /// * `path` - The path to the node (indices from root).
    ///
    /// # Returns
    ///
    /// A reference to the node at the path, or `None` if not found.
    pub(crate) fn get_node_at_path<'a, T>(
        &self,
        nodes: &'a [TreeNode<T>],
        path: &[usize],
    ) -> Option<&'a TreeNode<T>> {
        if path.is_empty() {
            return None;
        }

        let mut current_nodes = nodes;
        let mut node = None;

        for &idx in path {
            node = current_nodes.get(idx);
            if let Some(n) = node {
                current_nodes = &n.children;
            } else {
                return None;
            }
        }

        node
    }
}
