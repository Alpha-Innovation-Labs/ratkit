//! Border style utilities for interactive widgets.
//!
//! Provides helper functions for calculating border styles based on
//! widget state such as hover, selection, and drag interactions.

#[cfg(feature = "theme")]
use crate::services::theme::AppTheme;
use ratatui::style::Style;

/// Get the border style based on selection, hover, and drag states.
///
/// This helper function provides a consistent way to determine border
/// styling for interactive widgets that support these states.
///
/// # Arguments
///
/// * `theme` - The application theme (required for colored borders).
/// * `selection_active` - Whether text/item selection is active.
/// * `is_hovering` - Whether the mouse is hovering over the element.
/// * `is_dragging` - Whether the element is being dragged.
///
/// # Returns
///
/// The appropriate `Style` for the border based on the provided states.
///
/// # Example
///
/// ```rust
/// use ratatui_toolkit::primitives::border_style_helper;
///
/// let style = border_style_helper::get_border_style(
///     &theme,
///     false,  // selection not active
///     true,   // hovering
///     false,  // not dragging
/// );
/// ```
#[cfg(feature = "theme")]
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

/// Get a simple border style without theme support.
///
/// Use this function when theming is not enabled or when a neutral
/// border style is preferred.
///
/// # Arguments
///
/// * `selection_active` - Whether text/item selection is active.
/// * `is_hovering` - Whether the mouse is hovering over the element.
/// * `is_dragging` - Whether the element is being dragged.
///
/// # Returns
///
/// A highlighted `Style` when any state is active, default style otherwise.
pub fn get_border_style_simple(
    selection_active: bool,
    is_hovering: bool,
    is_dragging: bool,
) -> Style {
    if selection_active || is_hovering || is_dragging {
        Style::default().fg(ratatui::style::Color::Yellow)
    } else {
        Style::default()
    }
}
