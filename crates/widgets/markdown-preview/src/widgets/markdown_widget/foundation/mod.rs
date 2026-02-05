//! Foundation module for markdown widget.
//!
//! Contains core elements, types, events, helpers, and rendering functions.

pub mod elements;
pub mod events;
pub mod functions;
pub mod helpers;
pub mod parser;
pub mod source;
pub mod types;

pub use events::{MarkdownDoubleClickEvent, MarkdownEvent};
pub use functions::{render_markdown, render_markdown_with_style};
pub use types::{GitStats, SelectionPos};
