//! Handle click event at the given position.

use crate::markdown_renderer::render_element;
use crate::markdown_renderer::scroll_manager::MarkdownScrollManager;
use crate::markdown_renderer::markdown_elements::ElementKind;

use super::should_render_line::should_render_line;

/// Handle click event at the given position.
///
/// # Arguments
///
/// * `_x` - X coordinate (unused)
/// * `y` - Y coordinate relative to the widget
/// * `width` - Width of the widget
/// * `content` - The markdown content
/// * `scroll` - The scroll manager
///
/// # Returns
///
/// `true` if the click was handled.
pub(crate) fn handle_click(
    _x: usize,
    y: usize,
    width: usize,
    content: &str,
    scroll: &mut MarkdownScrollManager,
) -> bool {
    let elements = crate::markdown_renderer::render_markdown_to_elements(content, true);

    // Account for scroll offset - y is relative to visible area
    let document_y = y + scroll.scroll_offset;
    let mut line_idx = 0;

    for (idx, element) in elements.iter().enumerate() {
        // Skip elements that shouldn't be rendered (collapsed sections)
        if !should_render_line(element, idx, scroll) {
            continue;
        }

        let rendered = render_element(element, width);
        let line_count = rendered.len();

        if document_y >= line_idx && document_y < line_idx + line_count {
            match &element.kind {
                ElementKind::Heading {
                    section_id,
                    collapsed: _,
                    ..
                } => {
                    scroll.toggle_section_collapse(*section_id);
                    scroll.invalidate_cache(); // Invalidate cache after toggle
                    return true;
                }
                ElementKind::Frontmatter { .. } => {
                    scroll.toggle_section_collapse(0);
                    scroll.invalidate_cache();
                    return true;
                }
                ElementKind::FrontmatterStart { .. } => {
                    scroll.toggle_section_collapse(0);
                    scroll.invalidate_cache();
                    return true;
                }
                ElementKind::ExpandToggle { content_id, .. } => {
                    scroll.toggle_expandable(content_id);
                    scroll.invalidate_cache();
                    return true;
                }
                _ => {}
            }
        }

        line_idx += line_count;
    }

    false
}
