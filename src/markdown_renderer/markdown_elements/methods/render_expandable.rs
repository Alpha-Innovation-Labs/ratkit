//! Render expandable content blocks.

use super::super::MarkdownElement;
use super::render::render;
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};

pub fn render_expandable(
    _element: &MarkdownElement,
    _content_id: &str,
    lines: &[MarkdownElement],
    max_lines: usize,
    collapsed: bool,
    total_lines: usize,
    width: usize,
) -> Vec<Line<'static>> {
    let mut result = Vec::new();

    if collapsed {
        let visible_lines = lines.iter().take(max_lines);
        for line in visible_lines {
            let rendered = render(line, width);
            result.extend(rendered);
        }

        let hidden_count = total_lines.saturating_sub(max_lines);
        let toggle_text = format!("▼ Show more ({} hidden) ", hidden_count);
        let toggle_style = Style::default()
            .fg(Color::Blue)
            .add_modifier(Modifier::UNDERLINED);
        result.push(Line::from(vec![Span::styled(toggle_text, toggle_style)]));
    } else {
        for line in lines {
            let rendered = render(line, width);
            result.extend(rendered);
        }

        let toggle_text = "▲ Show less ";
        let toggle_style = Style::default()
            .fg(Color::Blue)
            .add_modifier(Modifier::UNDERLINED);
        result.push(Line::from(vec![Span::styled(toggle_text, toggle_style)]));
    }

    result
}

pub fn render_expand_toggle(
    _element: &MarkdownElement,
    _content_id: &str,
    expanded: bool,
    hidden_count: usize,
    _width: usize,
) -> Vec<Line<'static>> {
    let toggle_text = if expanded {
        "▲ Show less ".to_string()
    } else {
        format!("▼ Show more ({} hidden) ", hidden_count)
    };

    let toggle_style = Style::default()
        .fg(Color::Blue)
        .add_modifier(Modifier::UNDERLINED);

    vec![Line::from(vec![Span::styled(toggle_text, toggle_style)])]
}
