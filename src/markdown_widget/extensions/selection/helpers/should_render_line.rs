//! Check if a line should be rendered based on collapse state.

use crate::markdown_widget::state::scroll_manager::MarkdownScrollManager;
use crate::markdown_widget::foundation::elements::{ElementKind, MarkdownElement};

/// Check if a markdown element should be rendered based on collapse state.
///
/// # Arguments
///
/// * `element` - The element to check
/// * `_idx` - The index of the element (unused but kept for API compatibility)
/// * `scroll` - The scroll manager containing collapse state
///
/// # Returns
///
/// `true` if the element should be rendered.
pub fn should_render_line(
    element: &MarkdownElement,
    _idx: usize,
    scroll: &MarkdownScrollManager,
) -> bool {
    // Headings: visible unless a parent section is collapsed (hierarchical collapse)
    if let ElementKind::Heading { section_id, .. } = &element.kind {
        // Check if any parent section is collapsed
        if let Some(&(_level, parent_id)) = scroll.section_hierarchy.get(section_id) {
            if let Some(parent) = parent_id {
                // If parent is collapsed, this heading is hidden
                if scroll.is_section_collapsed(parent) {
                    return false;
                }
            }
        }
        return true;
    }

    // Legacy Frontmatter block is always visible
    if matches!(element.kind, ElementKind::Frontmatter { .. }) {
        return true;
    }

    // FrontmatterStart is always visible (contains collapse toggle)
    if matches!(element.kind, ElementKind::FrontmatterStart { .. }) {
        return true;
    }

    // FrontmatterField and FrontmatterEnd are hidden when frontmatter is collapsed
    if matches!(
        element.kind,
        ElementKind::FrontmatterField { .. } | ElementKind::FrontmatterEnd
    ) {
        // Frontmatter uses section_id 0 for collapse state
        if scroll.is_section_collapsed(0) {
            return false;
        }
        return true;
    }

    // Check if this element belongs to a collapsed section
    if let Some(section_id) = element.section_id {
        if scroll.is_section_collapsed(section_id) {
            return false;
        }
    }

    true
}
