//! Handle click event at the given position.

use crate::markdown_widget::foundation::elements::{render, ElementKind};
use crate::markdown_widget::foundation::parser::render_markdown_to_elements;
use crate::markdown_widget::state::{CacheState, CollapseState, ExpandableState, ScrollState};

use super::should_render_line::should_render_line;

/// Handle click event at the given position.
///
/// # Arguments
///
/// * `_x` - X coordinate (unused)
/// * `y` - Y coordinate relative to the widget
/// * `width` - Width of the widget
/// * `content` - The markdown content
/// * `scroll` - The scroll state
/// * `collapse` - The collapse state
/// * `expandable` - The expandable state
/// * `cache` - The cache state
///
/// # Returns
///
/// `true` if the click was handled.
#[allow(clippy::too_many_arguments)]
pub fn handle_click(
    _x: usize,
    y: usize,
    width: usize,
    content: &str,
    scroll: &ScrollState,
    collapse: &mut CollapseState,
    expandable: &mut ExpandableState,
    cache: &mut CacheState,
) -> bool {
    let elements = render_markdown_to_elements(content, true);

    // Account for scroll offset - y is relative to visible area
    let document_y = y + scroll.scroll_offset;
    let mut line_idx = 0;

    for (idx, element) in elements.iter().enumerate() {
        // Skip elements that shouldn't be rendered (collapsed sections)
        if !should_render_line(&element, idx, collapse) {
            continue;
        }

        let rendered = render(&element, width);
        let line_count = rendered.len();

        if document_y >= line_idx && document_y < line_idx + line_count {
            match &element.kind {
                ElementKind::Heading {
                    section_id,
                    collapsed: _,
                    ..
                } => {
                    collapse.toggle_section(*section_id);
                    cache.invalidate();
                    return true;
                }
                ElementKind::Frontmatter { .. } => {
                    collapse.toggle_section(0);
                    cache.invalidate();
                    return true;
                }
                ElementKind::FrontmatterStart { .. } => {
                    collapse.toggle_section(0);
                    cache.invalidate();
                    return true;
                }
                ElementKind::ExpandToggle { content_id, .. } => {
                    expandable.toggle(content_id);
                    cache.invalidate();
                    return true;
                }
                _ => {}
            }
        }

        line_idx += line_count;
    }

    false
}
