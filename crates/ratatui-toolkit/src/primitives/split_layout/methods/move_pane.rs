use crate::primitives::split_layout::enums::layout_node::LayoutNode;
use crate::primitives::split_layout::PaneId;
use crate::primitives::split_layout::SplitLayout;

impl SplitLayout {
    /// Moves a pane by swapping it with another pane.
    ///
    /// # Arguments
    /// - `pane_id`: The pane to move.
    /// - `target_pane_id`: The pane that receives the original pane's position.
    ///
    /// # Returns
    /// `true` when both panes exist and the swap completed.
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
    /// - O(n) to locate both panes.
    ///
    /// # Example
    /// ```rust
    /// use ratatui_toolkit::primitives::split_layout::SplitLayout;
    ///
    /// let mut layout = SplitLayout::new(1);
    /// let pane_id = layout.split_pane_vertically(1).unwrap();
    /// let _ = layout.move_pane(1, pane_id);
    /// ```
    pub fn move_pane(&mut self, pane_id: PaneId, target_pane_id: PaneId) -> bool {
        let Some(source_index) = self.find_pane_node_index(pane_id) else {
            return false;
        };
        let Some(target_index) = self.find_pane_node_index(target_pane_id) else {
            return false;
        };

        if source_index == target_index {
            return true;
        }

        let source_id = match self.nodes.get(source_index) {
            Some(LayoutNode::Pane { id }) => *id,
            _ => return false,
        };
        let target_id = match self.nodes.get(target_index) {
            Some(LayoutNode::Pane { id }) => *id,
            _ => return false,
        };

        self.nodes[source_index] = LayoutNode::Pane { id: target_id };
        self.nodes[target_index] = LayoutNode::Pane { id: source_id };

        true
    }
}
