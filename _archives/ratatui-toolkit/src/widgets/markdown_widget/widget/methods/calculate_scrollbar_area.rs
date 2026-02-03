//! Calculate the scrollbar area for a given content area.

use ratatui::layout::Rect;

use crate::widgets::markdown_widget::widget::MarkdownWidget;

impl<'a> MarkdownWidget<'a> {
    /// Calculate the scrollbar area based on the content area.
    ///
    /// Returns `Some(Rect)` if the scrollbar should be shown, `None` otherwise.
    ///
    /// # Arguments
    ///
    /// * `area` - The main widget area
    pub fn calculate_scrollbar_area(&self, area: Rect) -> Option<Rect> {
        // Calculate content area (same logic as render)
        let content_area = if self.show_statusline && area.height > 1 {
            Rect {
                height: area.height.saturating_sub(1),
                ..area
            }
        } else {
            area
        };

        // Only show scrollbar if content exceeds viewport
        if !self.show_scrollbar || self.scroll.total_lines <= content_area.height as usize {
            return None;
        }

        let scrollbar_width = self.scrollbar_config.width;

        Some(Rect {
            x: content_area.x + content_area.width.saturating_sub(scrollbar_width),
            y: content_area.y,
            width: scrollbar_width,
            height: content_area.height,
        })
    }
}
