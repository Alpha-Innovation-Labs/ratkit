//! Calculate split panel areas.

use ratatui::layout::Rect;

/// Panel areas returned from split calculation.
pub struct SplitAreas {
    pub left: Rect,
    pub right: Rect,
}

/// Calculate left and right panel areas based on split percentage.
///
/// # Arguments
///
/// * `area` - The total area to split.
/// * `split_percent` - The percentage for the left panel (0-100).
///
/// # Returns
///
/// A `SplitAreas` struct containing the left and right panel rectangles.
pub fn calculate_split_areas(area: Rect, split_percent: u16) -> SplitAreas {
    let left_width = (area.width as u32 * split_percent as u32 / 100) as u16;

    let left = Rect {
        x: area.x,
        y: area.y,
        width: left_width,
        height: area.height,
    };

    let right = Rect {
        x: area.x + left_width,
        y: area.y,
        width: area.width - left_width,
        height: area.height,
    };

    SplitAreas { left, right }
}
