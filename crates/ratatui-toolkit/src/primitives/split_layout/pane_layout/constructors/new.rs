use ratatui::layout::Rect;

use crate::primitives::split_layout::pane_layout::PaneLayout;
use crate::primitives::split_layout::PaneId;

impl PaneLayout {
    /// Creates a new pane layout from an ID and rectangle.
    ///
    /// # Arguments
    /// - `pane_id`: Identifier for the pane.
    /// - `area`: The rectangle allocated to the pane.
    ///
    /// # Returns
    /// A `PaneLayout` containing the provided values.
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
    /// let _ = panes.first();
    /// ```
    pub(crate) fn new(pane_id: PaneId, area: Rect) -> Self {
        Self { pane_id, area }
    }
}
