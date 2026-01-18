//! Render paragraph text.

use super::super::{
    get_link_icon, CheckboxState, MarkdownElement, TextSegment, CHECKBOX_CHECKED, CHECKBOX_TODO,
    CHECKBOX_UNCHECKED,
};
use super::helpers::wrap_text;
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};

pub fn render(
    _element: &MarkdownElement,
    segments: &[TextSegment],
    width: usize,
    app_theme: Option<&crate::theme::AppTheme>,
) -> Vec<Line<'static>> {
    let plain_text = segments_to_plain_text(segments);
    let wrapped = wrap_text(&plain_text, width);

    wrapped
        .into_iter()
        .map(|line_text| {
            let spans = render_line_with_segments(&line_text, segments, app_theme);
            Line::from(spans)
        })
        .collect()
}

/// Render a single wrapped line, preserving styling from segments
fn render_line_with_segments(line_text: &str, segments: &[TextSegment], app_theme: Option<&crate::theme::AppTheme>) -> Vec<Span<'static>> {
    if line_text.is_empty() {
        return vec![Span::raw("")];
    }

    // Build a character-to-style map from segments
    let full_text = segments_to_plain_text(segments);

    // Find where this line starts in the full text
    let line_start = full_text.find(line_text).unwrap_or(0);
    let line_end = line_start + line_text.len();

    // Build spans for just this line's portion
    let mut spans = Vec::new();
    let mut char_pos = 0;

    // Get theme colors with fallbacks
    let code_color = app_theme
        .map(|t| t.markdown.code)
        .unwrap_or(Color::Rgb(230, 180, 100));
    let link_color = app_theme
        .map(|t| t.markdown.link_text)
        .unwrap_or(Color::Rgb(100, 200, 100));
    let emph_color = app_theme
        .map(|t| t.markdown.emph)
        .unwrap_or(Color::Reset);
    let strong_color = app_theme
        .map(|t| t.markdown.strong)
        .unwrap_or(Color::Reset);

    for segment in segments {
        let (text, style) = match segment {
            TextSegment::Plain(t) => (t.clone(), Style::default()),
            TextSegment::Bold(t) => (
                t.clone(),
                Style::default()
                    .fg(strong_color)
                    .add_modifier(Modifier::BOLD),
            ),
            TextSegment::Italic(t) => (
                t.clone(),
                Style::default()
                    .fg(emph_color)
                    .add_modifier(Modifier::ITALIC),
            ),
            TextSegment::BoldItalic(t) => (
                t.clone(),
                Style::default()
                    .fg(strong_color)
                    .add_modifier(Modifier::BOLD)
                    .add_modifier(Modifier::ITALIC),
            ),
            TextSegment::InlineCode(t) => (
                t.clone(),
                Style::default()
                    .bg(Color::Rgb(60, 60, 60))
                    .fg(code_color),
            ),
            TextSegment::Link {
                text,
                url,
                is_autolink,
                bold,
                italic,
                show_icon,
            } => {
                // Only show icon for first segment of a link
                let full_text = if *show_icon {
                    let icon = get_link_icon(url);
                    format!("{}{}", icon, text)
                } else {
                    text.clone()
                };

                let mut style = if *is_autolink {
                    // Autolinks: italic blue underlined
                    Style::default()
                        .fg(Color::Rgb(100, 150, 255))
                        .add_modifier(Modifier::ITALIC)
                        .add_modifier(Modifier::UNDERLINED)
                } else {
                    // Regular links: use theme color
                    Style::default().fg(link_color)
                };

                // Add bold/italic modifiers if present
                if *bold {
                    style = style.add_modifier(Modifier::BOLD);
                }
                if *italic && !*is_autolink {
                    // Only add italic if not autolink (autolinks are already italic)
                    style = style.add_modifier(Modifier::ITALIC);
                }

                (full_text, style)
            }
            TextSegment::Strikethrough(t) => (
                t.clone(),
                Style::default()
                    .fg(Color::Rgb(150, 150, 150))
                    .add_modifier(Modifier::CROSSED_OUT),
            ),
            TextSegment::Html(t) => (t.clone(), Style::default()),
            TextSegment::Checkbox(state) => {
                let (icon, color) = match state {
                    CheckboxState::Unchecked => (CHECKBOX_UNCHECKED, Color::Rgb(150, 150, 150)),
                    CheckboxState::Checked => (CHECKBOX_CHECKED, Color::Rgb(100, 200, 100)),
                    CheckboxState::Todo => (CHECKBOX_TODO, Color::Rgb(200, 150, 50)),
                };
                (format!("{} ", icon), Style::default().fg(color))
            }
        };

        let seg_start = char_pos;
        let seg_end = char_pos + text.len();
        char_pos = seg_end;

        // Check if this segment overlaps with our line
        if seg_end <= line_start || seg_start >= line_end {
            continue; // No overlap
        }

        // Calculate the overlap
        let overlap_start = seg_start.max(line_start);
        let overlap_end = seg_end.min(line_end);

        // Extract the portion of this segment that's in our line
        let local_start = overlap_start - seg_start;
        let local_end = overlap_end - seg_start;

        if local_start < text.len() && local_end <= text.len() {
            let slice = &text[local_start..local_end];
            if !slice.is_empty() {
                spans.push(Span::styled(slice.to_string(), style));
            }
        }
    }

    if spans.is_empty() {
        vec![Span::raw(line_text.to_string())]
    } else {
        spans
    }
}

fn segments_to_plain_text(segments: &[TextSegment]) -> String {
    segments
        .iter()
        .map(|seg| match seg {
            TextSegment::Plain(text) => text.clone(),
            TextSegment::Bold(text) => text.clone(),
            TextSegment::Italic(text) => text.clone(),
            TextSegment::BoldItalic(text) => text.clone(),
            TextSegment::InlineCode(text) => text.clone(),
            TextSegment::Link {
                text,
                url,
                show_icon,
                ..
            } => {
                // Only include icon if show_icon is true (first segment of link)
                if *show_icon {
                    let icon = get_link_icon(url);
                    format!("{}{}", icon, text)
                } else {
                    text.clone()
                }
            }
            TextSegment::Strikethrough(text) => text.clone(),
            TextSegment::Html(content) => content.clone(),
            TextSegment::Checkbox(_) => String::new(), // Checkbox handled separately
        })
        .collect::<Vec<_>>()
        .join("")
}
