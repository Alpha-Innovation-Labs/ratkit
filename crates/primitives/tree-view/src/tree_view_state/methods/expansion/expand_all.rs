//! TreeViewState::expand_all method.

use std::collections::HashSet;

use crate::tree_node::TreeNode;
use crate::tree_view_state::TreeViewState;

impl TreeViewState {
    /// Expands all nodes in the tree.
    ///
    /// Recursively collects all expandable node paths and adds them
    /// to the expanded set.
    ///
    /// # Arguments
    ///
    /// * `nodes` - The tree nodes to expand.
    ///
    /// # Example
    ///
    /// ```rust
    /// use ratatui_toolkit::tree_view::{TreeNode, TreeViewState};
    ///
    /// let child = TreeNode::new("Child");
    /// let parent = TreeNode::with_children("Parent", vec![child]);
    /// let nodes = vec![parent];
    ///
    /// let mut state = TreeViewState::new();
    /// state.expand_all(&nodes);
    /// assert!(state.is_expanded(&[0]));
    /// ```
    pub fn expand_all<T>(&mut self, nodes: &[TreeNode<T>]) {
        fn collect_paths<T>(
            nodes: &[TreeNode<T>],
            current_path: Vec<usize>,
            expanded: &mut HashSet<Vec<usize>>,
        ) {
            for (idx, node) in nodes.iter().enumerate() {
                let mut path = current_path.clone();
                path.push(idx);

                if node.expandable {
                    expanded.insert(path.clone());
                }

                if !node.children.is_empty() {
                    collect_paths(&node.children, path, expanded);
                }
            }
        }

        collect_paths(nodes, Vec::new(), &mut self.expanded);
    }
}
