//! Menu bar component
//!
//! Provides a horizontal menu bar with selectable items.

pub mod constructors;
pub mod functions;
pub mod methods;
pub mod traits;

use ratatui::layout::Rect;
use ratatui::style::Style;

/// A single menu item
#[derive(Debug, Clone)]
pub struct MenuItem {
    /// The display name of the menu item
    pub name: String,
    /// Optional icon to display on the left (can be Nerd Font icon or emoji)
    pub icon: Option<String>,
    /// Internal index/value
    pub value: usize,
    /// Whether this item is currently selected
    pub selected: bool,
    /// Whether the mouse is hovering over this item
    pub hovered: bool,
    /// The rendered area of this item
    pub area: Option<Rect>,
}

/// A horizontal menu bar with selectable items
#[derive(Debug, Clone)]
pub struct MenuBar {
    pub items: Vec<MenuItem>,
    pub area: Option<Rect>,

    // Styling
    pub normal_style: Style,
    pub selected_style: Style,
    pub hover_style: Style,
    pub selected_hover_style: Style,
}
