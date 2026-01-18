//! Render the theme picker popup.

use ratatui::{
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Paragraph},
};

use crate::app::App;
use crate::helpers::{all_themes, get_theme_name};

/// Render the theme picker popup.
pub fn render_theme_picker(frame: &mut ratatui::Frame, app: &App) {
    let area = frame.area();
    let themes = all_themes();

    // Calculate popup size
    let popup_width = 30u16;
    let popup_height = (themes.len() + 2) as u16; // +2 for borders

    // Center the popup
    let popup_area = Rect {
        x: area.width.saturating_sub(popup_width) / 2,
        y: area.height.saturating_sub(popup_height) / 2,
        width: popup_width.min(area.width),
        height: popup_height.min(area.height),
    };

    // Clear the popup area
    frame.render_widget(ratatui::widgets::Clear, popup_area);

    // Build theme list
    let items: Vec<Line> = themes
        .iter()
        .enumerate()
        .map(|(i, theme)| {
            let name = get_theme_name(*theme);
            let is_selected = i == app.theme_picker_index;
            let is_current = *theme == app.markdown_scroll.code_block_theme;

            let prefix = if is_selected { "▶ " } else { "  " };
            let suffix = if is_current { " ✓" } else { "" };

            let style = if is_selected {
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD)
            } else if is_current {
                Style::default().fg(Color::Green)
            } else {
                Style::default()
            };

            Line::from(Span::styled(format!("{}{}{}", prefix, name, suffix), style))
        })
        .collect();

    let popup = Paragraph::new(items).block(
        Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Style::default().fg(Color::Cyan))
            .title(" Select Theme (j/k, Enter) "),
    );

    frame.render_widget(popup, popup_area);
}
