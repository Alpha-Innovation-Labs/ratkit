//! Render a text segment with base style.

use super::super::{get_link_icon, TextSegment};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::Span;

/// Render a text segment with base style.
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
    }
}
