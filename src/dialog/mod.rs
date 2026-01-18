//! Dialog component
//!
//! Provides modal dialog widgets with customizable buttons and styles.

pub mod constructors;
pub mod methods;
pub mod traits;
pub mod widget;

pub use widget::DialogWidget;

use ratatui::layout::Rect;
use ratatui::style::{Color, Style};

/// Dialog type
///
/// Represents different types of dialogs with associated visual styles.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DialogType {
    /// Informational dialog (cyan border)
    Info,
    /// Success dialog (green border)
    Success,
    /// Warning dialog (yellow border)
    Warning,
    /// Error dialog (red border)
    Error,
    /// Confirmation dialog (blue border)
    Confirm,
}

/// A dialog/modal widget that overlays content
///
/// Dialogs are centered modals that display a message and buttons.
/// They support different visual styles (info, success, warning, error, confirm)
/// and can handle mouse clicks on buttons.
#[allow(dead_code)]
pub struct Dialog<'a> {
    /// Dialog title
    title: &'a str,
    /// Dialog message
    message: &'a str,
    /// Dialog type
    dialog_type: DialogType,
    /// Buttons to show
    buttons: Vec<&'a str>,
    /// Selected button index
    selected_button: usize,
    /// Width percentage (0.0 to 1.0)
    width_percent: f32,
    /// Height percentage (0.0 to 1.0)
    height_percent: f32,
    /// Style for the dialog
    style: Style,
    /// Style for selected button
    button_selected_style: Style,
    /// Style for unselected button
    button_style: Style,
    /// Areas for buttons (for click detection)
    button_areas: Vec<Rect>,
    /// Theme color for info dialogs (overrides default when set)
    theme_info_color: Option<Color>,
    /// Theme color for success dialogs (overrides default when set)
    theme_success_color: Option<Color>,
    /// Theme color for warning dialogs (overrides default when set)
    theme_warning_color: Option<Color>,
    /// Theme color for error dialogs (overrides default when set)
    theme_error_color: Option<Color>,
    /// Theme color for confirm dialogs (overrides default when set)
    theme_confirm_color: Option<Color>,
}
