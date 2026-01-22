use crate::primitives::split_layout::constants::{MAX_SPLIT_PERCENT, MIN_SPLIT_PERCENT};
use crate::primitives::split_layout::enums::layout_node::LayoutNode;
use crate::primitives::split_layout::PaneId;
use crate::primitives::split_layout::SplitLayout;

impl SplitLayout {
    /// Resizes the split containing a pane.
    ///
    /// # Arguments
    /// - `pane_id`: The pane whose split divider should be resized.
    /// - `percent`: The percentage to allocate to the pane within its split.
    ///
    /// Use this when you only have a pane id and want to set that pane's
    /// percentage of its containing split. If you already know the split
    /// node index, use `resize_split` instead.
    ///
    /// # Returns
    /// `true` when the pane is part of a split and the divider was updated.
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
    /// let _ = layout.resize_divider(pane_id, 30);
    /// ```
    pub fn resize_divider(&mut self, pane_id: PaneId, percent: u16) -> bool {
        let Some((parent_index, is_first)) = self.find_parent_split(pane_id) else {
            return false;
        };
        let Some(LayoutNode::Split { ratio, .. }) = self.nodes.get_mut(parent_index) else {
            return false;
        };

        let new_ratio = if is_first {
            percent
        } else {
            100_u16.saturating_sub(percent)
        };

        *ratio = new_ratio.clamp(MIN_SPLIT_PERCENT, MAX_SPLIT_PERCENT);
        true
    }
}
