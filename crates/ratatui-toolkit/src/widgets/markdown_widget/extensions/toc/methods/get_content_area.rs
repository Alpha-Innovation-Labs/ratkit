//! Get content area inside border for position calculations.

use ratatui::layout::Rect;

use super::super::enums::TocConfig;

/// Get the content area inside border for position calculations.
///
/// # Arguments
///
/// * `area` - The outer TOC area.
/// * `config` - The TOC configuration.
///
/// # Returns
///
/// The inner content area, accounting for border if enabled.
pub fn get_content_area(area: Rect, config: &TocConfig) -> Rect {
    if config.show_border && area.width >= 4 && area.height >= 3 {
        Rect {
            x: area.x + 1,
            y: area.y + 1,
            width: area.width.saturating_sub(2),
            height: area.height.saturating_sub(2),
        }
    } else {
        area
    }
}
