//! Handle TOC hover events for interactive expansion and entry highlight.

use crossterm::event::{MouseEvent, MouseEventKind};
use ratatui::layout::Rect;

use super::super::MarkdownWidget;
use crate::markdown_renderer::toc::Toc;

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
                let changed = self.scroll.toc_hovered || self.scroll.toc_hovered_entry.is_some();
                if changed {
                    self.scroll.toc_hovered = false;
                    self.scroll.toc_hovered_entry = None;
                }
                return changed;
            }
        };

        // Check if mouse is within TOC area
        let is_over_toc = event.column >= toc_area.x
            && event.column < toc_area.x + toc_area.width
            && event.row >= toc_area.y
            && event.row < toc_area.y + toc_area.height;

        let prev_hovered = self.scroll.toc_hovered;
        let prev_entry = self.scroll.toc_hovered_entry;

        if is_over_toc {
            self.scroll.toc_hovered = true;

            // Find which entry is being hovered
            if self.scroll.toc_hovered {
                let viewport_start = self.scroll.scroll_offset;
                let viewport_height = toc_area.height as usize;
                let total_lines = self.scroll.total_lines;

                let toc = Toc::new(self.content)
                    .expanded(true) // Use expanded mode for entry detection
                    .viewport(viewport_start, viewport_height, total_lines)
                    .toc_scroll(self.scroll.toc_scroll_offset)
                    .config(self.toc_config.clone());

                self.scroll.toc_hovered_entry =
                    toc.entry_at_position(event.column, event.row, toc_area);
            }
        } else {
            self.scroll.toc_hovered = false;
            self.scroll.toc_hovered_entry = None;
        }

        // Check if any state changed
        prev_hovered != self.scroll.toc_hovered || prev_entry != self.scroll.toc_hovered_entry
    }

    /// Check if the TOC is currently being hovered.
    ///
    /// # Returns
    ///
    /// `true` if the mouse is over the TOC, `false` otherwise.
    pub fn is_toc_hovered(&self) -> bool {
        self.scroll.toc_hovered
    }

    /// Get the currently hovered TOC entry index.
    ///
    /// # Returns
    ///
    /// The index of the hovered entry, or `None` if no entry is hovered.
    pub fn get_toc_hovered_entry(&self) -> Option<usize> {
        self.scroll.toc_hovered_entry
    }

    /// Set the TOC hover state directly.
    ///
    /// Useful for manually controlling hover state in tests or special scenarios.
    ///
    /// # Arguments
    ///
    /// * `hovered` - Whether the TOC should be considered hovered.
    pub fn set_toc_hovered(&mut self, hovered: bool) {
        self.scroll.toc_hovered = hovered;
        if !hovered {
            self.scroll.toc_hovered_entry = None;
        }
    }

    /// Get the current TOC scroll offset.
    ///
    /// # Returns
    ///
    /// The current scroll offset for the TOC list.
    pub fn get_toc_scroll_offset(&self) -> usize {
        self.scroll.toc_scroll_offset
    }

    /// Set the TOC scroll offset directly.
    ///
    /// # Arguments
    ///
    /// * `offset` - The scroll offset for the TOC list.
    pub fn set_toc_scroll_offset(&mut self, offset: usize) {
        self.scroll.toc_scroll_offset = offset;
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
        let viewport_start = self.scroll.scroll_offset;
        let viewport_height = toc_area.height as usize;
        let total_lines = self.scroll.total_lines;

        let toc = Toc::new(self.content)
            .expanded(true) // Use expanded mode for entry detection when hovered
            .viewport(viewport_start, viewport_height, total_lines)
            .toc_scroll(self.scroll.toc_scroll_offset)
            .config(self.toc_config.clone()); // Use same config as rendering

        self.scroll.toc_hovered_entry = toc.entry_at_position(x, y, toc_area);
    }

    /// Calculate the TOC area based on current widget configuration.
    ///
    /// Uses dynamic dimensions based on content:
    /// - Expanded mode: width fits all headers, height fits all entries
    /// - Compact mode: fixed width, height based on entry count and line spacing
    ///
    /// # Arguments
    ///
    /// * `total_area` - The total area available for the widget
    ///
    /// # Returns
    ///
    /// `Some(Rect)` with the TOC area if TOC is enabled, `None` otherwise.
    pub fn calculate_toc_area(&self, total_area: Rect) -> Option<Rect> {
        if !self.show_toc {
            return None;
        }

        // Account for statusline
        let main_area = if self.show_statusline && total_area.height > 1 {
            Rect {
                height: total_area.height.saturating_sub(1),
                ..total_area
            }
        } else {
            total_area
        };

        let padding_right: u16 = 2;
        let padding_top: u16 = 1;

        // Use dynamic dimensions matching the rendering code
        let toc_width = if self.scroll.toc_hovered {
            // Dynamic width based on content for expanded mode
            Toc::required_expanded_width(self.content, self.toc_config.show_border)
                .min(main_area.width.saturating_sub(padding_right + 4))
        } else {
            self.toc_config.compact_width
        };

        let toc_height = if self.scroll.toc_hovered {
            // Expanded: one row per entry
            Toc::required_height(self.content, self.toc_config.show_border)
                .min(main_area.height.saturating_sub(1))
        } else {
            // Compact: based on entries and line_spacing
            Toc::required_compact_height(
                self.content,
                self.toc_config.line_spacing,
                self.toc_config.show_border,
            )
            .min(main_area.height.saturating_sub(1))
        };

        if main_area.width <= toc_width + padding_right + 2 {
            return None;
        }

        Some(Rect {
            x: main_area.x + main_area.width.saturating_sub(toc_width + padding_right),
            y: main_area.y + padding_top,
            width: toc_width,
            height: toc_height,
        })
    }
}
