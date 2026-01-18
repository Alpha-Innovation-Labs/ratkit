//! Calculate the minimap area.

use ratatui::layout::Rect;

use crate::markdown_widget::widget::MarkdownWidget;

impl<'a> MarkdownWidget<'a> {
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
