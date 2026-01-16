//! Render code block header, content, and border.

use super::super::{get_language_icon, CodeBlockBorderKind, StyledLine};
use ratatui::style::{Color, Style};
use ratatui::text::{Line, Span};

pub fn render_header(_styled_line: &StyledLine, language: &str, width: usize) -> Line<'static> {
    let icon = get_language_icon(language);
    let lang_display = if language.is_empty() {
        "text"
    } else {
        language
    };
    let header_content = format!("{}{}", icon, lang_display);
    let header_len = header_content.chars().count();

    let inner_width = width.saturating_sub(4);
    let padding = if header_len < inner_width {
        " ".repeat(inner_width.saturating_sub(header_len))
    } else {
        String::new()
    };

    let style = Style::default()
        .fg(Color::Yellow)
        .bg(Color::Rgb(40, 40, 40));
    Line::from(vec![
        Span::styled("│ ", Style::default().fg(Color::DarkGray)),
        Span::styled(format!("{}{}", header_content, padding), style),
        Span::styled(" │", Style::default().fg(Color::DarkGray)),
    ])
}

pub fn render_content(
    _styled_line: &StyledLine,
    content: &str,
    highlighted: Option<&ratatui::text::Text<'static>>,
    width: usize,
) -> Line<'static> {
    let inner_width = width.saturating_sub(4);
    let bg_color = Color::Rgb(30, 30, 30);

    if let Some(highlighted_text) = highlighted {
        let spans: Vec<Span<'static>> = highlighted_text
            .lines
            .iter()
            .flat_map(|line| line.spans.clone())
            .collect();

        let total_width: usize = spans.iter().map(|s| s.content.chars().count()).sum();
        let padding = inner_width.saturating_sub(total_width);

        let border_style = Style::default().fg(Color::DarkGray);
        let bg_style = Style::default().bg(bg_color);

        let content_spans: Vec<Span<'static>> = spans
            .into_iter()
            .map(|mut span| {
                span.style = span.style.bg(bg_color);
                span
            })
            .collect();

        let mut all_spans = vec![Span::styled("│ ", border_style)];

        all_spans.extend(content_spans);

        if padding > 0 {
            all_spans.push(Span::styled(" ".repeat(padding), bg_style));
        }

        all_spans.push(Span::styled(" │", border_style));

        Line::from(all_spans)
    } else {
        let inner_width = width.saturating_sub(4);
        let padded = if content.chars().count() < inner_width {
            format!(
                "{}{}",
                content,
                " ".repeat(inner_width.saturating_sub(content.chars().count()))
            )
        } else {
            content.chars().take(inner_width).collect()
        };

        let style = Style::default().fg(Color::Green).bg(bg_color);
        Line::from(vec![
            Span::styled("│ ", Style::default().fg(Color::DarkGray)),
            Span::styled(padded, style),
            Span::styled(" │", Style::default().fg(Color::DarkGray)),
        ])
    }
}

pub fn render_border(
    _styled_line: &StyledLine,
    kind: &CodeBlockBorderKind,
    width: usize,
) -> Line<'static> {
    let inner_width = width.saturating_sub(2);
    let border_style = Style::default().fg(Color::DarkGray);

    let content = match kind {
        CodeBlockBorderKind::Top => {
            format!("┌{}┐", "─".repeat(inner_width))
        }
        CodeBlockBorderKind::HeaderSeparator => {
            format!("├{}┤", "─".repeat(inner_width))
        }
        CodeBlockBorderKind::Bottom => {
            format!("└{}┘", "─".repeat(inner_width))
        }
    };

    Line::from(Span::styled(content, border_style))
}
