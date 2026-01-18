//! Render markdown to visual lines with scroll state applied.

use ratatui::text::Line;

use crate::markdown_renderer::render_element;
use crate::markdown_renderer::scroll_manager::MarkdownScrollManager;

use super::helpers::should_render_line;

/// Render markdown to visual lines with scroll state applied.
///
/// # Arguments
///
/// * `content` - The markdown content
/// * `scroll` - The scroll manager
/// * `width` - The width to render to
///
/// # Returns
///
/// A vector of rendered lines.
#[allow(dead_code)]
pub fn render_markdown_scrollable(
    content: &str,
    scroll: &MarkdownScrollManager,
    width: usize,
) -> Vec<Line<'static>> {
    let elements = crate::markdown_renderer::render_markdown_to_elements(content, true);
    let mut result = Vec::new();

    for (idx, element) in elements.iter().enumerate() {
        if should_render_line(element, idx, scroll) {
            let rendered = render_element(element, width);
            result.extend(rendered);
        }
    }

    result
}
