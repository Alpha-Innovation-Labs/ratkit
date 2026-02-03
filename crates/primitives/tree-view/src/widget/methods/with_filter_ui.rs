//! Add built-in filter UI to the tree view.

use crate::widget::TreeView;

impl<'a, T> TreeView<'a, T> {
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
    /// use ratatui_toolkit::tree_view::{TreeNode, TreeView};
    ///
    /// let nodes = vec![TreeNode::new("Root")];
    /// let tree = TreeView::new(nodes)
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
