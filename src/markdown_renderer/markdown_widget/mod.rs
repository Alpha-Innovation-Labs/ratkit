//! Markdown renderer widget with scroll and interaction support.
//!
//! This module provides a complete markdown rendering widget that handles
//! scrolling, click interactions, and collapse/expand of sections.
//!
//! # Usage
//!
//! Use the `MarkdownWidget` as the primary entry point for rendering markdown:
//!
//! ```rust,no_run
//! use ratatui_toolkit::{MarkdownWidget, MarkdownScrollManager, SelectionState, DoubleClickState};
//!
//! // Create state managers
//! let mut scroll = MarkdownScrollManager::new();
//! let mut selection = SelectionState::new();
//! let mut double_click = DoubleClickState::new();
//!
//! // Create and configure the widget
//! let widget = MarkdownWidget::new(content, &mut scroll, &mut selection, &mut double_click)
//!     .show_toc(true)
//!     .show_statusline(true)
//!     .show_scrollbar(true);
//! ```

mod double_click_state;
mod helpers;
mod markdown_double_click_event;
mod markdown_event;
mod markdown_widget;
mod selection_state;

// Internal functions (not re-exported publicly)
mod find_line_at_position;
mod handle_mouse_event;
mod handle_mouse_event_with_double_click;
mod handle_mouse_event_with_selection;
mod is_clickable_at_position;
pub(crate) mod render_markdown_interactive_with_options;
pub(crate) mod render_markdown_interactive_with_selection;

// Public re-exports - the widget-based API
pub use double_click_state::DoubleClickState;
pub use markdown_double_click_event::MarkdownDoubleClickEvent;
pub use markdown_event::MarkdownEvent;
pub use markdown_widget::{GitStats, MarkdownWidget, MarkdownWidgetMode};
pub use selection_state::{SelectionPos, SelectionState};

// Internal re-exports for widget implementation
pub(crate) use find_line_at_position::find_line_at_position;
pub(crate) use handle_mouse_event::handle_mouse_event;
pub(crate) use handle_mouse_event_with_double_click::handle_mouse_event_with_double_click;
pub(crate) use handle_mouse_event_with_selection::{
    copy_selection_to_clipboard, handle_mouse_event_with_selection, SelectionMouseResult,
};
pub(crate) use is_clickable_at_position::is_clickable_at_position;

#[cfg(test)]
mod tests;
