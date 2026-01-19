//! Extension trait for mouse event handling.
//!
//! This module defines the trait that extends [`ClickableScrollbarState`] with
//! mouse event handling capabilities.

use crossterm::event::MouseEvent;

use crate::clickable_scrollbar::ScrollbarEvent;

/// Extension trait providing mouse event handling methods for [`ClickableScrollbarState`].
pub trait ClickableScrollbarStateMouseExt {
    /// Handles a mouse event on the scrollbar.
    ///
    /// # Arguments
    ///
    /// * `event` - The mouse event to handle
    ///
    /// # Returns
    ///
    /// A [`ScrollbarEvent`] indicating the result of handling the event
    fn handle_mouse_event(&mut self, event: &MouseEvent) -> ScrollbarEvent;
}
