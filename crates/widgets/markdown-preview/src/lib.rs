//! Markdown rendering widget for ratatui applications.
//!
//! Provides a markdown viewer with optional extensions:
//! - Syntax highlighting
//! - Scroll support
//!
//! # Example
//!
//! ```rust
//! use ratkit_markdown_preview::MarkdownWidget;
//!
//! let widget = MarkdownWidget::new("# Hello World\n\nWelcome!");
//! ```

pub mod foundation;
pub mod state;
pub mod widget;

pub use foundation::elements::{CodeBlockColors, CodeBlockTheme, MarkdownElement, TextSegment};
pub use foundation::parser::render_markdown_to_elements;
pub use foundation::source::MarkdownSource;
pub use foundation::types::GitStats;
pub use foundation::ElementKind;
pub use state::{
    CacheState, CollapseState, DisplaySettings, DoubleClickState, ExpandableEntry, ExpandableState,
    GitStatsState, MarkdownState, ScrollState, SelectionState, SourceState, TocEntry, TocState,
    VimMode, VimState,
};
pub use widget::enums::MarkdownWidgetMode;
pub use widget::MarkdownWidget;
