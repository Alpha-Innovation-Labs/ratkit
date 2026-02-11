//! Functions module.

use ratatui::style::Style;
use ratatui::text::Text;

use super::elements::MarkdownElement;

pub fn render_markdown(content: &str) -> Text<'_> {
    Text::raw(content)
}

pub fn render_markdown_with_style(content: &str, style: Style) -> Text<'_> {
    Text::styled(content, style)
}
