use ratatui::layout::Rect;

use crate::primitives::split_layout::pane_layout::PaneLayout;

impl PaneLayout {
    /// Returns the rectangle allocated to this pane.
    ///
    /// # Arguments
    /// - None.
    ///
    /// # Returns
    /// The allocated `Rect`.
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
    /// let _ = panes[0].area();
    /// ```
    pub fn area(&self) -> Rect {
        self.area
    }
}
