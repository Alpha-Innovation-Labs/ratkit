//! Render list item.

use crate::markdown_widget::foundation::elements::constants::{
    BULLET_MARKERS, CHECKBOX_CHECKED, CHECKBOX_TODO, CHECKBOX_UNCHECKED,
};
use crate::markdown_widget::foundation::elements::enums::{CheckboxState, TextSegment};
use crate::markdown_widget::foundation::elements::methods::helpers::{
    render_text_segment, segments_to_plain_text, wrap_text,
};
use crate::markdown_widget::foundation::elements::MarkdownElement;
use ratatui::style::{Color, Style};
use ratatui::text::{Line, Span};

pub fn render(
    _element: &MarkdownElement,
    depth: usize,
    ordered: bool,
    number: Option<usize>,
    content: &[TextSegment],
    width: usize,
    app_theme: Option<&crate::services::theme::AppTheme>,
) -> Vec<Line<'static>> {
    let indent = "  ".repeat(depth);

    // Check if first segment is a checkbox
    let (checkbox, remaining_content) = match content.first() {
        Some(TextSegment::Checkbox(state)) => (Some(*state), &content[1..]),
        _ => (None, content),
    };

    // Use theme color for bullets or fall back to yellow
    let bullet_color = app_theme
        .map(|t| t.markdown.list_item)
        .unwrap_or(Color::Yellow);

    let marker = if ordered {
        format!("{}. ", number.unwrap_or(1))
    } else {
        let marker_idx = depth % BULLET_MARKERS.len();
        BULLET_MARKERS[marker_idx].to_string()
    };

    // Build the checkbox span if present
    let checkbox_span = checkbox.map(|state| {
        let (icon, color) = match state {
            CheckboxState::Unchecked => (CHECKBOX_UNCHECKED, Color::Rgb(180, 180, 180)),
            CheckboxState::Checked => (CHECKBOX_CHECKED, Color::Rgb(100, 200, 100)),
            CheckboxState::Todo => (CHECKBOX_TODO, Color::Rgb(255, 200, 100)),
        };
        Span::styled(icon.to_string(), Style::default().fg(color))
    });

    let prefix = format!("{}{}", indent, marker);
    let checkbox_len = if checkbox.is_some() { 2 } else { 0 }; // icon + space
    let prefix_len = prefix.chars().count() + checkbox_len;
    let content_width = width.saturating_sub(prefix_len);

    let text = segments_to_plain_text(remaining_content);
    let wrapped = wrap_text(&text, content_width);

    let mut lines = Vec::new();
    for (i, line_text) in wrapped.into_iter().enumerate() {
        if i == 0 {
            let mut spans = vec![
                Span::styled(indent.clone(), Style::default()),
                Span::styled(marker.clone(), Style::default().fg(bullet_color)),
            ];
            if let Some(ref cb_span) = checkbox_span {
                spans.push(cb_span.clone());
            }
            // Render styled segments for the first line
            for segment in remaining_content {
                spans.push(render_text_segment(segment, Style::default()));
            }
            lines.push(Line::from(spans));
        } else {
            let continuation_indent = " ".repeat(prefix_len);
            lines.push(Line::from(vec![
                Span::styled(continuation_indent, Style::default()),
                Span::styled(line_text, Style::default()),
            ]));
        }
    }

    if lines.is_empty() {
        let mut spans = vec![
            Span::styled(indent, Style::default()),
            Span::styled(marker, Style::default().fg(bullet_color)),
        ];
        if let Some(cb_span) = checkbox_span {
            spans.push(cb_span);
        }
        lines.push(Line::from(spans));
    }

    lines
}
