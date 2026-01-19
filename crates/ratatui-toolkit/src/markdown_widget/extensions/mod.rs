//! Extensions for the markdown widget.
//!
//! This module contains optional extensions that add functionality to the
//! markdown widget. Each extension is designed to work independently or
//! together with other extensions.
//!
//! # Available Extensions
//!
//! - `scrollbar`: Custom scrollbar with accurate scroll tracking
//! - `selection`: Mouse event handling for selection and navigation
//! - `theme`: Color themes and syntax highlighting
//! - `toc`: Table of Contents navigation widget

pub mod scrollbar;
pub mod selection;
pub mod theme;
pub mod toc;

pub use scrollbar::{CustomScrollbar, ScrollbarConfig};
pub use selection::{
    handle_click, handle_mouse_event, handle_mouse_event_with_double_click, should_render_line,
};
pub use toc::{Toc, TocConfig};
