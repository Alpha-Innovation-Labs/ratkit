//! Navigation methods for DiffFileTree.
//!
//! These methods delegate to TreeNavigator for centralized keyboard handling.

use crate::primitives::tree_view::{get_visible_paths_filtered, matches_filter, TreeNavigator};
use crate::widgets::code_diff::diff_file_tree::{DiffFileEntry, DiffFileTree};

impl DiffFileTree {
    /// Returns the filter matcher function for DiffFileEntry nodes.
    fn filter_matcher() -> impl Fn(&DiffFileEntry, &Option<String>) -> bool + Copy {
        |entry: &DiffFileEntry, filter: &Option<String>| matches_filter(&entry.name, filter)
    }

    /// Selects the next visible item in the tree.
    ///
    /// Respects the current filter (if any).
    pub fn select_next(&mut self) {
        let navigator = TreeNavigator::new();
        navigator.select_next_filtered(&self.nodes, &mut self.state, Self::filter_matcher());
    }

    /// Selects the previous visible item in the tree.
    ///
    /// Respects the current filter (if any).
    pub fn select_prev(&mut self) {
        let navigator = TreeNavigator::new();
        navigator.select_previous_filtered(&self.nodes, &mut self.state, Self::filter_matcher());
    }

    /// Toggles the expansion state of the currently selected node.
    ///
    /// Only has an effect if the selected node is a directory.
    pub fn toggle_expand(&mut self) {
        let navigator = TreeNavigator::new();
        navigator.toggle_selected(&self.nodes, &mut self.state);
    }

    /// Expands the currently selected directory.
    ///
    /// Only has an effect if the selected node is a directory.
    pub fn expand_selected(&mut self) {
        let navigator = TreeNavigator::new();
        navigator.expand_selected(&self.nodes, &mut self.state);
    }

    /// Collapses the currently selected directory if it's expanded.
    ///
    /// # Returns
    ///
    /// `true` if a directory was collapsed, `false` otherwise (not expanded
    /// or not a directory).
    pub fn collapse_selected(&mut self) -> bool {
        if let Some(path) = self.state.selected_path.clone() {
            if self.state.is_expanded(&path) {
                // Check if it's a directory
                if let Some(node) = self.get_node_at_path(&path) {
                    if node.data.is_dir {
                        self.state.collapse(path);
                        return true;
                    }
                }
            }
        }
        false
    }

    /// Moves selection to the parent directory of the currently selected node.
    ///
    /// If the selected node is already at the root level, this does nothing.
    pub fn go_to_parent(&mut self) {
        if let Some(path) = &self.state.selected_path {
            if path.len() > 1 {
                // Move to parent by removing the last index
                let parent = path[..path.len() - 1].to_vec();
                self.state.select(parent);
            }
        }
    }

    /// Goes to the first visible item in the tree.
    ///
    /// Respects the current filter (if any).
    pub fn goto_top(&mut self) {
        let navigator = TreeNavigator::new();
        navigator.goto_top_filtered(&self.nodes, &mut self.state, Self::filter_matcher());
    }

    /// Goes to the last visible item in the tree.
    ///
    /// Respects the current filter (if any).
    pub fn goto_bottom(&mut self) {
        let navigator = TreeNavigator::new();
        navigator.goto_bottom_filtered(&self.nodes, &mut self.state, Self::filter_matcher());
    }

    /// Returns whether the selected item is a directory.
    ///
    /// # Returns
    ///
    /// `Some(true)` if selected item is a directory, `Some(false)` if it's a file,
    /// `None` if no item is selected.
    #[must_use]
    pub fn selected_is_dir(&self) -> Option<bool> {
        let path = self.state.selected_path.as_ref()?;
        let node = self.get_node_at_path(path)?;
        Some(node.data.is_dir)
    }

    /// Returns the total number of visible items.
    ///
    /// Respects the current filter (if any).
    #[must_use]
    pub fn visible_count(&self) -> usize {
        get_visible_paths_filtered(&self.nodes, &self.state, Self::filter_matcher()).len()
    }

    /// Sets the selection by visible index.
    ///
    /// Respects the current filter (if any).
    ///
    /// # Arguments
    ///
    /// * `index` - The 0-based index in the visible items list
    pub fn set_selected_index(&mut self, index: usize) {
        let visible_paths =
            get_visible_paths_filtered(&self.nodes, &self.state, Self::filter_matcher());
        if let Some(path) = visible_paths.get(index) {
            self.state.select(path.clone());
        }
    }

    /// Helper to get a node at a specific path.
    pub(crate) fn get_node_at_path(
        &self,
        path: &[usize],
    ) -> Option<
        &crate::primitives::tree_view::TreeNode<
            crate::widgets::code_diff::diff_file_tree::DiffFileEntry,
        >,
    > {
        if path.is_empty() {
            return None;
        }

        let mut current_nodes = &self.nodes;
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
