//! TreeViewRef::icon_style method.

use ratatui::style::Style;

use crate::tree_view_ref::TreeViewRef;

impl<'a, 'b, T> TreeViewRef<'a, 'b, T> {
    /// Sets the style for expand/collapse icons.
    ///
    /// # Arguments
    ///
    /// * `style` - The style to apply to icons.
    ///
    /// # Returns
    ///
    /// Self for method chaining.
    ///
    /// # Example
    ///
    /// ```rust
    /// use ratatui::style::{Color, Style};
    /// use ratatui_toolkit::tree_view::{TreeNode, TreeViewRef};
    ///
    /// let nodes = vec![TreeNode::new("Item")];
    /// let tree = TreeViewRef::new(&nodes)
    ///     .icon_style(Style::default().fg(Color::Yellow));
    /// ```
    pub fn icon_style(mut self, style: Style) -> Self {
        self.icon_style = style;
        self
    }
}
