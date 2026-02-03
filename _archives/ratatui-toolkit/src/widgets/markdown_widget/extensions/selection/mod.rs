//! Selection extension for markdown widget.
//!
//! Provides text selection via click-and-drag with auto-copy to clipboard.
//! Contains handlers for selection events, NOT state (state lives in state/selection_state/).
//!
//! # Mouse Capture Requirement
//!
//! For text selection to work (drag to select, click to highlight),
//! you must enable mouse capture with crossterm:
//!
//! ```rust,ignore
//! use crossterm::event::{EnableMouseCapture, DisableMouseCapture};
//! execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
//! ```
//!
//! Without `EnableMouseCapture`, click and drag events will not be received.

pub mod handlers;
pub mod helpers;

pub use handlers::{handle_mouse_event, handle_mouse_event_with_double_click};
pub use helpers::{handle_click, should_render_line};
