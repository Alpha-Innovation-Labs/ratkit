//! Rendering utilities for button widgets.

use ratatui::layout::Rect;
use ratatui::text::Line;
use ratatui::text::Span;

use crate::widget::Button;

pub fn render_title_with_buttons(
    panel_area: Rect,
    title: &str,
    buttons: &mut [&mut Button],
) -> Line<'static> {
    let mut spans = vec![Span::raw(title.to_string())];

    let mut offset = 0u16;

    for button in buttons.iter_mut().rev() {
        let (button_span, area) = button.render_at_offset(panel_area, offset);
        (*button).set_area(area);

        let button_width = format!(" [{}] ", (*button).text()).len() as u16;
        offset += button_width;

        spans.insert(1, button_span);
    }

    Line::from(spans)
}
