//! TreeNode::new constructor.

use crate::tree_view::tree_node::TreeNode;

impl<T> TreeNode<T> {
    /// Creates a new tree node with no children.
    ///
    /// The node is created as non-expandable since it has no children.
    ///
    /// # Arguments
    ///
    /// * `data` - The data to store in the node.
    ///
    /// # Returns
    ///
    /// A new `TreeNode` with the given data and no children.
    ///
    /// # Example
    ///
    /// ```rust
    /// use ratatui_toolkit::tree_view::TreeNode;
    ///
    /// let node = TreeNode::new("Hello");
    /// assert!(!node.expandable);
    /// assert!(node.children.is_empty());
    /// ```
    pub fn new(data: T) -> Self {
        Self {
            data,
            children: Vec::new(),
            expandable: false,
        }
    }
}
