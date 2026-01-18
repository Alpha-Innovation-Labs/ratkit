//! Get line information at a given screen position.

use crate::markdown_renderer::render_element;
use crate::markdown_renderer::scroll_manager::MarkdownScrollManager;

use super::should_render_line::should_render_line;
use super::element_kind_to_string::element_kind_to_string;
use super::element_to_plain_text::element_to_plain_text;
use crate::markdown_renderer::markdown_widget::MarkdownDoubleClickEvent;

/// Get line information at the given screen position.
///
/// # Arguments
///
/// * `y` - Y coordinate relative to the widget
/// * `width` - Width of the widget
/// * `content` - The markdown content
/// * `scroll` - The scroll manager
///
/// # Returns
///
/// A `MarkdownDoubleClickEvent` if a line was found at the position.
pub(crate) fn get_line_at_position(
    y: usize,
    width: usize,
    content: &str,
    scroll: &MarkdownScrollManager,
) -> Option<MarkdownDoubleClickEvent> {
    let elements = crate::markdown_renderer::render_markdown_to_elements(content, true);
    let document_y = y + scroll.scroll_offset;
    let mut visual_line_idx = 0;
    let mut logical_line_num = 0; // Track the visible logical line number (1-indexed for display)

    for (idx, element) in elements.iter().enumerate() {
        if !should_render_line(element, idx, scroll) {
            continue;
        }

        logical_line_num += 1; // Increment for each visible logical line

        let rendered = render_element(element, width);
        let line_count = rendered.len();

        if document_y >= visual_line_idx && document_y < visual_line_idx + line_count {
            let line_kind = element_kind_to_string(&element.kind);
            let text_content = element_to_plain_text(&element.kind);

            return Some(MarkdownDoubleClickEvent {
                line_number: logical_line_num,
                line_kind,
                content: text_content,
            });
        }

        visual_line_idx += line_count;
    }

    None
}
