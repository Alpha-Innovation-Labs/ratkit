//! Constructors for [`ClickableScrollbarState`].
//!
//! This module contains constructor functions for creating
//! [`ClickableScrollbarState`] instances.

use ratatui::layout::Rect;
use ratatui::widgets::ScrollbarOrientation;

use crate::clickable_scrollbar::ClickableScrollbarState;

impl ClickableScrollbarState {
    /// Creates a new [`ClickableScrollbarState`] with default values.
    ///
    /// # Returns
    ///
    /// A new [`ClickableScrollbarState`] instance with:
    /// - area: `Rect::default()`
    /// - orientation: `VerticalRight`
    /// - offset: 0
    /// - page_len: 0
    /// - max_offset: 0
    /// - scroll_by: None
    /// - drag_active: false
    pub fn new() -> Self {
        Self {
            area: Rect::default(),
            orientation: ScrollbarOrientation::VerticalRight,
            offset: 0,
            page_len: 0,
            max_offset: 0,
            scroll_by: None,
            drag_active: false,
        }
    }
}
