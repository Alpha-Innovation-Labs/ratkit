//! Calculate split panel areas.
//!
//! This module has been migrated to use the library utility.
//! The original implementation has been extracted to:
//! `ratatui_toolkit::primitives::resizable_grid::ResizableGrid::calculate_split_area()`

use ratatui::layout::Rect;
use ratatui_toolkit::primitives::resizable_grid::{ResizableGrid, SplitAreas};

/// Calculate left and right panel areas based on split percentage.
///
/// This is a wrapper that delegates to the library utility.
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
    let grid = ResizableGrid::new(0);
    grid.calculate_split_area(area, split_percent)
}
