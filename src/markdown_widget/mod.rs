//! Markdown rendering widget for ratatui applications.
//!
//! Provides a feature-rich markdown viewer with optional extensions:
//! - Table of contents (TOC)
//! - Minimap navigation
//! - Syntax highlighting
//! - Text selection
//! - File watching
//!
//! # Example
//!
//! ```rust,ignore
//! use ratatui_toolkit::markdown_widget::{MarkdownWidget, MarkdownScrollManager};
//!
//! // Minimal usage
//! let widget = MarkdownWidget::new(content, &mut scroll);
//!
//! // With TOC
//! let widget = MarkdownWidget::new(content, &mut scroll)
//!     .with_toc(&mut toc_state, TocConfig::default());
//!
//! // Full-featured
//! let widget = MarkdownWidget::new(content, &mut scroll)
//!     .with_toc(&mut toc_state, TocConfig::default())
//!     .with_selection(&mut selection, &mut double_click)
//!     .with_minimap(MinimapConfig::default())
//!     .with_theme(&app_theme)
//!     .show_scrollbar(true)
//!     .show_statusline(true);
//! ```

// Core modules
pub mod foundation;
pub mod state;
pub mod extensions;
pub mod widget;

// Internal re-exports for cross-module use
mod internal;

// ============================================================================
// Foundation (always available)
// ============================================================================

// Elements
pub use foundation::elements::{
    // Struct
    MarkdownElement,
    // Enums
    CheckboxState, CodeBlockBorderKind, ColumnAlignment, ElementKind, TableBorderKind, TextSegment,
    // Constants
    CodeBlockColors, CodeBlockTheme, BLOCKQUOTE_MARKER, BULLET_MARKERS, CHECKBOX_CHECKED,
    CHECKBOX_TODO, CHECKBOX_UNCHECKED, HEADING_ICONS, HORIZONTAL_RULE_CHAR,
};

// Element methods
pub use foundation::elements::methods::{render as render_element, render_with_options as render_element_with_options, RenderOptions};

// Parser
pub use foundation::parser::render_markdown_to_elements;

// Source
pub use foundation::source::MarkdownSource;

// Events
pub use foundation::events::{MarkdownDoubleClickEvent, MarkdownEvent};

// Types
pub use foundation::types::{GitStats, SelectionPos};

// Functions
pub use foundation::functions::{render_markdown, render_markdown_with_style};

// ============================================================================
// Widget
// ============================================================================

pub use widget::MarkdownWidget;
pub use widget::enums::MarkdownWidgetMode;

// ============================================================================
// State (always required)
// ============================================================================

pub use state::scroll_manager::{ExpandableState, MarkdownScrollManager, ParsedCache, RenderCache};
pub use state::toc_state::{TocEntry, TocState};
pub use state::selection_state::SelectionState;
pub use state::double_click_state::DoubleClickState;

// ============================================================================
// Extensions (toggleable)
// ============================================================================

// Minimap
pub use extensions::minimap::{Minimap, MinimapConfig};

// TOC
pub use extensions::toc::{Toc, TocConfig};

// Theme
pub use extensions::theme::{
    // Structs
    ColorMapping, ColorPalette, MarkdownStyle, MarkdownTheme, SyntaxHighlighter,
    // Enums
    SyntaxThemeVariant, ThemeVariant,
    // Functions
    get_effective_theme_variant, load_theme_from_json,
    // Palettes
    palettes,
};

// File watcher
pub use extensions::file_watcher::MarkdownFileWatcher;

// Selection handlers
pub use extensions::selection::{
    handlers::{handle_mouse_event, handle_mouse_event_with_double_click},
    helpers::{handle_click, should_render_line},
};
