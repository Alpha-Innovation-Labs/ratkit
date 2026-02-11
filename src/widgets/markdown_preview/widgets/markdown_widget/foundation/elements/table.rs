use crate::widgets::markdown_preview::widgets::markdown_widget::foundation::elements::enums::TableBorderKind;
/// Render table border.
use crate::widgets::markdown_preview::widgets::markdown_widget::foundation::elements::MarkdownElement;
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};

pub fn render_table_border(_element: &MarkdownElement, kind: &TableBorderKind) -> Line<'static> {
    let border_style = Style::default().fg(Color::DarkGray);

    let content = match kind {
        TableBorderKind::Top(widths) => {
            let segments: Vec<String> = widths.iter().map(|w| "\u{2500}".repeat(*w + 2)).collect();
            format!("\u{250c}{}\u{2510}", segments.join("\u{252c}"))
        }
        TableBorderKind::HeaderSeparator(widths) => {
            let segments: Vec<String> = widths.iter().map(|w| "\u{2500}".repeat(*w + 2)).collect();
            format!("\u{251c}{}\u{2524}", segments.join("\u{253c}"))
        }
        TableBorderKind::Bottom(widths) => {
            let segments: Vec<String> = widths.iter().map(|w| "\u{2500}".repeat(*w + 2)).collect();
            format!("\u{2514}{}\u{2518}", segments.join("\u{2534}"))
        }
    };

    Line::from(Span::styled(content, border_style))
}

/// Render table row.
pub fn render_table_row(
    _element: &MarkdownElement,
    cells: &[String],
    is_header: bool,
) -> Line<'static> {
    let style = if is_header {
        Style::default()
            .fg(Color::Cyan)
            .add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(Color::White)
    };

    let mut spans = Vec::new();
    spans.push(Span::styled(
        "\u{2502}",
        Style::default().fg(Color::DarkGray),
    ));

    for cell in cells {
        spans.push(Span::styled(format!(" {} ", cell), style));
        spans.push(Span::styled(
            "\u{2502}",
            Style::default().fg(Color::DarkGray),
        ));
    }

    Line::from(spans)
}
