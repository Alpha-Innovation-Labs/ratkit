//! Check if a position is within an area.

use ratatui::layout::Rect;

/// Check if a position is within an area.
///
/// # Arguments
///
/// * `x` - X coordinate
/// * `y` - Y coordinate
/// * `area` - The area to check against
///
/// # Returns
///
/// `true` if the position is within the area.
pub fn is_in_area(x: u16, y: u16, area: Rect) -> bool {
    x >= area.x && x < area.x + area.width && y >= area.y && y < area.y + area.height
}
