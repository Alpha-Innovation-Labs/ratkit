//! Core rendering functions for markdown content.

use super::elements::render;
use super::parser::render_markdown_to_elements;
use crate::markdown_renderer::MarkdownStyle;

/// Render markdown string to ratatui Text with default styling
///
/// # Arguments
/// * `markdown` - The markdown string to render
/// * `max_width` - Optional maximum width for full-width backgrounds (defaults to 120)
pub fn render_markdown(markdown: &str, max_width: Option<usize>) -> ratatui::text::Text<'static> {
    let width = max_width.unwrap_or(120);
    let elements = render_markdown_to_elements(markdown, true);

    let mut lines = Vec::new();
    for element in elements {
        lines.extend(render(&element, width));
    }

    ratatui::text::Text::from(lines)
}

/// Render markdown string to ratatui Text with custom style configuration
///
/// # Arguments
/// * `markdown` - The markdown string to render
/// * `style` - Custom style configuration (currently unused, kept for API compatibility)
/// * `max_width` - Optional maximum width for full-width backgrounds (defaults to 120)
pub fn render_markdown_with_style(
    markdown: &str,
    _style: MarkdownStyle,
    max_width: Option<usize>,
) -> ratatui::text::Text<'static> {
    render_markdown(markdown, max_width)
}
