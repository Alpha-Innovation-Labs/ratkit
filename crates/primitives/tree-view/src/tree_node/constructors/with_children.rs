//! TreeNode::with_children constructor.

use crate::tree_node::TreeNode;

impl<T> TreeNode<T> {
    /// Creates a new tree node with children.
    ///
    /// The node is automatically marked as expandable if children are provided.
    ///
    /// # Arguments
    ///
    /// * `data` - The data to store in the node.
    /// * `children` - The child nodes.
    ///
    /// # Returns
    ///
    /// A new `TreeNode` with the given data and children.
    ///
    /// # Example
    ///
    /// ```rust
    /// use ratatui_toolkit::tree_view::TreeNode;
    ///
    /// let child = TreeNode::new("Child");
    /// let parent = TreeNode::with_children("Parent", vec![child]);
    /// assert!(parent.expandable);
    /// assert_eq!(parent.children.len(), 1);
    /// ```
    pub fn with_children(data: T, children: Vec<TreeNode<T>>) -> Self {
        let expandable = !children.is_empty();
        Self {
            data,
            children,
            expandable,
        }
    }
}
