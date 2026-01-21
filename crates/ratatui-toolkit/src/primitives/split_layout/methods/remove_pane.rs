use crate::primitives::split_layout::enums::layout_node::LayoutNode;
use crate::primitives::split_layout::PaneId;
use crate::primitives::split_layout::SplitLayout;

impl SplitLayout {
    /// Removes a pane and collapses its parent split.
    ///
    /// # Arguments
    /// - `pane_id`: The pane to remove.
    ///
    /// # Returns
    /// `true` when a pane is removed and the split collapses.
    ///
    /// # Errors
    /// - None.
    ///
    /// # Panics
    /// - Does not panic.
    ///
    /// # Safety
    /// - No safety requirements.
    ///
    /// # Performance
    /// - O(n) to locate the pane.
    ///
    /// # Example
    /// ```rust
    /// use ratatui_toolkit::primitives::split_layout::SplitLayout;
    ///
    /// let mut layout = SplitLayout::new(0);
    /// let pane_id = layout.split_pane_vertically(0).unwrap();
    /// let _ = layout.remove_pane(pane_id);
    /// ```
    pub fn remove_pane(&mut self, pane_id: PaneId) -> bool {
        let Some((parent_index, is_first)) = self.find_parent_split(pane_id) else {
            return false;
        };
        let Some(LayoutNode::Split { first, second, .. }) = self.nodes.get(parent_index) else {
            return false;
        };
        let sibling_index = if is_first { *second } else { *first };
        let Some(sibling_node) = self.nodes.get(sibling_index).cloned() else {
            return false;
        };

        self.nodes[parent_index] = sibling_node;
        true
    }
}
