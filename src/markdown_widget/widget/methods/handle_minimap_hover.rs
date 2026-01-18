//! Handle minimap hover events for interactive resizing.

use crossterm::event::{MouseEvent, MouseEventKind};
use ratatui::layout::Rect;

use crate::markdown_widget::widget::MarkdownWidget;

impl<'a> MarkdownWidget<'a> {
    /// Handle mouse move events to detect minimap hover.
    ///
    /// Call this method with `MouseEventKind::Moved` events to track
    /// whether the mouse is hovering over the minimap area.
    ///
    /// # Arguments
    ///
    /// * `event` - The mouse event (should be a Moved event)
    /// * `area` - The total widget area
    ///
    /// # Returns
    ///
    /// `true` if the hover state changed (entered or exited hover), `false` otherwise.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// // In your event loop:
    /// if let Event::Mouse(mouse_event) = event {
    ///     if matches!(mouse_event.kind, MouseEventKind::Moved) {
    ///         if widget.handle_minimap_hover(&mouse_event, area) {
    ///             // Hover state changed - you may want to redraw
    ///         }
    ///     }
    /// }
    /// ```
    pub fn handle_minimap_hover(&mut self, event: &MouseEvent, area: Rect) -> bool {
        // Only process move events
        if !matches!(event.kind, MouseEventKind::Moved) {
            return false;
        }

        // Get the minimap area
        let minimap_area = match self.calculate_minimap_area(area) {
            Some(mm_area) => mm_area,
            None => {
                // Minimap not visible, ensure not hovered
                if self.minimap_hovered {
                    self.minimap_hovered = false;
                    return true;
                }
                return false;
            }
        };

        // Check if mouse is within minimap area
        let is_over_minimap = event.column >= minimap_area.x
            && event.column < minimap_area.x + minimap_area.width
            && event.row >= minimap_area.y
            && event.row < minimap_area.y + minimap_area.height;

        // Check if state changed
        if is_over_minimap != self.minimap_hovered {
            self.minimap_hovered = is_over_minimap;
            return true;
        }

        false
    }

    /// Check if the minimap is currently being hovered.
    ///
    /// # Returns
    ///
    /// `true` if the mouse is over the minimap, `false` otherwise.
    pub fn is_minimap_hovered(&self) -> bool {
        self.minimap_hovered
    }

    /// Set the minimap hover state directly.
    ///
    /// Useful for manually controlling hover state in tests or special scenarios.
    ///
    /// # Arguments
    ///
    /// * `hovered` - Whether the minimap should be considered hovered.
    pub fn set_minimap_hovered(&mut self, hovered: bool) {
        self.minimap_hovered = hovered;
    }
}
