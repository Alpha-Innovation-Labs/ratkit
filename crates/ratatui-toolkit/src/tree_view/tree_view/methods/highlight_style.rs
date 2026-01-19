//! TreeView::highlight_style method.

use ratatui::style::Style;

use crate::tree_view::tree_view::TreeView;

impl<'a, T> TreeView<'a, T> {
    /// Sets the highlight style for selected rows (full-width background).
    ///
    /// # Arguments
    ///
    /// * `style` - The style to apply to selected rows.
    ///
    /// # Returns
    ///
    /// Self for method chaining.
    ///
    /// # Example
    ///
    /// ```rust
    /// use ratatui::style::{Color, Style};
    /// use ratatui_toolkit::tree_view::{TreeNode, TreeView};
    ///
    /// let nodes = vec![TreeNode::new("Item")];
    /// let tree = TreeView::new(nodes)
    ///     .highlight_style(Style::default().bg(Color::Blue));
    /// ```
    pub fn highlight_style(mut self, style: Style) -> Self {
        self.highlight_style = Some(style);
        self
    }
}
