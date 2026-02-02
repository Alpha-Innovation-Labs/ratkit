//! Direct event handling methods for MarkdownWidget.

use crate::widgets::markdown_widget::foundation::events::MarkdownEvent;
use crate::widgets::markdown_widget::widget::MarkdownWidget;
use crossterm::event::{KeyEvent, MouseEvent};
use ratatui::layout::Rect;

impl MarkdownWidget<'_> {
    /// Handle a keyboard event for navigation and actions.
    ///
    /// This is a convenience method that delegates to `handle_key_event`.
    /// The widget manages all state internally.
    ///
    /// # Arguments
    ///
    /// * `key` - The keyboard event to handle
    ///
    /// # Returns
    ///
    /// A `MarkdownEvent` indicating what action was taken.
    pub fn handle_key(&mut self, key: KeyEvent) -> MarkdownEvent {
        self.handle_key_event(key)
    }

    /// Handle a mouse event for all interactions.
    ///
    /// This is a convenience method that delegates to the internal handler.
    /// The widget manages all state internally.
    ///
    /// # Arguments
    ///
    /// * `event` - The mouse event to handle
    /// * `area` - The area the widget occupies (for bounds checking)
    ///
    /// # Returns
    ///
    /// A `MarkdownEvent` indicating what action was taken.
    pub fn handle_mouse(&mut self, event: MouseEvent, area: Rect) -> MarkdownEvent {
        self.handle_mouse_internal(&event, area)
    }

    /// Update git stats for the current content.
    ///
    /// This method loads git stats for the file if the source is a file path.
    pub fn update_git_stats(&mut self) {
        self.git_stats_state.update(self.source.source_path());
    }
}
