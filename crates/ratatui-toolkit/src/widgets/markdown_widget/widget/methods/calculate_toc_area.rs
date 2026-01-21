//! Calculate the TOC area.

use ratatui::layout::Rect;

use crate::widgets::markdown_widget::extensions::toc::Toc;
use crate::widgets::markdown_widget::widget::MarkdownWidget;

impl<'a> MarkdownWidget<'a> {
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
        let toc_width = if self.toc_hovered {
            // Dynamic width based on content for expanded mode
            Toc::required_expanded_width(self.content, self.toc_config.show_border)
                .min(main_area.width.saturating_sub(padding_right + 4))
        } else {
            self.toc_config.compact_width
        };

        let toc_height = if self.toc_hovered {
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
