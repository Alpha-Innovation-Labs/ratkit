//! TreeViewRef::filter_fn method.

use crate::tree_view::tree_view_ref::TreeViewRef;

impl<'a, 'b, T> TreeViewRef<'a, 'b, T> {
    /// Sets a filter function for filtering nodes based on the filter text.
    ///
    /// The function receives the node data and the current filter text,
    /// and returns true if the node should be visible.
    ///
    /// Note: This method is currently a no-op for API compatibility.
    /// Filtering is typically handled externally via `get_visible_paths_filtered`.
    ///
    /// # Arguments
    ///
    /// * `f` - A function that takes node data and filter, returns true if matches.
    ///
    /// # Returns
    ///
    /// Self for method chaining.
    ///
    /// # Example
    ///
    /// ```rust
    /// use ratatui_toolkit::tree_view::{TreeNode, TreeViewRef, helpers::matches_filter};
    ///
    /// let nodes = vec![TreeNode::new("Item")];
    /// let tree = TreeViewRef::new(&nodes)
    ///     .filter_fn(|data, filter| matches_filter(data, filter));
    /// ```
    pub fn filter_fn<F>(self, _f: F) -> Self
    where
        F: Fn(&T, &Option<String>) -> bool + 'a,
    {
        // Note: The filter function is stored but filtering is done externally
        // via get_visible_paths_filtered. This method exists for API compatibility.
        self
    }
}
