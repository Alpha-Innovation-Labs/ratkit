//! Markdown rendering widget for ratatui applications.
//!
//! Provides a feature-rich markdown viewer with optional extensions:
//! - Table of contents (TOC)
//! - Syntax highlighting
//! - Text selection
//! - Click-to-highlight line selection
//!
//! # Mouse Capture Requirement
//!
//! For mouse interactions (click, drag, hover) to work, you must enable
//! mouse capture with crossterm:
//!
//! ```rust,ignore
//! use crossterm::event::{EnableMouseCapture, DisableMouseCapture};
//!
//! // On startup:
//! execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
//!
//! // On cleanup:
//! execute!(stdout, LeaveAlternateScreen, DisableMouseCapture)?;
//! ```
//!
//! Without `EnableMouseCapture`, scroll wheel may still work (terminal-dependent),
//! but click events will not be received.
//!
//! # Example (Recommended - Unified State)
//!
//! ```rust,ignore
//! use ratatui_toolkit::{MarkdownWidget, MarkdownState};
//!
//! // Create unified state
//! let mut state = MarkdownState::default();
//! state.source.set_content("# Hello World\n\nWelcome!");
//!
//! // Create widget from state
//! let content = state.content().to_string();
//! let widget = MarkdownWidget::from_state(&content, &mut state)
//!     .show_toc(true)
//!     .show_statusline(true)
//!     .show_scrollbar(true);
//! ```
//!
//! # Example (Individual State Modules)
//!
//! ```rust,ignore
//! use ratatui_toolkit::markdown_widget::{MarkdownWidget, state::*};
//!
//! // Create state modules
//! let mut scroll = ScrollState::default();
//! let mut source = SourceState::default();
//! let mut cache = CacheState::default();
//! let display = DisplaySettings::default();
//! let mut collapse = CollapseState::default();
//! let mut expandable = ExpandableState::default();
//! let mut git_stats = GitStatsState::default();
//! let mut vim = VimState::default();
//! let mut selection = SelectionState::default();
//! let mut double_click = DoubleClickState::default();
//!
//! // Create widget with individual state modules
//! let widget = MarkdownWidget::new(
//!     content,
//!     &mut scroll,
//!     &mut source,
//!     &mut cache,
//!     &display,
//!     &mut collapse,
//!     &mut expandable,
//!     &mut git_stats,
//!     &mut vim,
//!     &mut selection,
//!     &mut double_click,
//! )
//! .show_toc(true)
//! .show_statusline(true)
//! .show_scrollbar(true);
//! ```

// Core modules
pub mod extensions;
pub mod foundation;
pub mod state;
pub mod widget;

// Internal re-exports for cross-module use
mod internal;

// ============================================================================
// Foundation (always available)
// ============================================================================

// Elements
pub use foundation::elements::{
    // Enums
    CheckboxState,
    CodeBlockBorderKind,
    // Constants
    CodeBlockColors,
    CodeBlockTheme,
    ColumnAlignment,
    ElementKind,
    // Struct
    MarkdownElement,
    TableBorderKind,
    TextSegment,
    BLOCKQUOTE_MARKER,
    BULLET_MARKERS,
    CHECKBOX_CHECKED,
    CHECKBOX_TODO,
    CHECKBOX_UNCHECKED,
    HEADING_ICONS,
    HORIZONTAL_RULE_CHAR,
};

// Element methods
pub use foundation::elements::{
    render as render_element, render_with_options as render_element_with_options, RenderOptions,
};

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
pub use widget::MarkdownWidgetMode;

// ============================================================================
// State (always required)
// ============================================================================

pub use state::{
    CacheState, CollapseState, DisplaySettings, DoubleClickState, ExpandableEntry, ExpandableState,
    GitStatsState, MarkdownState, ParsedCache, RenderCache, ScrollState, SelectionState,
    SourceState, TocEntry, TocState, VimState,
};

// ============================================================================
// Extensions (toggleable)
// ============================================================================

// TOC
pub use extensions::toc::{Toc, TocConfig};

// Theme
pub use extensions::theme::{
    // Functions
    get_effective_theme_variant,
    load_theme_from_json,
    // Palettes
    palettes,
    // Structs
    ColorMapping,
    ColorPalette,
    MarkdownStyle,
    MarkdownTheme,
    SyntaxHighlighter,
    // Enums
    SyntaxThemeVariant,
    ThemeVariant,
};

// Selection handlers
pub use extensions::selection::{
    handle_click, handle_mouse_event, handle_mouse_event_with_double_click, should_render_line,
};
