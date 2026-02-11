/// Render expandable content blocks.
use crate::widgets::markdown_preview::widgets::markdown_widget::foundation::elements::render::render;
use crate::widgets::markdown_preview::widgets::markdown_widget::foundation::elements::MarkdownElement;
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};

pub fn render_expandable(
    _element: &MarkdownElement,
    _content_id: &str,
    lines: &[MarkdownElement],
    max_lines: usize,
    collapsed: bool,
    width: usize,
    app_theme: Option<&crate::widgets::markdown_preview::services::theme::AppTheme>,
) -> Vec<Line<'static>> {
    let mut result = Vec::new();

    // Use theme color for toggle buttons or fall back to blue
    let toggle_color = app_theme.map(|t| t.info).unwrap_or(Color::Blue);

    if collapsed {
        let visible_lines = lines.iter().take(max_lines);
        for line in visible_lines {
            let rendered = render(line, width);
            result.extend(rendered);
        }

        let hidden_count = lines.len().saturating_sub(max_lines);
        let toggle_text = format!("\u{25bc} Show more ({} hidden) ", hidden_count);
        let toggle_style = Style::default()
            .fg(toggle_color)
            .add_modifier(Modifier::UNDERLINED);
        result.push(Line::from(vec![Span::styled(toggle_text, toggle_style)]));
    } else {
        for line in lines {
            let rendered = render(line, width);
            result.extend(rendered);
        }

        let toggle_text = "\u{25b2} Show less ";
        let toggle_style = Style::default()
            .fg(toggle_color)
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
    app_theme: Option<&crate::widgets::markdown_preview::services::theme::AppTheme>,
) -> Vec<Line<'static>> {
    let toggle_text = if expanded {
        "\u{25b2} Show less ".to_string()
    } else {
        format!("\u{25bc} Show more ({} hidden) ", hidden_count)
    };

    // Use theme color for toggle buttons or fall back to blue
    let toggle_color = app_theme.map(|t| t.info).unwrap_or(Color::Blue);

    let toggle_style = Style::default()
        .fg(toggle_color)
        .add_modifier(Modifier::UNDERLINED);

    vec![Line::from(vec![Span::styled(toggle_text, toggle_style)])]
}
