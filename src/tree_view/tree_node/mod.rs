//! Tree node type for hierarchical data representation.

mod constructors;

/// A node in the tree.
///
/// Represents a single node with data and optional children.
/// Each node can be marked as expandable (has children) for proper
/// UI rendering of expand/collapse indicators.
///
/// # Type Parameters
///
/// * `T` - The type of data stored in the node.
///
/// # Example
///
/// ```rust
/// use ratatui_toolkit::tree_view::TreeNode;
///
/// let leaf = TreeNode::new("Leaf");
/// let parent = TreeNode::with_children("Parent", vec![leaf]);
/// ```
#[derive(Debug, Clone)]
pub struct TreeNode<T> {
    /// Node data
    pub data: T,
    /// Child nodes
    pub children: Vec<TreeNode<T>>,
    /// Whether this node can be expanded (has children)
    pub expandable: bool,
}
