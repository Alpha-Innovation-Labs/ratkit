//! # ratatui-toolkit
//!
//! A comprehensive collection of reusable TUI components for [ratatui](https://ratatui.rs/),
//! the Rust terminal UI library.
//!
//! ## Overview
//!
//! This crate provides production-ready, reusable widgets for building terminal user interfaces:
//!
//! - **Layout Components**: [`ResizableSplit`] for flexible UI layouts
//! - **UI Components**: [`Button`], [`Dialog`], [`Toast`], [`Pane`] for common UI elements
//! - **Widgets**: [`TreeView`], [`FuzzyFinder`] for data display
//! - **Navigation**: [`MenuBar`], [`HotkeyFooter`] for navigation aids
//! - **Rendering**: [`render_markdown`] for markdown to ratatui text conversion
//! - **Terminal**: [`TermTui`] for embedded terminal emulation
//! - **Theming**: [`theme`] module with 33 builtin themes and JSON loader
//!
//! ## Feature Flags
//!
//! The crate uses feature flags to minimize dependencies:
//!
//! | Feature | Default | Description |
//! |---------|---------|-------------|
//! | `markdown` | Yes | Markdown rendering support |
//! | `tree` | Yes | Tree view widget |
//! | `dialog` | Yes | Modal dialog components |
//! | `toast` | Yes | Toast notification system |
//! | `split` | Yes | Resizable split panels |
//! | `menu` | Yes | Menu bar component |
//! | `statusline` | Yes | Powerline-style statusline |
//! | `hotkey` | Yes | Hotkey footer and modal |
//! | `terminal` | No | Terminal emulator (TermTui) |
//! | `fuzzy` | No | Fuzzy finder component |
//! | `file-tree` | No | File system tree with devicons |
//! | `theme` | No | Theme system with 33 builtin themes |
//! | `full` | No | Enable all features |
//!
//! ## Quick Start
//!
//! ```rust,no_run
//! use ratatui::prelude::*;
//! use ratatui_toolkit::prelude::*;
//!
//! // Create a resizable split
//! let split = ResizableSplit::new(30); // 30% left, 70% right
//!
//! // Create a toast notification
//! let mut manager = ToastManager::new();
//! manager.success("File saved!");
//!
//! // Render markdown
//! let text = render_markdown("# Hello\n\n**Bold** and *italic* text.", None);
//! ```
//!
//! ## Examples
//!
//! See the `examples/` directory for runnable demos of each component:
//!
//! ```bash
//! cargo run --example resizable_split_demo
//! cargo run --example tree_view_demo --features tree
//! cargo run --example toast_manager_demo --features toast
//! ```

#![cfg_attr(docsrs, feature(doc_cfg))]

// Core components - always available
pub mod primitives;

pub mod diff_file_tree;
pub mod widgets;

// Feature-gated components
#[cfg(feature = "hotkey")]
#[cfg_attr(docsrs, doc(cfg(feature = "hotkey")))]
pub mod hotkey_footer;

#[cfg(feature = "fuzzy")]
#[cfg_attr(docsrs, doc(cfg(feature = "fuzzy")))]
pub mod fuzzy_finder;

#[cfg(feature = "file-tree")]
#[cfg_attr(docsrs, doc(cfg(feature = "file-tree")))]
pub mod file_system_tree;

// Services module - shared infrastructure
pub mod services;

// Re-export commonly used types - always available
pub use diff_file_tree::{DiffFileTree, FileStatus};
pub use primitives::button::render_title_with_buttons::render_title_with_buttons;
pub use primitives::button::Button;
pub use primitives::pane::Pane;
pub use widgets::ai_chat::{AIChat, AIChatEvent, InputState, Message, MessageRole, MessageStore};
pub use widgets::code_diff::{CodeDiff, DiffConfig, DiffHunk, DiffLine, DiffLineKind, DiffStyle};
pub use widgets::split_layout::SplitLayoutWidget;
pub use widgets::split_layout::SplitLayoutWidgetState;

// Feature-gated re-exports
#[cfg(feature = "dialog")]
pub use primitives::dialog::widget::DialogWidget;
#[cfg(feature = "dialog")]
pub use primitives::dialog::{Dialog, DialogType};

#[cfg(feature = "toast")]
pub use primitives::toast::methods::render_toasts::render_toasts;
#[cfg(feature = "toast")]
pub use primitives::toast::{Toast, ToastLevel, ToastManager};

#[cfg(feature = "split")]
pub use primitives::resizable_split::{ResizableSplit, SplitDirection};
#[cfg(feature = "split")]
pub use primitives::split_layout::{PaneId, PaneLayout, SplitAxis, SplitLayout};

#[cfg(feature = "tree")]
pub use primitives::tree_view::{
    get_visible_paths, matches_filter, NodeState, TreeKeyBindings, TreeNavigator, TreeNode,
    TreeView, TreeViewRef, TreeViewState,
};

#[cfg(feature = "menu")]
pub use primitives::menu_bar::{MenuBar, MenuItem};

#[cfg(feature = "statusline")]
pub use primitives::statusline::{
    OperationalMode, StatusLineStacked, StyledStatusLine, SLANT_BL_TR, SLANT_TL_BR,
};

