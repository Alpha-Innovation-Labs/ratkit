//! Render table row.

use crate::markdown_widget::foundation::elements::MarkdownElement;
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};

pub fn render(_element: &MarkdownElement, cells: &[String], is_header: bool) -> Line<'static> {
    let style = if is_header {
        Style::default()
            .fg(Color::Cyan)
            .add_modifier(Modifier::BOLD)
    } else {
        Style::default()
    };

    let mut spans = vec![Span::styled("\u{2502} ", Style::default().fg(Color::DarkGray))];

    for (i, cell) in cells.iter().enumerate() {
        if i > 0 {
            spans.push(Span::styled(" \u{2502} ", Style::default().fg(Color::DarkGray)));
        }
        spans.push(Span::styled(cell.clone(), style));
    }

    spans.push(Span::styled(" \u{2502}", Style::default().fg(Color::DarkGray)));

    Line::from(spans)
}
