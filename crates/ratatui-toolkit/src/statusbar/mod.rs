//! Status bar component
//!
//! Provides a status bar widget with left, center, and right sections.

pub mod constructors;
pub mod methods;
pub mod traits;

use ratatui::style::Style;

/// A status bar widget typically displayed at the bottom of the screen
pub struct StatusBar<'a> {
    /// Left-aligned items
    left_items: Vec<StatusItem<'a>>,
    /// Center-aligned items
    center_items: Vec<StatusItem<'a>>,
    /// Right-aligned items
    right_items: Vec<StatusItem<'a>>,
    /// Background style for the status bar
    style: Style,
}

/// An item in the status bar
#[derive(Clone)]
pub struct StatusItem<'a> {
    /// The text content
    text: String,
    /// The style for this item
    style: Style,
    /// Optional separator after this item
    separator: Option<&'a str>,
}
