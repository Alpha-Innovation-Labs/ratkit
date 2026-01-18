//! Markdown renderer widget with scroll and interaction support.
//!
//! This module provides a complete markdown rendering widget that handles
//! scrolling, click interactions, and collapse/expand of sections.

mod double_click_state;
mod helpers;
mod markdown_double_click_event;
mod markdown_event;
mod markdown_widget;
mod selection_state;

// Standalone functions
mod find_line_at_position;
mod handle_mouse_event;
mod handle_mouse_event_with_double_click;
mod handle_mouse_event_with_selection;
mod is_clickable_at_position;
mod render_markdown_interactive;
mod render_markdown_interactive_with_options;
mod render_markdown_interactive_with_selection;
mod render_markdown_scrollable;
mod render_markdown_statusline;
mod render_markdown_with_minimap;

// Re-exports
pub use double_click_state::DoubleClickState;
pub use markdown_double_click_event::MarkdownDoubleClickEvent;
pub use markdown_event::MarkdownEvent;
pub use markdown_widget::{GitStats, MarkdownWidget, MarkdownWidgetMode};
pub use selection_state::{SelectionPos, SelectionState};

pub use find_line_at_position::find_line_at_position;
pub use handle_mouse_event::handle_mouse_event;
pub use handle_mouse_event_with_double_click::handle_mouse_event_with_double_click;
pub use handle_mouse_event_with_selection::{copy_selection_to_clipboard, handle_mouse_event_with_selection, SelectionMouseResult};
pub use is_clickable_at_position::is_clickable_at_position;
pub use render_markdown_interactive::render_markdown_interactive;
pub use render_markdown_interactive_with_options::render_markdown_interactive_with_options;
pub use render_markdown_interactive_with_selection::render_markdown_interactive_with_selection;
pub use render_markdown_scrollable::render_markdown_scrollable;
pub use render_markdown_statusline::{render_markdown_statusline, render_markdown_statusline_from_scroll};
pub use render_markdown_with_minimap::{render_markdown_with_minimap, MarkdownRenderOptions};

#[cfg(test)]
mod tests;
