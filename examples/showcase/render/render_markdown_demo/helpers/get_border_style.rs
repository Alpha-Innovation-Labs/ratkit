//! Get border style based on widget state.

use ratatui::style::Style;
use ratatui_toolkit::AppTheme;

/// Get the border style based on selection, hover, and drag states.
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
    if selection_active {
        Style::default().fg(theme.border_active)
    } else if is_hovering || is_dragging {
        Style::default().fg(theme.border_active)
    } else {
        Style::default()
    }
}
