//! TreeViewRef::block method.

use ratatui::widgets::Block;

use crate::primitives::tree_view::tree_view_ref::TreeViewRef;

impl<'a, 'b, T> TreeViewRef<'a, 'b, T> {
    /// Sets the block to wrap the tree.
    ///
    /// # Arguments
    ///
    /// * `block` - The block to wrap the tree view.
    ///
    /// # Returns
    ///
    /// Self for method chaining.
    ///
    /// # Example
    ///
    /// ```rust
    /// use ratatui::widgets::{Block, Borders};
    /// use ratatui_toolkit::tree_view::{TreeNode, TreeViewRef};
    ///
    /// let nodes = vec![TreeNode::new("Item")];
    /// let tree = TreeViewRef::new(&nodes)
    ///     .block(Block::default().borders(Borders::ALL).title("Tree"));
    /// ```
    pub fn block(mut self, block: Block<'a>) -> Self {
        self.block = Some(block);
        self
    }
}
