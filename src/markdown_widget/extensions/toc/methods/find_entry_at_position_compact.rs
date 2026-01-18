//! Find entry at position in compact mode.

use ratatui::layout::Rect;

use crate::markdown_widget::state::toc_state::TocEntry;
use super::super::enums::TocConfig;
use super::get_content_area::get_content_area;

/// Find entry at position in compact mode.
///
/// # Arguments
///
/// * `y` - Screen Y coordinate.
/// * `area` - The outer TOC area.
/// * `config` - The TOC configuration.
/// * `entries` - The TOC entries.
///
/// # Returns
///
/// The entry index at that position, or None if no entry is there.
pub fn find_entry_at_position_compact(
    y: u16,
    area: Rect,
    config: &TocConfig,
    entries: &[TocEntry],
) -> Option<usize> {
    // Account for border in compact mode too
    let content_area = get_content_area(area, config);

    if y < content_area.y || y >= content_area.y + content_area.height {
        return None;
    }

    let relative_y = (y - content_area.y) as usize;

    if relative_y < entries.len() {
        Some(relative_y)
    } else {
        None
    }
}
