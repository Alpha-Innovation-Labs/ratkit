//! Render horizontal rule.

use super::super::{MarkdownElement, HORIZONTAL_RULE_CHAR};
use ratatui::style::{Color, Style};
use ratatui::text::{Line, Span};

pub fn render(_element: &MarkdownElement, width: usize, app_theme: Option<&crate::theme::AppTheme>) -> Line<'static> {
    let rule = HORIZONTAL_RULE_CHAR.to_string().repeat(width);

    // Use theme color if available, otherwise fallback to default
    let hr_color = app_theme
        .map(|t| t.markdown.horizontal_rule)
        .unwrap_or(Color::Rgb(100, 100, 100));

    Line::from(Span::styled(
        rule,
        Style::default().fg(hr_color),
    ))
}
