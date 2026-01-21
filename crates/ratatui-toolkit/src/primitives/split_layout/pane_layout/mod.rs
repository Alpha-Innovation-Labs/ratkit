//! Pane layout output for `SplitLayout`.
//!
//! Contains the pane identifier and its calculated rectangle.

pub mod constructors;
pub mod methods;

use ratatui::layout::Rect;

use crate::primitives::split_layout::PaneId;

/// A computed rectangle for a pane.
///
/// Combines a pane identifier with the rectangle allocated by a split layout.
///
/// # Example
/// ```rust
/// use ratatui::layout::Rect;
/// use ratatui_toolkit::primitives::split_layout::SplitLayout;
///
/// let layout = SplitLayout::new(1);
/// let panes = layout.layout_panes(Rect::new(0, 0, 10, 5));
/// let _ = panes[0].pane_id();
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PaneLayout {
    pane_id: PaneId,
    area: Rect,
}
