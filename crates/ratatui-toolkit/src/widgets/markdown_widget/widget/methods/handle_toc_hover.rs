//! Handle TOC hover events for interactive expansion and entry highlight.

use crossterm::event::{MouseEvent, MouseEventKind};
use ratatui::layout::Rect;

use crate::widgets::markdown_widget::extensions::toc::Toc;
use crate::widgets::markdown_widget::state::toc_state::TocState;
use crate::widgets::markdown_widget::widget::MarkdownWidget;

impl<'a> MarkdownWidget<'a> {
    /// Handle mouse move events to detect TOC hover.
    ///
    /// Call this method with `MouseEventKind::Moved` events to track
    /// whether the mouse is hovering over the TOC area and which entry
    /// is being hovered.
    ///
    /// # Arguments
    ///
    /// * `event` - The mouse event (should be a Moved event)
    /// * `area` - The total widget area
    ///
    /// # Returns
    ///
    /// `true` if the hover state changed (entered/exited hover or hovered entry changed),
    /// `false` otherwise.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// // In your event loop:
    /// if let Event::Mouse(mouse_event) = event {
    ///     if matches!(mouse_event.kind, MouseEventKind::Moved) {
    ///         if widget.handle_toc_hover(&mouse_event, area) {
    ///             // Hover state changed - you may want to redraw
    ///         }
    ///     }
    /// }
    /// ```
    pub fn handle_toc_hover(&mut self, event: &MouseEvent, area: Rect) -> bool {
        // Only process move events
        if !matches!(event.kind, MouseEventKind::Moved) {
            return false;
        }

        // Get the TOC area
        let toc_area = match self.calculate_toc_area(area) {
            Some(t_area) => t_area,
            None => {
                // TOC not visible, ensure not hovered
                let changed = self.toc_hovered || self.toc_hovered_entry.is_some();
                if changed {
                    self.toc_hovered = false;
                    self.toc_hovered_entry = None;
                }
                return changed;
            }
        };

        // Check if mouse is within TOC area horizontally and at or below top
        // Don't check lower vertical bound - let entry_at_position handle that
        // based on actual entry count
        let is_potentially_over_toc = event.column >= toc_area.x
            && event.column < toc_area.x + toc_area.width
            && event.row >= toc_area.y;

        let prev_hovered = self.toc_hovered;
        let prev_entry = self.toc_hovered_entry;

        if is_potentially_over_toc {
            // Create state from content with entries
            let auto_state = TocState::from_content(&self.content);
            let toc_state = if let Some(provided) = &self.toc_state {
                if provided.entries.is_empty() {
                    &auto_state
                } else {
                    provided
                }
            } else {
                &auto_state
            };

            // Try to find an entry at this position
            // Use compact mode when not hovered, expanded mode when hovered
            let toc = Toc::new(toc_state)
                .expanded(self.toc_hovered)
                .config(self.toc_config.clone());

            let entry = toc.entry_at_position(event.column, event.row, toc_area);

            // Only consider hovering if we found an entry
            if entry.is_some() {
                self.toc_hovered = true;
                self.toc_hovered_entry = entry;
            } else {
                self.toc_hovered = false;
                self.toc_hovered_entry = None;
            }
        } else {
            self.toc_hovered = false;
            self.toc_hovered_entry = None;
        }

        // Check if any state changed
        prev_hovered != self.toc_hovered || prev_entry != self.toc_hovered_entry
    }

    /// Check if the TOC is currently being hovered.
    ///
    /// # Returns
    ///
    /// `true` if the mouse is over the TOC, `false` otherwise.
    pub fn is_toc_hovered(&self) -> bool {
        self.toc_hovered
    }

    /// Get the currently hovered TOC entry index.
    ///
    /// # Returns
    ///
    /// The index of the hovered entry, or `None` if no entry is hovered.
    pub fn get_toc_hovered_entry(&self) -> Option<usize> {
        self.toc_hovered_entry
    }

    /// Set the TOC hover state directly.
    ///
    /// Useful for manually controlling hover state in tests or special scenarios.
    ///
    /// # Arguments
    ///
    /// * `hovered` - Whether the TOC should be considered hovered.
    pub fn set_toc_hovered(&mut self, hovered: bool) {
        self.toc_hovered = hovered;
        if !hovered {
            self.toc_hovered_entry = None;
        }
    }

    /// Get the current TOC scroll offset.
    ///
    /// # Returns
    ///
    /// The current scroll offset for the TOC list.
    pub fn get_toc_scroll_offset(&self) -> usize {
        self.toc_scroll_offset
    }

    /// Set the TOC scroll offset directly.
    ///
    /// # Arguments
    ///
    /// * `offset` - The scroll offset for the TOC list.
    pub fn set_toc_scroll_offset(&mut self, offset: usize) {
        self.toc_scroll_offset = offset;
    }

    /// Update the hovered entry based on current mouse position and scroll offset.
    ///
    /// Call this after scrolling the TOC to recalculate which entry is under the cursor.
    ///
    /// # Arguments
    ///
    /// * `x` - Mouse X coordinate
    /// * `y` - Mouse Y coordinate
    /// * `toc_area` - The TOC area rect
    pub fn update_toc_hovered_entry(&mut self, x: u16, y: u16, toc_area: Rect) {
        // Create state from content with entries
        let auto_state = TocState::from_content(&self.content);
        let toc_state = if let Some(provided) = &self.toc_state {
            if provided.entries.is_empty() {
                &auto_state
            } else {
                provided
            }
        } else {
            &auto_state
        };

        let toc = Toc::new(toc_state)
            .expanded(true) // Use expanded mode for entry detection when hovered
            .config(self.toc_config.clone()); // Use same config as rendering

        self.toc_hovered_entry = toc.entry_at_position(x, y, toc_area);
    }
}
