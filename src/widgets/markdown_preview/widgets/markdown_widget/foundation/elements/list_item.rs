//! Render list item.

use crate::widgets::markdown_preview::widgets::markdown_widget::foundation::elements::constants::{
    BULLET_MARKERS, CHECKBOX_CHECKED, CHECKBOX_TODO, CHECKBOX_UNCHECKED,
};
use crate::widgets::markdown_preview::widgets::markdown_widget::foundation::elements::enums::{
    CheckboxState, TextSegment,
};
use crate::widgets::markdown_preview::widgets::markdown_widget::foundation::elements::MarkdownElement;
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use unicode_width::UnicodeWidthChar;

pub fn render(
    _element: &MarkdownElement,
    depth: usize,
    ordered: bool,
    number: Option<usize>,
    content: &[TextSegment],
    width: usize,
    app_theme: Option<&crate::widgets::markdown_preview::services::theme::AppTheme>,
) -> Vec<Line<'static>> {
    let indent = "  ".repeat(depth);

    let (checkbox, remaining_content) = match content.first() {
        Some(TextSegment::Checkbox(state)) => (Some(*state), &content[1..]),
        _ => (None, content),
    };

    let bullet_color = app_theme
        .map(|t| t.markdown.list_item)
        .unwrap_or(Color::Yellow);

    let marker = if ordered {
        format!("{}. ", number.unwrap_or(1))
    } else {
        let marker_idx = depth % BULLET_MARKERS.len();
        BULLET_MARKERS[marker_idx].to_string()
    };

    let checkbox_span = checkbox.map(|state| {
        let (icon, color) = match state {
            CheckboxState::Unchecked => (CHECKBOX_UNCHECKED, Color::Rgb(180, 180, 180)),
            CheckboxState::Checked => (CHECKBOX_CHECKED, Color::Rgb(100, 200, 100)),
            CheckboxState::Todo => (CHECKBOX_TODO, Color::Rgb(255, 200, 100)),
        };
        Span::styled(icon.to_string(), Style::default().fg(color))
    });

    let prefix = format!("{}{}", indent, marker);
    let checkbox_len = if checkbox.is_some() { 2 } else { 0 };
    let prefix_len = prefix.chars().count() + checkbox_len;
    let content_width = width.saturating_sub(prefix_len).max(1);

    let rendered_segments: Vec<(String, Style)> = remaining_content
        .iter()
        .map(|segment| segment_text_and_style(segment, app_theme))
        .collect();

    let wrapped_lines = wrap_styled_segments(&rendered_segments, content_width);

    let mut lines = Vec::new();
    for (i, line_spans) in wrapped_lines.into_iter().enumerate() {
        if i == 0 {
            let mut spans = vec![
                Span::styled(indent.clone(), Style::default()),
                Span::styled(marker.clone(), Style::default().fg(bullet_color)),
            ];
            if let Some(ref cb_span) = checkbox_span {
                spans.push(cb_span.clone());
            }
            spans.extend(line_spans);
            lines.push(Line::from(spans));
        } else {
            let continuation_indent = " ".repeat(prefix_len);
            let mut spans = vec![Span::styled(continuation_indent, Style::default())];
            spans.extend(line_spans);
            lines.push(Line::from(spans));
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

fn segment_text_and_style(
    segment: &TextSegment,
    app_theme: Option<&crate::widgets::markdown_preview::services::theme::AppTheme>,
) -> (String, Style) {
    let code_color = app_theme
        .map(|t| t.markdown.code)
        .unwrap_or(Color::Rgb(230, 180, 100));
    let link_color = app_theme
        .map(|t| t.markdown.link_text)
        .unwrap_or(Color::Rgb(100, 200, 100));
    let emph_color = app_theme.map(|t| t.markdown.emph).unwrap_or(Color::Reset);
    let strong_color = app_theme.map(|t| t.markdown.strong).unwrap_or(Color::Reset);

    match segment {
        TextSegment::Plain(text) => (text.clone(), Style::default()),
        TextSegment::Bold(text) => (
            text.clone(),
            Style::default()
                .fg(strong_color)
                .add_modifier(Modifier::BOLD),
        ),
        TextSegment::Italic(text) => (
            text.clone(),
            Style::default()
                .fg(emph_color)
                .add_modifier(Modifier::ITALIC),
        ),
        TextSegment::BoldItalic(text) => (
            text.clone(),
            Style::default()
                .fg(strong_color)
                .add_modifier(Modifier::BOLD)
                .add_modifier(Modifier::ITALIC),
        ),
        TextSegment::InlineCode(text) => (
            format!(" {} ", text),
            Style::default().bg(Color::Rgb(60, 60, 60)).fg(code_color),
        ),
        TextSegment::Link {
            text,
            is_autolink,
            bold,
            italic,
            ..
        } => {
            let mut style = if *is_autolink {
                Style::default()
                    .fg(Color::Rgb(100, 150, 255))
                    .add_modifier(Modifier::ITALIC)
                    .add_modifier(Modifier::UNDERLINED)
            } else {
                Style::default().fg(link_color)
            };

            if *bold {
                style = style.add_modifier(Modifier::BOLD);
            }
            if *italic && !*is_autolink {
                style = style.add_modifier(Modifier::ITALIC);
            }

            (text.clone(), style)
        }
        TextSegment::Strikethrough(text) => (
            text.clone(),
            Style::default()
                .fg(Color::Rgb(150, 150, 150))
                .add_modifier(Modifier::CROSSED_OUT),
        ),
        TextSegment::Html(text) => (
            text.clone(),
            Style::default()
                .fg(Color::Rgb(100, 180, 100))
                .add_modifier(Modifier::ITALIC),
        ),
        TextSegment::Checkbox(state) => {
            let (icon, color) = match state {
                CheckboxState::Unchecked => (CHECKBOX_UNCHECKED, Color::Rgb(180, 180, 180)),
                CheckboxState::Checked => (CHECKBOX_CHECKED, Color::Rgb(100, 200, 100)),
                CheckboxState::Todo => (CHECKBOX_TODO, Color::Rgb(255, 200, 100)),
            };
            (format!("{} ", icon), Style::default().fg(color))
        }
    }
}

fn wrap_styled_segments(segments: &[(String, Style)], width: usize) -> Vec<Vec<Span<'static>>> {
    if width == 0 {
        return vec![vec![Span::raw("")]];
    }

    let mut lines: Vec<Vec<Span<'static>>> = Vec::new();
    let mut current_line: Vec<Span<'static>> = Vec::new();
    let mut current_width = 0usize;
    let mut run_text = String::new();
    let mut run_style: Option<Style> = None;

    fn flush_run(line: &mut Vec<Span<'static>>, text: &mut String, style: &mut Option<Style>) {
        if !text.is_empty() {
            if let Some(s) = *style {
                line.push(Span::styled(text.clone(), s));
            } else {
                line.push(Span::raw(text.clone()));
            }
            text.clear();
        }
    }

    fn flush_line(lines: &mut Vec<Vec<Span<'static>>>, line: &mut Vec<Span<'static>>) {
        if line.is_empty() {
            lines.push(vec![Span::raw("")]);
        } else {
            lines.push(std::mem::take(line));
        }
    }

    for (text, style) in segments {
        for ch in text.chars() {
            let ch_width = UnicodeWidthChar::width(ch).unwrap_or(0);

            if current_width > 0 && current_width + ch_width > width {
                flush_run(&mut current_line, &mut run_text, &mut run_style);
                flush_line(&mut lines, &mut current_line);
                current_width = 0;
            }

            if run_style != Some(*style) {
                flush_run(&mut current_line, &mut run_text, &mut run_style);
                run_style = Some(*style);
            }

            run_text.push(ch);
            current_width += ch_width;
        }
    }

    flush_run(&mut current_line, &mut run_text, &mut run_style);
    flush_line(&mut lines, &mut current_line);

    if lines.is_empty() {
        vec![vec![Span::raw("")]]
    } else {
        lines
    }
}
