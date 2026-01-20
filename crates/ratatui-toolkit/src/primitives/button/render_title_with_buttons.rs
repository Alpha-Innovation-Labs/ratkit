use ratatui::layout::Rect;
use ratatui::text::Line;

use crate::primitives::button::Button;

/// Renders a title line with buttons aligned to the right.
///
/// This function creates a line where the title appears first, followed by
/// buttons rendered in reverse order so the first button in the list appears
/// rightmost in the UI.
///
/// # Arguments
///
/// * `panel_area` - The rectangular area available for rendering
/// * `title` - The title text to display
/// * `buttons` - Mutable slice of buttons to render (order determines visual position)
///
/// # Returns
///
/// A `Line` containing the title and button spans
///
/// # Example
///
/// ```rust
/// use ratatui::layout::Rect;
/// use ratatui_toolkit::{Button, render_title_with_buttons};
///
/// let panel_area = Rect::new(0, 0, 80, 1);
/// let mut btn1 = Button::new("Save");
/// let mut btn2 = Button::new("Cancel");
/// let mut buttons: Vec<&mut Button> = vec![&mut btn1, &mut btn2];
/// let line = render_title_with_buttons(panel_area, "My Panel", &mut buttons);
/// ```
pub fn render_title_with_buttons(
    panel_area: Rect,
    title: &str,
    buttons: &mut [&mut Button],
) -> Line<'static> {
    use ratatui::text::Span;

    let mut spans = vec![Span::raw(title.to_string())];

    let mut offset = 0u16;

    for button in buttons.iter_mut().rev() {
        let (button_span, area) = button.render_at_offset(panel_area, offset);
        button.set_area(area);

        let button_width = format!(" [{}] ", button.text()).len() as u16;
        offset += button_width;

        spans.insert(1, button_span);
    }

    Line::from(spans)
}
