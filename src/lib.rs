//! # ratatui-toolkit
//!
//! A comprehensive collection of reusable TUI components for [ratatui](https://ratatui.rs/),
//! the Rust terminal UI library.
//!
//! ## Overview
//!
//! This crate provides production-ready, reusable widgets for building terminal user interfaces:
//!
//! - **Layout Components**: [`ResizableSplit`], [`MasterLayout`] for flexible UI layouts
//! - **UI Components**: [`Button`], [`Dialog`], [`Toast`], [`Pane`] for common UI elements
//! - **Widgets**: [`TreeView`], [`ClickableScrollbar`], [`FuzzyFinder`] for data display
//! - **Navigation**: [`MenuBar`], [`HotkeyFooter`], [`StatusBar`] for navigation aids
//! - **Rendering**: [`render_markdown`] for markdown to ratatui text conversion
//! - **Terminal**: [`TermTui`] for embedded terminal emulation
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
//! | `statusbar` | Yes | Status bar components |
//! | `hotkey` | Yes | Hotkey footer and modal |
//! | `terminal` | No | Terminal emulator (TermTui) |
//! | `fuzzy` | No | Fuzzy finder component |
//! | `master-layout` | No | Full application layout framework |
//! | `file-tree` | No | File system tree with devicons |
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
pub mod button;
pub mod clickable_scrollbar;
pub mod pane;

// Feature-gated components
#[cfg(feature = "dialog")]
#[cfg_attr(docsrs, doc(cfg(feature = "dialog")))]
pub mod dialog;

#[cfg(feature = "toast")]
#[cfg_attr(docsrs, doc(cfg(feature = "toast")))]
pub mod toast;

#[cfg(feature = "split")]
#[cfg_attr(docsrs, doc(cfg(feature = "split")))]
pub mod resizable_split;

#[cfg(feature = "tree")]
#[cfg_attr(docsrs, doc(cfg(feature = "tree")))]
pub mod tree_view;

#[cfg(feature = "menu")]
#[cfg_attr(docsrs, doc(cfg(feature = "menu")))]
pub mod menu_bar;

#[cfg(feature = "statusbar")]
#[cfg_attr(docsrs, doc(cfg(feature = "statusbar")))]
pub mod statusbar;

#[cfg(feature = "statusbar")]
#[cfg_attr(docsrs, doc(cfg(feature = "statusbar")))]
pub mod statusline_stacked;

#[cfg(feature = "hotkey")]
#[cfg_attr(docsrs, doc(cfg(feature = "hotkey")))]
pub mod hotkey_footer;

#[cfg(feature = "hotkey")]
#[cfg_attr(docsrs, doc(cfg(feature = "hotkey")))]
pub mod hotkey_modal;

#[cfg(feature = "markdown")]
#[cfg_attr(docsrs, doc(cfg(feature = "markdown")))]
pub mod markdown_renderer;

#[cfg(feature = "terminal")]
#[cfg_attr(docsrs, doc(cfg(feature = "terminal")))]
pub mod termtui;

#[cfg(feature = "fuzzy")]
#[cfg_attr(docsrs, doc(cfg(feature = "fuzzy")))]
pub mod fuzzy_finder;

#[cfg(feature = "file-tree")]
#[cfg_attr(docsrs, doc(cfg(feature = "file-tree")))]
pub mod file_system_tree;

#[cfg(feature = "master-layout")]
#[cfg_attr(docsrs, doc(cfg(feature = "master-layout")))]
pub mod master_layout;

// Re-export commonly used types - always available
pub use button::render_title_with_buttons::render_title_with_buttons;
pub use button::Button;
pub use clickable_scrollbar::{
    ClickableScrollbar, ClickableScrollbarState, ClickableScrollbarStateMouseExt,
    ClickableScrollbarStateScrollExt, ClickableScrollbarStatefulWidgetExt, ScrollbarEvent,
};
pub use pane::Pane;

// Feature-gated re-exports
#[cfg(feature = "dialog")]
pub use dialog::widget::DialogWidget;
#[cfg(feature = "dialog")]
pub use dialog::{Dialog, DialogType};

#[cfg(feature = "toast")]
pub use toast::methods::render_toasts::render_toasts;
#[cfg(feature = "toast")]
pub use toast::{Toast, ToastLevel, ToastManager};

#[cfg(feature = "split")]
pub use resizable_split::{ResizableSplit, SplitDirection};

#[cfg(feature = "tree")]
pub use tree_view::{
    get_visible_paths, NodeState, TreeKeyBindings, TreeNavigator, TreeNode, TreeView, TreeViewState,
};

#[cfg(feature = "menu")]
pub use menu_bar::{MenuBar, MenuItem};

