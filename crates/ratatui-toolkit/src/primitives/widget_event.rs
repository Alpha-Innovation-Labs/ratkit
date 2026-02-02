//! Common events emitted by interactive widgets.
//!
//! This module provides a unified event system that all interactive widgets
//! can use to communicate actions back to the parent application.

use std::fmt;

/// Common events emitted by interactive widgets.
///
/// This enum provides a standardized way for widgets to communicate actions
/// to the parent application, reducing boilerplate in event handling loops.
///
/// # Example
///
/// ```rust
/// use ratatui_toolkit::prelude::*;
///
/// fn handle_widget_event(event: WidgetEvent) {
///     match event {
///         WidgetEvent::None => {}
///         WidgetEvent::Selected { path } => println!("Selected: {:?}", path),
///         WidgetEvent::Toggled { path, expanded } => println!("Toggled: {:?} = {}", path, expanded),
///         WidgetEvent::Scrolled { offset, direction } => println!("Scrolled to {}", offset),
///         WidgetEvent::FilterModeChanged { active, filter } => {
///             println!("Filter {}: {}", if active { "on" } else { "off" }, filter);
///         }
///         WidgetEvent::FilterModeExited { path } => println!("Filter exited, focus: {:?}", path),
///         WidgetEvent::MenuSelected { index, action: _ } => println!("Menu item {} selected", index),
///     }
/// }
/// ```
pub enum WidgetEvent {
    /// No event occurred.
    None,

    /// A node/item was selected.
    Selected {
        /// Path indices from root to the selected item.
        path: Vec<usize>,
    },

    /// An expandable item was toggled (expanded/collapsed).
    Toggled {
        /// Path indices from root to the toggled item.
        path: Vec<usize>,
        /// Whether the item is now expanded (true) or collapsed (false).
        expanded: bool,
    },

    /// Content was scrolled.
    Scrolled {
        /// The new scroll offset.
        offset: usize,
        /// Direction of scroll (positive = down, negative = up).
        direction: i32,
    },

    /// Filter mode changed (entered, text changed, or exited with Esc).
    FilterModeChanged {
        /// Whether filter mode is active.
        active: bool,
        /// Current filter text.
        filter: String,
    },

    /// Filter mode exited via Enter (focuses the selected item).
    FilterModeExited {
        /// Path indices of the item that was focused when filter mode was exited.
        path: Vec<usize>,
    },

    /// A menu item was selected.
    MenuSelected {
        /// Index of the selected menu item.
        index: usize,
        /// Optional action to execute when the menu item is selected.
        /// The action is consumed to ensure it is only executed once.
        action: Option<Box<dyn FnOnce() + Send>>,
    },
}

impl Default for WidgetEvent {
    fn default() -> Self {
        WidgetEvent::None
    }
}

impl fmt::Debug for WidgetEvent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            WidgetEvent::None => write!(f, "WidgetEvent::None"),
            WidgetEvent::Selected { path } => {
                write!(f, "WidgetEvent::Selected {{ path: {:?} }}", path)
            }
            WidgetEvent::Toggled { path, expanded } => {
                write!(
                    f,
                    "WidgetEvent::Toggled {{ path: {:?}, expanded: {} }}",
                    path, expanded
                )
            }
            WidgetEvent::Scrolled { offset, direction } => write!(
                f,
                "WidgetEvent::Scrolled {{ offset: {}, direction: {} }}",
                offset, direction
            ),
            WidgetEvent::FilterModeChanged { active, filter } => write!(
                f,
                "WidgetEvent::FilterModeChanged {{ active: {}, filter: {:?} }}",
                active, filter
            ),
            WidgetEvent::FilterModeExited { path } => {
                write!(f, "WidgetEvent::FilterModeExited {{ path: {:?} }}", path)
            }
            WidgetEvent::MenuSelected { index, action: _ } => {
                write!(f, "WidgetEvent::MenuSelected {{ index: {} }}", index)
            }
        }
    }
}
