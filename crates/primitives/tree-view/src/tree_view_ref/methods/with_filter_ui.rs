//! Add built-in filter UI to TreeViewRef.

use crate::tree_view_ref::TreeViewRef;

impl<'a, 'b, T> TreeViewRef<'a, 'b, T> {
    /// Enable or disable the built-in filter UI.
    ///
    /// When enabled, the tree view will automatically render a filter input line
    /// at the bottom of the widget when filter mode is active or a filter is set.
    ///
    /// This eliminates the need for manual buffer manipulation when rendering
    /// the filter cursor.
    ///
    /// # Example
    ///
    /// ```rust
    /// use ratatui_toolkit::tree_view::{TreeNode, TreeViewRef};
    ///
    /// let nodes = vec![TreeNode::new("Root")];
    /// let tree = TreeViewRef::new(&nodes)
    ///     .render_fn(|data, state| {
    ///         ratatui::text::Line::from(*data)
    ///     })
    ///     .with_filter_ui(true);
    /// ```
    pub fn with_filter_ui(mut self, show: bool) -> Self {
        self.show_filter_ui = show;
        self
    }
}
