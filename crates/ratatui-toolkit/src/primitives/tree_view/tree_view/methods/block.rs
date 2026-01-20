//! TreeView::block method.

use ratatui::widgets::Block;

use crate::primitives::tree_view::tree_view::TreeView;

impl<'a, T> TreeView<'a, T> {
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
    /// use ratatui_toolkit::tree_view::{TreeNode, TreeView};
    ///
    /// let nodes = vec![TreeNode::new("Item")];
    /// let tree = TreeView::new(nodes)
    ///     .block(Block::default().borders(Borders::ALL).title("Tree"));
    /// ```
    pub fn block(mut self, block: Block<'a>) -> Self {
        self.block = Some(block);
        self
    }
}
