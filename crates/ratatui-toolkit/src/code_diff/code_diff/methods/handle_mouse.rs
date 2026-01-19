//! Method to handle mouse events for sidebar resize.

use crossterm::event::{MouseButton, MouseEvent, MouseEventKind};
use ratatui::layout::Rect;

use crate::code_diff::code_diff::CodeDiff;

impl CodeDiff {
    /// Handles mouse events for sidebar resizing.
    ///
    /// Supports mouse drag to resize the sidebar divider using ResizableSplit's
    /// built-in drag handling.
    ///
    /// # Arguments
    ///
    /// * `event` - The mouse event to handle
    /// * `area` - The area the widget is rendered in
    ///
    /// # Returns
    ///
    /// `true` if the event was handled (consumed), `false` otherwise
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use crossterm::event::{Event, MouseEvent};
    /// use ratatui::layout::Rect;
    /// use ratatui_toolkit::code_diff::{CodeDiff, DiffConfig};
    ///
    /// let mut diff = CodeDiff::new()
    ///     .with_config(DiffConfig::new().sidebar_enabled(true));
    ///
    /// // In your event loop:
    /// // if let Event::Mouse(mouse) = event {
    /// //     let area = Rect::new(0, 0, 100, 50);
    /// //     if diff.handle_mouse(mouse, area) {
    /// //         // Mouse event was handled
    /// //     }
    /// // }
    /// ```
    pub fn handle_mouse(&mut self, event: MouseEvent, area: Rect) -> bool {
        // Only handle mouse events when sidebar is visible
        if !self.show_sidebar || !self.config.sidebar_enabled {
            return false;
        }

        // Update divider position for mouse detection
        self.sidebar_split.update_divider_position(area);

        match event.kind {
            MouseEventKind::Down(MouseButton::Left) => {
                if self
                    .sidebar_split
                    .is_on_divider(event.column, event.row, area)
                {
                    self.sidebar_split.start_drag();
                    return true;
                }
            }
            MouseEventKind::Drag(MouseButton::Left) => {
                if self.sidebar_split.is_dragging {
                    self.sidebar_split
                        .update_from_mouse(event.column, event.row, area);
                    return true;
                }
            }
            MouseEventKind::Up(MouseButton::Left) => {
                if self.sidebar_split.is_dragging {
                    self.sidebar_split.stop_drag();
                    return true;
                }
            }
            MouseEventKind::Moved => {
                self.sidebar_split.is_hovering =
                    self.sidebar_split
                        .is_on_divider(event.column, event.row, area);
            }
            _ => {}
        }
        false
    }
}
