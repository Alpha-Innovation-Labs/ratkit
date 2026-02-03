//! TreeView::icons method.

use crate::widget::TreeView;

impl<'a, T> TreeView<'a, T> {
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
    /// use ratatui_toolkit::tree_view::{TreeNode, TreeView};
    ///
    /// let nodes = vec![TreeNode::new("Item")];
    /// let tree = TreeView::new(nodes)
    ///     .icons("+", "-");
    /// ```
    pub fn icons(mut self, expand: &'a str, collapse: &'a str) -> Self {
        self.expand_icon = expand;
        self.collapse_icon = collapse;
        self
    }
}
