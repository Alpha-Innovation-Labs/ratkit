//! TreeViewRef::icons method.

use crate::tree_view_ref::TreeViewRef;

impl<'a, 'b, T> TreeViewRef<'a, 'b, T> {
    /// Sets custom expand/collapse icons.
    ///
    /// # Arguments
    ///
    /// * `expand` - The icon to show for collapsed nodes.
    /// * `collapse` - The icon to show for expanded nodes.
    ///
    /// # Returns
    ///
    /// Self for method chaining.
    ///
    /// # Example
    ///
    /// ```rust
    /// use ratatui_toolkit::tree_view::{TreeNode, TreeViewRef};
    ///
    /// let nodes = vec![TreeNode::new("Item")];
    /// let tree = TreeViewRef::new(&nodes)
    ///     .icons("+", "-");
    /// ```
    pub fn icons(mut self, expand: &'a str, collapse: &'a str) -> Self {
        self.expand_icon = expand;
        self.collapse_icon = collapse;
        self
    }
}
