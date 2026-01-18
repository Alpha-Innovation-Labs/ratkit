//! Tree view widget for rendering hierarchical data.

pub mod constructors;
pub mod methods;
pub mod traits;

use ratatui::{
    style::Style,
    text::Line,
    widgets::Block,
};

use crate::tree_view::node_state::NodeState;

/// Type alias for node render function to reduce complexity.
pub type NodeRenderFn<'a, T> = Box<dyn Fn(&T, &NodeState) -> Line<'a> + 'a>;

/// Tree view widget.
///
/// A widget for rendering hierarchical tree data with expand/collapse
/// functionality, custom rendering, and selection highlighting.
///
/// # Type Parameters
///
/// * `T` - The type of data stored in tree nodes.
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
///     });
/// ```
pub struct TreeView<'a, T> {
    /// Root nodes of the tree
    pub(crate) nodes: Vec<crate::tree_view::tree_node::TreeNode<T>>,
    /// Block to wrap the tree
    pub(crate) block: Option<Block<'a>>,
    /// Render callback for custom node display
    pub(crate) render_fn: NodeRenderFn<'a, T>,
    /// Default expand icon
    pub(crate) expand_icon: &'a str,
    /// Default collapse icon
    pub(crate) collapse_icon: &'a str,
    /// Style for selected row background (full-width highlight)
    pub(crate) highlight_style: Option<Style>,
    /// Style for expand/collapse icons
    pub(crate) icon_style: Style,
}
