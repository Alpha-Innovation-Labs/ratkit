//! Find entry at position in expanded mode.

use ratatui::layout::Rect;

use super::super::TocEntry;

/// Find entry at position in expanded mode.
///
/// # Arguments
///
/// * `x` - Screen X coordinate.
/// * `y` - Screen Y coordinate.
/// * `content_area` - The content area inside the border.
/// * `entries` - The TOC entries.
/// * `toc_scroll_offset` - Current scroll offset within the TOC.
///
/// # Returns
///
/// The entry index at that position, or None if no entry is there.
pub fn find_entry_at_position_expanded(
    x: u16,
    y: u16,
    content_area: Rect,
    entries: &[TocEntry],
    toc_scroll_offset: usize,
) -> Option<usize> {
    // Check if position is within the content area horizontally
    if x < content_area.x || x >= content_area.x + content_area.width {
        return None;
    }

    // Check vertical bounds - must be at or below the content start
    if y < content_area.y {
        return None;
    }

    // Use TOC scroll offset for position calculation
    let relative_y = (y - content_area.y) as usize;
    let entry_idx = toc_scroll_offset + relative_y;

    // Return the entry at position if it exists
    if entry_idx < entries.len() {
        Some(entry_idx)
    } else {
        None
    }
}
