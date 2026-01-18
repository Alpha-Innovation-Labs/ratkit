//! Render the scrollbar demo tab.

use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    text::Line,
    widgets::{Block, BorderType, Borders, Paragraph},
};
use ratatui_toolkit::{ClickableScrollbar, ClickableScrollbarStateScrollExt};

use crate::app::App;

/// Render the scrollbar demo.
pub fn render_scrollbar_demo(frame: &mut ratatui::Frame, area: Rect, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Min(0), Constraint::Length(1)])
        .split(area);

    // Update page length based on visible area
    let visible_height = chunks[0].height.saturating_sub(2) as usize;
    app.scrollbar_state = app
        .scrollbar_state
        .clone()
        .set_content(app.scroll_content.len(), visible_height);

    // Content
    let visible_lines: Vec<Line> = app
        .scroll_content
        .iter()
        .skip(app.scrollbar_state.offset())
        .take(visible_height)
        .map(|s| Line::from(s.as_str()))
        .collect();

    let content = Paragraph::new(visible_lines).block(
        Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .title(format!(
                " ClickableScrollbar - Line {}/{} ",
                app.scrollbar_state.offset() + 1,
                app.scroll_content.len()
            )),
    );

    frame.render_widget(content, chunks[0]);

    // Scrollbar
    let scrollbar = ClickableScrollbar::vertical();
    frame.render_stateful_widget(scrollbar, chunks[1], &mut app.scrollbar_state);
}
