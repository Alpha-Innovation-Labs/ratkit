//! Constructors for `ResizableGrid`.

use crate::types::{LayoutNode, PaneId, ResizableGrid};

impl ResizableGrid {
    /// Creates a new grid with a single pane as the root.
    ///
    /// # Arguments
    /// - `pane_id`: Identifier for the initial pane.
    ///
    /// # Returns
    /// A new `ResizableGrid` containing the provided pane ID.
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
    /// use ratatui_toolkit::primitives::resizable_grid::ResizableGrid;
    ///
    /// let grid = ResizableGrid::new(0);
    /// ```
    pub fn new(pane_id: PaneId) -> Self {
        Self {
            root_index: 0,
            nodes: vec![LayoutNode::Pane { id: pane_id }],
            next_pane_id: pane_id.saturating_add(1),
            hovered_split: None,
            dragging_split: None,
            hit_threshold: 2,
        }
    }

    /// Creates a new grid from a single pane.
    ///
    /// This is equivalent to [`Self::new()`] but provides a more semantic
    /// name when constructing a grid that will be populated with panes.
    ///
    /// # Arguments
    /// - `pane_id`: Identifier for the initial pane.
    ///
    /// # Returns
    /// A new `ResizableGrid` containing the provided pane ID.
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
    /// use ratatui_toolkit::primitives::resizable_grid::ResizableGrid;
    ///
    /// let grid = ResizableGrid::from_pane(0);
    /// ```
    pub fn from_pane(pane_id: PaneId) -> Self {
        Self::new(pane_id)
    }
}
