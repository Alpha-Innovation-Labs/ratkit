//! Render the dialogs demo tab.

use ratatui::{
    layout::Rect,
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Paragraph},
};
use ratatui_toolkit::AppTheme;

use crate::app::App;

/// Render the dialogs demo.
///
/// # Arguments
///
/// * `frame` - The frame to render into.
/// * `area` - The area to render in.
/// * `_app` - The application state.
/// * `theme` - The application theme.
pub fn render_dialogs_demo(
    frame: &mut ratatui::Frame,
    area: Rect,
    _app: &mut App,
    theme: &AppTheme,
) {
    let content = Paragraph::new(vec![
        Line::from(""),
        Line::from(Span::styled(
            "  Dialog Types",
            Style::default().add_modifier(Modifier::BOLD),
        )),
        Line::from(""),
        Line::from("  Press a key to show dialog:"),
        Line::from(""),
        Line::styled("    [i] Info Dialog", Style::default().fg(theme.info)),
        Line::styled("    [s] Success Dialog", Style::default().fg(theme.success)),
        Line::styled("    [w] Warning Dialog", Style::default().fg(theme.warning)),
        Line::styled("    [e] Error Dialog", Style::default().fg(theme.error)),
        Line::styled("    [c] Confirm Dialog", Style::default().fg(theme.primary)),
        Line::from(""),
        Line::from("  Dialog features:"),
        Line::from("    • Modal overlay with dimmed background"),
        Line::from("    • Customizable width/height"),
        Line::from("    • Multiple button support"),
        Line::from("    • Click detection on buttons"),
        Line::from("    • Tab navigation between buttons"),
    ])
    .block(
        Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .title(" Dialogs Demo "),
    );

    frame.render_widget(content, area);
}