#[cfg(feature = "hotkey")]
pub use hotkey_footer::{HotkeyFooter, HotkeyFooterBuilder, HotkeyItem};

#[cfg(feature = "hotkey")]
#[cfg(feature = "markdown")]
pub use widgets::markdown_widget::{
    render_markdown, render_markdown_with_style, CacheState, CodeBlockTheme, CollapseState,
    DisplaySettings, DoubleClickState, ExpandableState, GitStats, GitStatsState,
    MarkdownDoubleClickEvent, MarkdownEvent, MarkdownState, MarkdownStyle, MarkdownWidget,
    MarkdownWidgetMode, ScrollState, SelectionPos, SelectionState, SourceState, VimState,
};

#[cfg(feature = "terminal")]
pub use primitives::termtui::{TermTui, TermTuiKeyBindings};

#[cfg(feature = "fuzzy")]
pub use fuzzy_finder::FuzzyFinder;

#[cfg(feature = "file-tree")]
pub use file_system_tree::{FileSystemEntry, FileSystemTree, FileSystemTreeConfig};

#[cfg(feature = "theme")]
pub use services::theme::{AppTheme, DiffColors, MarkdownColors, SyntaxColors, ThemeVariant};

// File watcher service - always available
pub use services::file_watcher::{FileWatcher, WatchConfig, WatchMode};

/// Prelude module for convenient imports
///
/// # Example
///
/// ```rust
/// use ratatui_toolkit::prelude::*;
/// ```
pub mod prelude {
    // Core components
    pub use crate::diff_file_tree::{DiffFileTree, FileStatus};
    pub use crate::primitives::button::render_title_with_buttons::render_title_with_buttons;
    pub use crate::primitives::button::Button;
    pub use crate::primitives::pane::Pane;
    pub use crate::widgets::ai_chat::{
        AIChat, AIChatEvent, InputState, Message, MessageRole, MessageStore,
    };
    pub use crate::widgets::code_diff::{
        CodeDiff, DiffConfig, DiffHunk, DiffLine, DiffLineKind, DiffStyle,
    };

    // Feature-gated components
    #[cfg(feature = "dialog")]
    pub use crate::primitives::dialog::widget::DialogWidget;
    #[cfg(feature = "dialog")]
    pub use crate::primitives::dialog::{Dialog, DialogType};

    #[cfg(feature = "toast")]
    pub use crate::primitives::toast::methods::render_toasts::render_toasts;
    #[cfg(feature = "toast")]
    pub use crate::primitives::toast::{Toast, ToastLevel, ToastManager};

    #[cfg(feature = "split")]
    pub use crate::primitives::resizable_split::{ResizableSplit, SplitDirection};
    #[cfg(feature = "split")]
    pub use crate::primitives::split_layout::{PaneId, PaneLayout, SplitAxis, SplitLayout};
    #[cfg(feature = "split")]
    pub use crate::widgets::split_layout::{SplitLayoutWidget, SplitLayoutWidgetState};

    #[cfg(feature = "tree")]
    pub use crate::primitives::tree_view::{
        get_visible_paths, matches_filter, NodeState, TreeKeyBindings, TreeNavigator, TreeNode,
        TreeView, TreeViewRef, TreeViewState,
    };

    #[cfg(feature = "menu")]
    pub use crate::primitives::menu_bar::{MenuBar, MenuItem};

    #[cfg(feature = "statusline")]
    pub use crate::primitives::statusline::{
        OperationalMode, StatusLineStacked, StyledStatusLine, SLANT_BL_TR, SLANT_TL_BR,
    };

    #[cfg(feature = "hotkey")]
    pub use crate::hotkey_footer::{HotkeyFooter, HotkeyFooterBuilder, HotkeyItem};

    #[cfg(feature = "markdown")]
    pub use crate::widgets::markdown_widget::{
        render_markdown, render_markdown_with_style, CacheState, CollapseState, DisplaySettings,
        DoubleClickState, ExpandableState, GitStatsState, MarkdownState, MarkdownStyle,
        MarkdownWidget, ScrollState, SelectionState, SourceState, VimState,
    };

    #[cfg(feature = "terminal")]
    pub use crate::primitives::termtui::{TermTui, TermTuiKeyBindings};

    #[cfg(feature = "fuzzy")]
    pub use crate::fuzzy_finder::FuzzyFinder;

    #[cfg(feature = "file-tree")]
    pub use crate::file_system_tree::{FileSystemEntry, FileSystemTree, FileSystemTreeConfig};

    #[cfg(feature = "theme")]
    pub use crate::services::theme::{
        AppTheme, DiffColors, MarkdownColors, SyntaxColors, ThemeVariant,
    };

    // Services
    pub use crate::services::file_watcher::{FileWatcher, WatchConfig, WatchMode};
}

/// Error types for the crate
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// IO error
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    /// Terminal error
    #[error("Terminal error: {0}")]
    Terminal(String),

    /// Parse error
    #[error("Parse error: {0}")]
    Parse(String),
}

/// Result type for the crate
pub type Result<T> = std::result::Result<T, Error>;
