use ratatui::layout::Rect;
use ratatui::text::Line;
use ratatui::text::Span;

use crate::button::Button;

impl Button {
    /// Renders the button alongside a title line.
    ///
    /// # Arguments
    ///
    /// * `panel_area` - The area where the button will be rendered
    /// * `title` - The title text to display
    ///
    /// # Returns
    ///
    /// A `Line` containing the title span and the button span
    ///
    /// # Note
    ///
    /// This method updates the button's internal area for click detection
    pub fn render_with_title(&mut self, panel_area: Rect, title: &str) -> Line<'static> {
        let (button_span, area) = self.render(panel_area, title);
        self.area = Some(area);
        let title_line = Line::from(vec![Span::raw(title.to_string()), button_span]);
        title_line
    }
}
