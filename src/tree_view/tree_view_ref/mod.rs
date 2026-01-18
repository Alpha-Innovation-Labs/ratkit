//! Tree view widget that borrows nodes instead of owning them.

pub mod constructors;
pub mod methods;
pub mod traits;

use ratatui::{
    style::Style,
    text::Line,
    widgets::Block,
};

use crate::tree_view::node_state::NodeState;
use crate::tree_view::tree_node::TreeNode;

/// Type alias for node render function that borrows data (for TreeViewRef).
pub type NodeRenderRefFn<'a, T> = Box<dyn Fn(&T, &NodeState) -> Line<'a> + 'a>;

/// Type alias for node filter function.
pub type NodeFilterFn<T> = Box<dyn Fn(&T, &Option<String>) -> bool>;

/// Tree view widget that borrows nodes instead of owning them.
///
/// This is useful when you want to avoid cloning the tree on every render frame.
/// Unlike `TreeView` which takes ownership of nodes, `TreeViewRef` borrows them,
/// allowing the original data to remain in place after rendering.
///
/// # Type Parameters
///
/// * `'a` - Lifetime for borrowed UI elements.
/// * `'b` - Lifetime for borrowed tree nodes.
/// * `T` - The type of data stored in tree nodes.
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
///     });
/// ```
pub struct TreeViewRef<'a, 'b, T> {
    /// Reference to root nodes of the tree
    pub(crate) nodes: &'b [TreeNode<T>],
    /// Block to wrap the tree
    pub(crate) block: Option<Block<'a>>,
    /// Render callback for custom node display
    pub(crate) render_fn: NodeRenderRefFn<'a, T>,
    /// Default expand icon
    pub(crate) expand_icon: &'a str,
    /// Default collapse icon
    pub(crate) collapse_icon: &'a str,
    /// Style for selected row background (full-width highlight)
    pub(crate) highlight_style: Option<Style>,
    /// Style for expand/collapse icons
    pub(crate) icon_style: Style,
}
