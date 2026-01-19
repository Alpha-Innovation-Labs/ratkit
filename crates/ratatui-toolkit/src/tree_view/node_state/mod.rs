//! Node state information for rendering.

/// State information for rendering a node.
///
/// Provides context about a node's current state during rendering,
/// allowing the render function to customize the display based on
/// selection, expansion, and position in the tree.
///
/// # Example
///
/// ```rust
/// use ratatui_toolkit::tree_view::NodeState;
///
/// let state = NodeState {
///     is_selected: true,
///     is_expanded: false,
///     level: 0,
///     has_children: true,
///     path: vec![0],
/// };
/// ```
#[derive(Debug, Clone)]
pub struct NodeState {
    /// Whether this node is selected
    pub is_selected: bool,
    /// Whether this node is expanded
    pub is_expanded: bool,
    /// Depth level in the tree (0 = root)
    pub level: usize,
    /// Whether this node has children
    pub has_children: bool,
    /// Path to this node (indices from root)
    pub path: Vec<usize>,
}
