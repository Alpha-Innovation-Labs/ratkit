//! Render markdown with interactive scroll and collapse state.

use ratatui::{layout::Rect, text::Text};

use crate::markdown_renderer::scroll_manager::MarkdownScrollManager;

use super::render_markdown_interactive_with_options;

/// Render markdown with interactive scroll and collapse state.
///
/// # Arguments
///
/// * `content` - The markdown content to render
/// * `scroll` - The scroll manager
/// * `area` - The area to render into
///
/// # Returns
///
/// The rendered text.
pub fn render_markdown_interactive(
    content: &str,
    scroll: &mut MarkdownScrollManager,
    area: Rect,
) -> Text<'static> {
    render_markdown_interactive_with_options(content, scroll, area, false, None)
}
