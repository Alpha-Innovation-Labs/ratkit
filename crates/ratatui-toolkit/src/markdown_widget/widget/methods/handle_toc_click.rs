//! Handle TOC click events for scroll-to-heading navigation.

use crossterm::event::{MouseButton, MouseEvent, MouseEventKind};
use ratatui::layout::Rect;

use crate::markdown_widget::extensions::toc::Toc;
use crate::markdown_widget::state::toc_state::TocState;
use crate::markdown_widget::widget::MarkdownWidget;

impl<'a> MarkdownWidget<'a> {
    /// Handle a click on the TOC to scroll to the selected heading.
    ///
    /// # Arguments
    ///
    /// * `event` - The mouse event
    /// * `area` - The total widget area
    ///
    /// # Returns
    ///
    /// `true` if the click was handled (was on a TOC entry), `false` otherwise.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// // In your event loop:
    /// if let Event::Mouse(mouse_event) = event {
    ///     if matches!(mouse_event.kind, MouseEventKind::Down(MouseButton::Left)) {
    ///         if widget.handle_toc_click(&mouse_event, area) {
    ///             // Click was handled - you may want to redraw
    ///         }
    ///     }
    /// }
    /// ```
    pub fn handle_toc_click(&mut self, event: &MouseEvent, area: Rect) -> bool {
        // Only handle left clicks
        if !matches!(event.kind, MouseEventKind::Down(MouseButton::Left)) {
            return false;
        }

        // Get the TOC area
        let toc_area = match self.calculate_toc_area(area) {
            Some(t_area) => t_area,
            None => return false,
        };

        // Check horizontal bounds and if above TOC
        // Don't check lower vertical bound - let entry_at_position handle that
        // based on actual entry count
        if event.column < toc_area.x
            || event.column >= toc_area.x + toc_area.width
            || event.row < toc_area.y
        {
            return false;
        }

        // Create state from content with entries
        let auto_state = TocState::from_content(self.content);
        let toc_state = if let Some(provided) = self.toc_state {
            if provided.entries.is_empty() {
                &auto_state
            } else {
                provided
            }
        } else {
            &auto_state
        };

        // Create a TOC to find the clicked entry
        let toc = Toc::new(toc_state)
            .expanded(self.toc_hovered) // Use current expansion state
            .config(self.toc_config.clone());

        // Find which entry was clicked
        if let Some(entry_idx) = toc.entry_at_position(event.column, event.row, toc_area) {
            // Get the target line number
            if let Some(target_line) = toc.click_to_line(entry_idx) {
                // Scroll to the heading (a bit above for context)
                let new_offset = target_line.saturating_sub(2);

                // Clamp to valid range
                let max_offset = self
                    .scroll
                    .total_lines
                    .saturating_sub(self.scroll.viewport_height);
                self.scroll.scroll_offset = new_offset.min(max_offset);

                // Update current line
                self.scroll.current_line = target_line.saturating_add(1); // 1-indexed

                // Update hovered entry to match the clicked entry
                self.toc_hovered_entry = Some(entry_idx);

                return true;
            }
        }

        false
    }

    /// Handle a click on the TOC in a specific area (for when area is pre-calculated).
    ///
    /// # Arguments
    ///
    /// * `event` - The mouse event
    /// * `toc_area` - The pre-calculated TOC area
    ///
    /// # Returns
    ///
    /// `true` if the click was handled (was on a TOC entry), `false` otherwise.
    pub fn handle_toc_click_in_area(&mut self, event: &MouseEvent, toc_area: Rect) -> bool {
        // Only handle left clicks
        if !matches!(event.kind, MouseEventKind::Down(MouseButton::Left)) {
            return false;
        }

        // Check horizontal bounds and if above TOC
        // Don't check lower vertical bound - let entry_at_position handle that
        if event.column < toc_area.x
            || event.column >= toc_area.x + toc_area.width
            || event.row < toc_area.y
        {
            return false;
        }

        // Create state from content with entries
        let auto_state = TocState::from_content(self.content);
        let toc_state = if let Some(provided) = self.toc_state {
            if provided.entries.is_empty() {
                &auto_state
            } else {
                provided
            }
        } else {
            &auto_state
        };

        // Create a TOC to find the clicked entry
        let toc = Toc::new(toc_state)
            .expanded(self.toc_hovered)
            .config(self.toc_config.clone());

        // Find which entry was clicked
        if let Some(entry_idx) = toc.entry_at_position(event.column, event.row, toc_area) {
            // Get the target line number
            if let Some(target_line) = toc.click_to_line(entry_idx) {
                // Scroll to the heading
                let new_offset = target_line.saturating_sub(2);
                let max_offset = self
                    .scroll
                    .total_lines
                    .saturating_sub(self.scroll.viewport_height);
                self.scroll.scroll_offset = new_offset.min(max_offset);
                self.scroll.current_line = target_line.saturating_add(1);

                // Update hovered entry to match the clicked entry
                self.toc_hovered_entry = Some(entry_idx);

                return true;
            }
        }

        false
    }
}
