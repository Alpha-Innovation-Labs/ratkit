//! Find the markdown element at a given screen position.

use crate::markdown_renderer::render_element;
use crate::markdown_renderer::scroll_manager::MarkdownScrollManager;
use crate::markdown_renderer::markdown_elements::MarkdownElement;

use super::helpers::should_render_line;

/// Find the markdown element at the given screen position.
///
/// # Arguments
///
/// * `content` - The markdown content
/// * `screen_y` - The Y coordinate on screen
/// * `scroll` - The scroll manager
///
/// # Returns
///
/// The index and element at the position, if found.
#[allow(dead_code)]
pub fn find_line_at_position(
    content: &str,
    screen_y: usize,
    scroll: &MarkdownScrollManager,
) -> Option<(usize, MarkdownElement)> {
    let elements = crate::markdown_renderer::render_markdown_to_elements(content, true);
    let mut current_y = 0;

    for (idx, element) in elements.iter().enumerate() {
        if should_render_line(element, idx, scroll) {
            let rendered = render_element(element, 80);
            let line_count = rendered.len();

            if screen_y >= current_y && screen_y < current_y + line_count {
                return Some((idx, element.clone()));
            }

            current_y += line_count;
        }
    }

    None
}
