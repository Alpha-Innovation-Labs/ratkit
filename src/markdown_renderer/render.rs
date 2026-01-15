use pulldown_cmark::Parser;
use ratatui::text::Text;

use super::markdown_style::MarkdownStyle;
use super::parser::MarkdownParser;

/// Render markdown string to ratatui Text with custom styling
///
/// # Arguments
/// * `markdown` - The markdown string to render
/// * `max_width` - Optional maximum width for full-width backgrounds (defaults to 120)
pub fn render_markdown(markdown: &str, max_width: Option<usize>) -> Text<'static> {
    render_markdown_with_style(markdown, MarkdownStyle::default(), max_width)
}

/// Render markdown string to ratatui Text with custom style configuration
///
/// # Arguments
/// * `markdown` - The markdown string to render
/// * `style` - Custom style configuration
/// * `max_width` - Optional maximum width for full-width backgrounds (defaults to 120)
pub fn render_markdown_with_style(
    markdown: &str,
    style: MarkdownStyle,
    max_width: Option<usize>,
) -> Text<'static> {
    let parser = Parser::new(markdown);
    let width = max_width.unwrap_or(120);
    let mut md_parser = MarkdownParser::new(style, width);

    for event in parser {
        md_parser.process_event(event);
    }

    let lines = md_parser.finalize();
    Text::from(lines)
}
