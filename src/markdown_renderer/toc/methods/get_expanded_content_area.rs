//! Get expanded content area sized to fit all entries.

use ratatui::layout::Rect;

use super::super::TocConfig;

/// Get the content area for expanded mode, sized to fit all entries.
///
/// This ensures click detection works even if the passed area height
/// doesn't match the actual number of entries.
///
/// # Arguments
///
/// * `area` - The outer TOC area.
/// * `config` - The TOC configuration.
/// * `entry_count` - Number of entries in the TOC.
///
/// # Returns
///
/// The inner content area with height based on entry count.
pub fn get_expanded_content_area(area: Rect, config: &TocConfig, entry_count: usize) -> Rect {
    let border_offset = if config.show_border { 1 } else { 0 };
    let border_size = if config.show_border { 2 } else { 0 };

    Rect {
        x: area.x + border_offset,
        y: area.y + border_offset,
        width: area.width.saturating_sub(border_size),
        // Use entry count as height to ensure all entries are clickable
        height: entry_count as u16,
    }
}
