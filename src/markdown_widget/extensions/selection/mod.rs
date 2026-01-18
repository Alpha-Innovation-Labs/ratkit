//! Selection extension for markdown widget.
//!
//! Contains handlers for selection events, NOT state (state lives in state/selection_state/).

pub mod handlers;
pub mod helpers;

pub use handlers::{handle_mouse_event, handle_mouse_event_with_double_click};
pub use helpers::{handle_click, should_render_line};