#[cfg(feature = "statusbar")]
pub use statusbar::{StatusBar, StatusItem};

#[cfg(feature = "statusbar")]
pub use statusline_stacked::{
    OperationalMode, StatusLineStacked, StyledStatusLine, SLANT_BL_TR, SLANT_TL_BR,
};

#[cfg(feature = "hotkey")]
pub use hotkey_footer::{HotkeyFooter, HotkeyFooterBuilder, HotkeyItem};

#[cfg(feature = "hotkey")]
pub use hotkey_modal::{functions::render_hotkey_modal, Hotkey, HotkeyModalConfig, HotkeySection};

#[cfg(feature = "markdown")]
pub use markdown_renderer::{
    handle_mouse_event, render_markdown, render_markdown_interactive, render_markdown_with_style,
    MarkdownScrollManager, MarkdownStyle, MarkdownWidget,
};

#[cfg(feature = "terminal")]
pub use termtui::{TermTui, TermTuiKeyBindings};

#[cfg(feature = "fuzzy")]
pub use fuzzy_finder::FuzzyFinder;

#[cfg(feature = "file-tree")]
pub use file_system_tree::{FileSystemEntry, FileSystemTree, FileSystemTreeConfig};

#[cfg(feature = "master-layout")]
pub use master_layout::{
    EventResult, InteractionMode, MasterLayout, NavigationBar, PaneContent, PaneId, PaneLayout,
    Tab, TabButton,
};

/// Prelude module for convenient imports
///
/// # Example
///
/// ```rust
/// use ratatui_toolkit::prelude::*;
/// ```
pub mod prelude {
    // Core components
    pub use crate::button::render_title_with_buttons::render_title_with_buttons;
    pub use crate::button::Button;
    pub use crate::clickable_scrollbar::{
        ClickableScrollbar, ClickableScrollbarState, ClickableScrollbarStateMouseExt,
        ClickableScrollbarStateScrollExt, ClickableScrollbarStatefulWidgetExt, ScrollbarEvent,
    };
    pub use crate::pane::Pane;

    // Feature-gated components
    #[cfg(feature = "dialog")]
    pub use crate::dialog::widget::DialogWidget;
    #[cfg(feature = "dialog")]
    pub use crate::dialog::{Dialog, DialogType};

    #[cfg(feature = "toast")]
    pub use crate::toast::methods::render_toasts::render_toasts;
    #[cfg(feature = "toast")]
    pub use crate::toast::{Toast, ToastLevel, ToastManager};

    #[cfg(feature = "split")]
    pub use crate::resizable_split::{ResizableSplit, SplitDirection};

    #[cfg(feature = "tree")]
    pub use crate::tree_view::{
        get_visible_paths, NodeState, TreeKeyBindings, TreeNavigator, TreeNode, TreeView,
        TreeViewState,
    };

    #[cfg(feature = "menu")]
    pub use crate::menu_bar::{MenuBar, MenuItem};

    #[cfg(feature = "statusbar")]
    pub use crate::statusbar::{StatusBar, StatusItem};

    #[cfg(feature = "statusbar")]
    pub use crate::statusline_stacked::{
        OperationalMode, StatusLineStacked, StyledStatusLine, SLANT_BL_TR, SLANT_TL_BR,
    };

    #[cfg(feature = "hotkey")]
    pub use crate::hotkey_footer::{HotkeyFooter, HotkeyFooterBuilder, HotkeyItem};

    #[cfg(feature = "hotkey")]
    pub use crate::hotkey_modal::functions::render_hotkey_modal;
    #[cfg(feature = "hotkey")]
    pub use crate::hotkey_modal::Hotkey;
    #[cfg(feature = "hotkey")]
    pub use crate::hotkey_modal::HotkeyModalConfig;
    #[cfg(feature = "hotkey")]
    pub use crate::hotkey_modal::HotkeySection;

    #[cfg(feature = "markdown")]
    pub use crate::markdown_renderer::{
        handle_mouse_event, render_markdown, render_markdown_interactive,
        render_markdown_with_style, MarkdownScrollManager, MarkdownStyle, MarkdownWidget,
    };

    #[cfg(feature = "terminal")]
    pub use crate::termtui::{TermTui, TermTuiKeyBindings};

    #[cfg(feature = "fuzzy")]
    pub use crate::fuzzy_finder::FuzzyFinder;

    #[cfg(feature = "file-tree")]
    pub use crate::file_system_tree::{FileSystemEntry, FileSystemTree, FileSystemTreeConfig};

    #[cfg(feature = "master-layout")]
    pub use crate::master_layout::{
        EventResult, InteractionMode, MasterLayout, NavigationBar, PaneContent, PaneId, PaneLayout,
        Tab, TabButton,
    };
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
