//! Find entry at position in compact mode.

use ratatui::layout::Rect;

use super::super::enums::TocConfig;
use super::get_content_area::get_content_area;
use crate::markdown_widget::state::toc_state::TocEntry;

/// Find entry at position in compact mode.
///
/// Compact mode uses Braille markers (4 dots per cell) with configurable line_spacing.
/// This maps terminal Y coordinate to entry index by converting to Braille dot coordinates.
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

    let relative_y = y - content_area.y;
    let spacing = config.line_spacing.max(1) as f64;

    // Braille has 4 dots per cell. Convert terminal row to dot position.
    // Entries are rendered from top (entry 0 at top) with spacing between them.
    // In render_compact: pixel_y = canvas_height - (idx * spacing)
    // So entry 0 is at the top (canvas_height), entry N is at canvas_height - N*spacing
    //
    // To reverse this: given a terminal row, find which entry is closest.
    // Terminal row 0 corresponds to Braille dots 0-3 (from bottom of canvas)
    // But render uses inverted Y where entry 0 is at canvas_height (visual top).
    //
    // Simplify: relative_y=0 is the first content row (top of canvas visually).
    // Entry 0 is drawn at canvas_height, entry 1 at canvas_height - spacing, etc.
    // In terminal rows (inverted): entry 0 is at row 0, entry 1 is at row spacing/4, etc.
    //
    // Entry index = floor(relative_y * 4 / spacing)
    let dot_y = (relative_y as f64) * 4.0;
    let entry_idx = (dot_y / spacing).floor() as usize;

    if entry_idx < entries.len() {
        Some(entry_idx)
    } else {
        None
    }
}
