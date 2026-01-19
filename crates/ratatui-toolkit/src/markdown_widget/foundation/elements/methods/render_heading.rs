//! Render heading and heading border.

use crate::markdown_widget::foundation::elements::constants::{
    heading_bg_color, heading_fg_color, HEADING_ICONS,
};
use crate::markdown_widget::foundation::elements::enums::TextSegment;
use crate::markdown_widget::foundation::elements::methods::helpers::render_text_segment;
use crate::markdown_widget::foundation::elements::MarkdownElement;
use ratatui::style::{Modifier, Style};
use ratatui::text::{Line, Span};

pub fn render(
    _element: &MarkdownElement,
    level: u8,
    text: &[TextSegment],
    collapsed: bool,
    width: usize,
    app_theme: Option<&crate::services::theme::AppTheme>,
    show_collapse_indicator: bool,
) -> Vec<Line<'static>> {
    let icon = HEADING_ICONS
        .get(level.saturating_sub(1) as usize)
        .unwrap_or(&"# ");

    // Always use level-specific colors for visual hierarchy
    // The original design has distinct colors per heading level
    let _ = app_theme; // Theme doesn't override heading colors - they're level-based
    let bg = heading_bg_color(level);
    let fg = heading_fg_color(level);

    // Level-based indentation: 1 base space + (level - 1) spaces
    // H1: 1 space, H2: 2 spaces, H3: 3 spaces, etc.
    let indent_count = level as usize;
    let indent = " ".repeat(indent_count);

    let mut spans = Vec::new();

    // Only show collapse indicator if enabled
    if show_collapse_indicator {
        let collapse_indicator = if collapsed { "\u{25b6}" } else { "\u{25bc}" };
        spans.push(Span::styled(
            collapse_indicator.to_string(),
            Style::default().fg(fg).bg(bg),
        ));
    }

    // Indentation with background
    spans.push(Span::styled(indent, Style::default().bg(bg)));
    // Icon with heading style
    spans.push(Span::styled(
        icon.to_string(),
        Style::default().fg(fg).bg(bg).add_modifier(Modifier::BOLD),
    ));

    for segment in text {
        spans.push(render_text_segment(segment, Style::default().fg(fg).bg(bg)));
    }

    let current_len: usize = spans.iter().map(|s| s.content.chars().count()).sum();
    if current_len < width {
        let padding = " ".repeat(width.saturating_sub(current_len));
        spans.push(Span::styled(padding, Style::default().bg(bg)));
    }

    vec![Line::from(spans)]
}

pub fn render_border(
    _element: &MarkdownElement,
    level: u8,
    width: usize,
    _app_theme: Option<&crate::services::theme::AppTheme>,
) -> Line<'static> {
    // Border always uses the level-based background color for visual hierarchy
    let bg = heading_bg_color(level);
    let border = "\u{2580}".repeat(width);
    Line::from(Span::styled(border, Style::default().fg(bg)))
}
