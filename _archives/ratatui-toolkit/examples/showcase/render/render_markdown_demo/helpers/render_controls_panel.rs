//! Render the controls/info panel.

use ratatui::{
    layout::Rect,
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Paragraph},
};

/// Render the controls/info panel on the right side.
///
/// # Arguments
///
/// * `frame` - The frame to render into.
/// * `area` - The area for the panel.
/// * `border_style` - The style for the border.
pub fn render_controls_panel(frame: &mut ratatui::Frame, area: Rect, border_style: Style) {
    let info = Paragraph::new(vec![
        Line::from(""),
        Line::from(Span::styled(
            "  Markdown Features",
            Style::default().add_modifier(Modifier::BOLD),
        )),
        Line::from(""),
        Line::from("  Navigation:"),
        Line::from("    j/↓  Scroll down"),
        Line::from("    k/↑  Scroll up"),
        Line::from("    PgDn/PgUp Page scroll"),
        Line::from("    Home/End  Top/Bottom"),
        Line::from(""),
        Line::from("  Selection:"),
        Line::from("    Drag   Select text"),
        Line::from("    y      Copy selection"),
        Line::from("    Ctrl+Shift+C  Copy"),
        Line::from("    Esc    Exit selection"),
        Line::from(""),
        Line::from("  Interactions:"),
        Line::from("    • Click headers to collapse"),
        Line::from("    • Mouse wheel to scroll"),
        Line::from("    • Drag divider to resize"),
        Line::from(""),
        Line::from("  Rendering:"),
        Line::from("    • Syntax highlighting"),
        Line::from("    • Line numbers in code"),
        Line::from("    • Tables, links, blockquotes"),
    ])
    .block(
        Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .title(" Controls ")
            .border_style(border_style),
    );

    frame.render_widget(info, area);
}
