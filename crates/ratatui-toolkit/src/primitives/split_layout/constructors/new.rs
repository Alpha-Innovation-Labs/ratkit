use crate::primitives::split_layout::enums::layout_node::LayoutNode;
use crate::primitives::split_layout::PaneId;
use crate::primitives::split_layout::SplitLayout;

impl SplitLayout {
    /// Creates a layout with a single pane as the root.
    ///
    /// # Arguments
    /// - `pane_id`: Identifier for the initial pane.
    ///
    /// # Returns
    /// A new `SplitLayout` containing the provided pane ID.
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
    /// - O(1).
    ///
    /// # Example
    /// ```rust
    /// use ratatui_toolkit::primitives::split_layout::SplitLayout;
    ///
    /// let layout = SplitLayout::new(0);
    /// ```
    pub fn new(pane_id: PaneId) -> Self {
        Self {
            root_index: 0,
            nodes: vec![LayoutNode::Pane { id: pane_id }],
            next_pane_id: pane_id.saturating_add(1),
        }
    }
}
