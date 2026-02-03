//! TreeViewRef::new constructor.

use ratatui::{
    style::{Color, Style},
    text::Line,
};

use crate::primitives::tree_view::tree_node::TreeNode;
use crate::primitives::tree_view::tree_view_ref::TreeViewRef;

impl<'a, 'b, T> TreeViewRef<'a, 'b, T> {
    /// Creates a new tree view with a reference to nodes (avoids cloning).
    ///
    /// # Arguments
    ///
    /// * `nodes` - A reference to the root nodes of the tree.
    ///
    /// # Returns
    ///
    /// A new `TreeViewRef` with default settings.
    ///
    /// # Example
    ///
    /// ```rust
    /// use ratatui_toolkit::tree_view::{TreeNode, TreeViewRef};
    ///
    /// let nodes = vec![TreeNode::new("Item")];
    /// let tree = TreeViewRef::new(&nodes);
    /// ```
    pub fn new(nodes: &'b [TreeNode<T>]) -> Self {
        Self {
            nodes,
            block: None,
            render_fn: Box::new(|_data, _state| Line::from("Node")),
            expand_icon: "\u{25b6}",
            collapse_icon: "\u{25bc}",
            highlight_style: None,
            icon_style: Style::default().fg(Color::DarkGray),
            show_filter_ui: false,
        }
    }
}
