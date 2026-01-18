//! Text processing helper functions for markdown rendering.

use super::super::{
    get_link_icon, CheckboxState, TextSegment, CHECKBOX_CHECKED, CHECKBOX_TODO,
    CHECKBOX_UNCHECKED,
};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::Span;

pub fn render_text_segment(segment: &TextSegment, base_style: Style) -> Span<'static> {
    match segment {
        TextSegment::Plain(text) => Span::styled(text.clone(), base_style),
        TextSegment::Bold(text) => {
            Span::styled(text.clone(), base_style.add_modifier(Modifier::BOLD))
        }
        TextSegment::Italic(text) => {
            Span::styled(text.clone(), base_style.add_modifier(Modifier::ITALIC))
        }
        TextSegment::BoldItalic(text) => Span::styled(
            text.clone(),
            base_style
                .add_modifier(Modifier::BOLD)
                .add_modifier(Modifier::ITALIC),
        ),
        TextSegment::InlineCode(text) => Span::styled(
            format!(" {} ", text),
            base_style
                .bg(Color::Rgb(60, 60, 60))
                .fg(Color::Rgb(230, 180, 100)),
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
                base_style
                    .fg(Color::Rgb(100, 150, 255))
                    .add_modifier(Modifier::ITALIC)
                    .add_modifier(Modifier::UNDERLINED)
            } else {
                // Regular links: green
                base_style.fg(Color::Rgb(100, 200, 100))
            };

            // Add bold/italic modifiers if present
            if *bold {
                style = style.add_modifier(Modifier::BOLD);
            }
            if *italic && !*is_autolink {
                style = style.add_modifier(Modifier::ITALIC);
            }

            Span::styled(full_text, style)
        }
        TextSegment::Strikethrough(text) => Span::styled(
            text.clone(),
            base_style
                .fg(Color::Rgb(150, 150, 150))
                .add_modifier(Modifier::CROSSED_OUT),
        ),
        TextSegment::Html(text) => Span::styled(
            text.clone(),
            base_style
                .fg(Color::Rgb(100, 180, 100))
                .add_modifier(Modifier::ITALIC),
        ),
        TextSegment::Checkbox(state) => {
            let (icon, color) = match state {
                CheckboxState::Unchecked => (CHECKBOX_UNCHECKED, Color::Rgb(180, 180, 180)),
                CheckboxState::Checked => (CHECKBOX_CHECKED, Color::Rgb(100, 200, 100)),
                CheckboxState::Todo => (CHECKBOX_TODO, Color::Rgb(255, 200, 100)),
            };
            Span::styled(icon.to_string(), base_style.fg(color))
        }
    }
}

pub fn segments_to_plain_text(segments: &[TextSegment]) -> String {
    segments
        .iter()
        .map(|seg| match seg {
            TextSegment::Plain(text) => text.clone(),
            TextSegment::Bold(text) => text.clone(),
            TextSegment::Italic(text) => text.clone(),
            TextSegment::BoldItalic(text) => text.clone(),
            TextSegment::InlineCode(text) => format!("`{}`", text),
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
            TextSegment::Html(text) => text.clone(),
            TextSegment::Checkbox(_) => String::new(), // Checkbox icon handled separately
        })
        .collect::<Vec<_>>()
        .join("")
}

pub fn wrap_text(text: &str, width: usize) -> Vec<String> {
    if width == 0 || text.is_empty() {
        return vec![text.to_string()];
    }

    let mut lines = Vec::new();
    let mut current_line = String::new();
    let mut current_width = 0;

    for word in text.split_whitespace() {
        let word_width = word.chars().count();

        if current_width == 0 {
            current_line = word.to_string();
            current_width = word_width;
        } else if current_width + 1 + word_width <= width {
            current_line.push(' ');
            current_line.push_str(word);
            current_width += 1 + word_width;
        } else {
            lines.push(current_line);
            current_line = word.to_string();
            current_width = word_width;
        }
    }

    if !current_line.is_empty() {
        lines.push(current_line);
    }

    if lines.is_empty() {
        lines.push(String::new());
    }

    lines
}
