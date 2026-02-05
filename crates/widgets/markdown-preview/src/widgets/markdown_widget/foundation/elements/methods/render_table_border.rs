//! Render table border.

use crate::widgets::markdown_widget::foundation::elements::enums::TableBorderKind;
use crate::widgets::markdown_widget::foundation::elements::MarkdownElement;
use ratatui::style::{Color, Style};
use ratatui::text::{Line, Span};

pub fn render(_element: &MarkdownElement, kind: &TableBorderKind) -> Line<'static> {
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
