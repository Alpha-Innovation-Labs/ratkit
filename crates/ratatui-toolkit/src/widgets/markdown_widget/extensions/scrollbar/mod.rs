//! Custom scrollbar widget for markdown document navigation.
//!
//! Provides a visual scrollbar that accurately tracks scroll position using block characters.
//! Supports click-to-scroll and drag interactions, with optional percentage indicator.
//!
//! # Features
//!
//! - Block character rendering (█ for thumb, ░ for track)
//! - Accurate scroll position tracking via ScrollState
//! - Click on track to jump to position
//! - Drag thumb to scroll
//! - Optional percentage indicator
//!
//! # Architecture
//!
//! The CustomScrollbar extension is a UI widget only - it receives `&ScrollState` as a parameter
//! and ONLY handles rendering. State mutations happen through ScrollState methods or via
//! the click_to_offset helper for interaction.

pub mod constructors;
pub mod enums;
mod methods;
mod traits;

pub use enums::*;
pub use methods::{click_to_offset, is_in_scrollbar_area};

use crate::widgets::markdown_widget::state::scroll_state::ScrollState;

/// Custom scrollbar widget for markdown navigation.
///
/// Shows scroll position using block characters with accurate tracking.
/// Supports click-to-scroll and drag interactions.
///
/// This is a UI-only widget that receives `&ScrollState` for state access.
/// State mutations happen through `ScrollState` methods, not here.
///
/// # Example
///
/// ```rust,ignore
/// use ratatui_toolkit::markdown_widget::extensions::scrollbar::{CustomScrollbar, ScrollbarConfig};
/// use ratatui_toolkit::markdown_widget::state::scroll_state::ScrollState;
///
/// let scroll_state = ScrollState::default();
/// let scrollbar = CustomScrollbar::new(&scroll_state)
///     .config(ScrollbarConfig::default())
///     .show_percentage(true);
/// ```
#[derive(Debug)]
pub struct CustomScrollbar<'a> {
    /// Reference to the scroll state.
    pub(crate) scroll_state: &'a ScrollState,
    /// Configuration for appearance.
    pub(crate) config: ScrollbarConfig,
    /// Whether to show percentage indicator.
    pub(crate) show_percentage: bool,
}
