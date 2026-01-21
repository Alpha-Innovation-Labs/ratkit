use crate::primitives::split_layout::pane_layout::PaneLayout;
use crate::primitives::split_layout::PaneId;

impl PaneLayout {
    /// Returns the identifier for this pane.
    ///
    /// # Arguments
    /// - None.
    ///
    /// # Returns
    /// The pane identifier.
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
    /// use ratatui::layout::Rect;
    /// use ratatui_toolkit::primitives::split_layout::SplitLayout;
    ///
    /// let layout = SplitLayout::new(0);
    /// let panes = layout.layout_panes(Rect::new(0, 0, 10, 5));
    /// let _ = panes[0].pane_id();
    /// ```
    pub fn pane_id(&self) -> PaneId {
        self.pane_id
    }
}
