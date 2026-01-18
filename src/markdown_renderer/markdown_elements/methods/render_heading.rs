//! Render heading and heading border.

use super::super::constants::{heading_bg_color, heading_fg_color};
use super::super::MarkdownElement;
use super::super::HEADING_ICONS;
use super::helpers::render_text_segment;
use ratatui::style::{Modifier, Style};
use ratatui::text::{Line, Span};

pub fn render(
    _element: &MarkdownElement,
    level: u8,
    text: &[super::super::TextSegment],
    collapsed: bool,
    width: usize,
    app_theme: Option<&crate::theme::AppTheme>,
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

    // Collapse indicator at the start
    let collapse_indicator = if collapsed { "▶" } else { "▼" };

    let mut spans = vec![
        // Collapse indicator at the very start (with bg)
        Span::styled(
            collapse_indicator.to_string(),
            Style::default().fg(fg).bg(bg),
        ),
        // Indentation with background
        Span::styled(indent, Style::default().bg(bg)),
        // Icon with heading style
        Span::styled(
            icon.to_string(),
            Style::default().fg(fg).bg(bg).add_modifier(Modifier::BOLD),
        ),
    ];

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

pub fn render_border(_element: &MarkdownElement, level: u8, width: usize, _app_theme: Option<&crate::theme::AppTheme>) -> Line<'static> {
    // Border always uses the level-based background color for visual hierarchy
    let bg = heading_bg_color(level);
    let border = "▀".repeat(width);
    Line::from(Span::styled(border, Style::default().fg(bg)))
}
