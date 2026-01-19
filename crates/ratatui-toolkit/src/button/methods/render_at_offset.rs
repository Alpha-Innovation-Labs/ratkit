use ratatui::layout::Rect;
use ratatui::text::Span;

use crate::button::Button;

impl Button {
    /// Renders the button at a specified offset from the right edge.
    ///
    /// # Arguments
    ///
    /// * `panel_area` - The area where the button will be rendered
    /// * `offset_from_right` - Distance from the right edge of the panel
    ///
    /// # Returns
    ///
    /// A tuple containing the styled span and the rendered area
    pub fn render_at_offset(
        &self,
        panel_area: Rect,
        offset_from_right: u16,
    ) -> (Span<'static>, Rect) {
        let button_text = format!(" [{}] ", self.text);
        let button_width = button_text.len() as u16;
        let button_x = panel_area.x
            + panel_area
                .width
                .saturating_sub(offset_from_right + button_width + 2);
        let button_y = panel_area.y;

        let area = Rect {
            x: button_x,
            y: button_y,
            width: button_width,
            height: 1,
        };

        let style = if self.hovered {
            self.hover_style
        } else {
            self.normal_style
        };

        (Span::styled(button_text, style), area)
    }
}
