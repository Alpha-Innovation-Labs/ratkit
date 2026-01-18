//! Handle minimap click events for scroll-to-position.

use crossterm::event::{MouseButton, MouseEvent, MouseEventKind};
use ratatui::layout::Rect;

use super::super::MarkdownWidget;
use crate::markdown_renderer::minimap::Minimap;

impl<'a> MarkdownWidget<'a> {
    /// Handle a click on the minimap to scroll to that position.
    ///
    /// # Arguments
    ///
    /// * `event` - The mouse event
    /// * `minimap_area` - The area occupied by the minimap
    ///
    /// # Returns
    ///
    /// `true` if the click was handled (was in minimap area), `false` otherwise.
    pub fn handle_minimap_click(&mut self, event: &MouseEvent, minimap_area: Rect) -> bool {
        // Only handle left clicks
        if !matches!(event.kind, MouseEventKind::Down(MouseButton::Left)) {
            return false;
        }

        // Check if click is within minimap area
        if event.column < minimap_area.x
            || event.column >= minimap_area.x + minimap_area.width
            || event.row < minimap_area.y
            || event.row >= minimap_area.y + minimap_area.height
        {
            return false;
        }

        // Calculate the clicked position within minimap
        let minimap_y = (event.row - minimap_area.y) as usize;
        let minimap_height = minimap_area.height as usize;

        // Use the Minimap to calculate the target line
        let minimap = Minimap::new(self.content).viewport(
            self.scroll.scroll_offset,
            self.scroll.scroll_offset + self.scroll.viewport_height,
            self.scroll.total_lines,
        );

        let target_line = minimap.click_to_line(minimap_y, minimap_height);

        // Scroll to center the target line in the viewport
        let half_viewport = self.scroll.viewport_height / 2;
        let new_offset = target_line.saturating_sub(half_viewport);

        // Clamp to valid range
        let max_offset = self
            .scroll
            .total_lines
            .saturating_sub(self.scroll.viewport_height);
        self.scroll.scroll_offset = new_offset.min(max_offset);

        // Update current line
        self.scroll.current_line = target_line.saturating_add(1); // 1-indexed

        true
    }

    /// Calculate the minimap area based on current widget configuration.
    ///
    /// # Arguments
    ///
    /// * `total_area` - The total area available for the widget
    ///
    /// # Returns
    ///
    /// `Some(Rect)` with the minimap area if minimap is enabled, `None` otherwise.
    pub fn calculate_minimap_area(&self, total_area: Rect) -> Option<Rect> {
        if !self.show_minimap {
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

        // When hovered, expand the minimap for better visibility
        let hover_scale: u16 = if self.minimap_hovered { 2 } else { 1 };
        let minimap_width = self.minimap_config.width * hover_scale;
        let minimap_height =
            (self.minimap_config.height * hover_scale).min(main_area.height.saturating_sub(1));
        let padding_right: u16 = 2;
        let padding_top: u16 = 1;
        if main_area.width <= minimap_width + padding_right + 2 {
            return None;
        }

        Some(Rect {
            x: main_area.x
                + main_area
                    .width
                    .saturating_sub(minimap_width + padding_right),
            y: main_area.y + padding_top,
            width: minimap_width,
            height: minimap_height,
        })
    }
}
