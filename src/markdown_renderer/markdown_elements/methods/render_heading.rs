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
) -> Vec<Line<'static>> {
    let icon = HEADING_ICONS
        .get(level.saturating_sub(1) as usize)
        .unwrap_or(&"# ");
    let bg = heading_bg_color(level);
    let fg = heading_fg_color(level);

    // Level-based indentation: (level - 1) * 2 spaces
    let indent = "  ".repeat(level.saturating_sub(1) as usize);

    // Collapse indicator
    let collapse_indicator = if collapsed { "▶" } else { "▼" };

    let mut spans = vec![
        // Indentation (no background)
        Span::raw(indent),
        // Collapse indicator with heading style
        Span::styled(
            collapse_indicator.to_string(),
            Style::default().fg(fg).bg(bg),
        ),
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

pub fn render_border(_element: &MarkdownElement, level: u8, width: usize) -> Line<'static> {
    let bg = heading_bg_color(level);
    let border = "▀".repeat(width);
    Line::from(Span::styled(border, Style::default().fg(bg)))
}
