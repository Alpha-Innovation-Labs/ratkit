//! Render horizontal rule.

use super::super::{MarkdownElement, HORIZONTAL_RULE_CHAR};
use ratatui::style::{Color, Style};
use ratatui::text::{Line, Span};

pub fn render(_element: &MarkdownElement, width: usize) -> Line<'static> {
    let rule = HORIZONTAL_RULE_CHAR.to_string().repeat(width);
    // Color matching render-markdown.nvim's RenderMarkdownDash -> LineNr
    Line::from(Span::styled(rule, Style::default().fg(Color::Rgb(100, 100, 100))))
}
