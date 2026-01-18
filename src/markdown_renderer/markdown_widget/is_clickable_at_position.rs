//! Check if a position is on a clickable element.

use crate::markdown_renderer::scroll_manager::MarkdownScrollManager;
use crate::markdown_renderer::markdown_elements::ElementKind;

use super::find_line_at_position;

/// Check if a position is on a clickable element.
///
/// # Arguments
///
/// * `content` - The markdown content
/// * `_x` - The X coordinate (unused)
/// * `y` - The Y coordinate
/// * `scroll` - The scroll manager
///
/// # Returns
///
/// `true` if the position is on a clickable element.
#[allow(dead_code)]
pub fn is_clickable_at_position(
    content: &str,
    _x: usize,
    y: usize,
    scroll: &MarkdownScrollManager,
) -> bool {
    if let Some((_, element)) = find_line_at_position(content, y, scroll) {
        matches!(
            element.kind,
            ElementKind::Heading { .. }
                | ElementKind::Frontmatter { .. }
                | ElementKind::ExpandToggle { .. }
        )
    } else {
        false
    }
}
