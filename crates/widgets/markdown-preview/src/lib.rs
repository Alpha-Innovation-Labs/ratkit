//! Markdown rendering widget for ratatui applications.
//!
//! Provides a feature-rich markdown viewer with TOC, selection, themes,
//! syntax highlighting, and more.

pub mod primitives;
pub mod services;
pub mod widgets;

pub use widgets::markdown_widget::*;
