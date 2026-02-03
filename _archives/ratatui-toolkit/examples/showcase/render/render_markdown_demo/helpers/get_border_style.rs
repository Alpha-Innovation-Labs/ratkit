//! Get border style based on widget state.
//!
//! This module has been migrated to use the library utility.
//! The original implementation has been extracted to:
//! `ratatui_toolkit::primitives::border_style_helper::get_border_style()`

use ratatui::style::Style;
use ratatui_toolkit::primitives::border_style_helper;
use ratatui_toolkit::AppTheme;

/// Get the border style based on selection, hover, and drag states.
///
/// This is a wrapper that delegates to the library utility.
///
/// # Arguments
///
/// * `theme` - The application theme.
/// * `selection_active` - Whether text selection is active.
/// * `is_hovering` - Whether the mouse is hovering over the divider.
/// * `is_dragging` - Whether the divider is being dragged.
///
/// # Returns
///
/// The appropriate `Style` for the border.
pub fn get_border_style(
    theme: &AppTheme,
    selection_active: bool,
    is_hovering: bool,
    is_dragging: bool,
) -> Style {
    border_style_helper::get_border_style(theme, selection_active, is_hovering, is_dragging)
}
