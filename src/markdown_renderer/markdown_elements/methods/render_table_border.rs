//! Render table border.

use super::super::{MarkdownElement, TableBorderKind};
use ratatui::style::{Color, Style};
use ratatui::text::{Line, Span};

pub fn render(_element: &MarkdownElement, kind: &TableBorderKind) -> Line<'static> {
    let border_style = Style::default().fg(Color::DarkGray);

    let content = match kind {
        TableBorderKind::Top(widths) => {
            let segments: Vec<String> = widths.iter().map(|w| "─".repeat(*w + 2)).collect();
            format!("┌{}┐", segments.join("┬"))
        }
        TableBorderKind::HeaderSeparator(widths) => {
            let segments: Vec<String> = widths.iter().map(|w| "─".repeat(*w + 2)).collect();
            format!("├{}┤", segments.join("┼"))
        }
        TableBorderKind::Bottom(widths) => {
            let segments: Vec<String> = widths.iter().map(|w| "─".repeat(*w + 2)).collect();
            format!("└{}┘", segments.join("┴"))
        }
    };

    Line::from(Span::styled(content, border_style))
}
