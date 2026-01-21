use crate::primitives::split_layout::PaneId;
use crate::primitives::split_layout::SplitAxis;
use crate::primitives::split_layout::SplitLayout;

impl SplitLayout {
    /// Splits a pane into a top and bottom pair.
    ///
    /// # Arguments
    /// - `pane_id`: The pane to split.
    ///
    /// # Returns
    /// The newly created pane ID when the split succeeds, or `None` when the
    /// pane cannot be found.
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
    /// let _ = layout.split_pane_horizontally(0);
    /// ```
    pub fn split_pane_horizontally(&mut self, pane_id: PaneId) -> Option<PaneId> {
        self.split_pane(pane_id, SplitAxis::Horizontal)
    }
}
