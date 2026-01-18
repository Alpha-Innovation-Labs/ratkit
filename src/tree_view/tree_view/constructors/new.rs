//! TreeView::new constructor.

use ratatui::text::Line;

use crate::tree_view::tree_node::TreeNode;
use crate::tree_view::tree_view::TreeView;

impl<'a, T> TreeView<'a, T> {
    /// Creates a new tree view with nodes.
    ///
    /// # Arguments
    ///
    /// * `nodes` - The root nodes of the tree.
    ///
    /// # Returns
    ///
    /// A new `TreeView` with default settings.
    ///
    /// # Example
    ///
    /// ```rust
    /// use ratatui_toolkit::tree_view::{TreeNode, TreeView};
    ///
    /// let nodes = vec![TreeNode::new("Item")];
    /// let tree = TreeView::new(nodes);
    /// ```
    pub fn new(nodes: Vec<TreeNode<T>>) -> Self {
        Self {
            nodes,
            block: None,
            render_fn: Box::new(|_data, _state| Line::from("Node")),
            expand_icon: "\u{25b6}",
            collapse_icon: "\u{25bc}",
            highlight_style: None,
            icon_style: ratatui::style::Style::default(),
        }
    }
}
