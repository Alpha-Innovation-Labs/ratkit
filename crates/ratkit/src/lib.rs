//! # ratkit - Meta-crate for ratkit TUI components
//!
//! ⚠️ **DEPRECATION NOTICE**: This crate is part of the ratkit migration from `ratatui-toolkit`.
//! If you were using `ratatui-toolkit`, please migrate to the individual `ratkit-*` crates directly.
//!
//! This meta-crate re-exports all ratkit TUI components for convenient access.
//! Each component can also be used individually by depending on the specific `ratkit-*` crate.
//!
//! # Migration from ratatui-toolkit
//!
//! ```toml
//! # Old (deprecated):
//! ratatui-toolkit = "0.1"
//!
//! # New (recommended):
//! ratkit-button = "0.1"
//! ratkit-pane = "0.1"
//! # ... etc for each component you use
//! ```
//!
//! # Quick Start
//!
//! ```rust,no_run
//! use ratkit::{Button, Pane, Dialog};
//! use ratatui::prelude::*;
//!
//! fn main() {
//!     // Your app code here
//! }
//! ```
//!
//! # Feature Flags
//!
//! - `default`: All widgets and services
//! - `full`: Same as default
//! - `widgets`: All UI widgets
//! - `services`: All service components
//! - `theme`: Theme support
//! - Individual feature flags for each component

#![doc(html_root_url = "https://docs.rs/ratkit/0.1")]
#![warn(missing_docs, clippy::cargo)]
#![cfg_attr(doc, cfg(feature = "docsrs"))]

#[deprecated(
    since = "0.1.0",
    note = "ratkit is deprecated in favor of individual ratkit-* crates. \
            See https://github.com/alpha-innovation-labs/ratatui-toolkit for migration guide."
)]
// Core UI widgets
#[cfg(feature = "button")]
pub use ratkit_button::*;

#[cfg(feature = "pane")]
pub use ratkit_pane::*;

#[cfg(feature = "dialog")]
pub use ratkit_dialog::*;

#[cfg(feature = "toast")]
pub use ratkit_toast::*;

#[cfg(feature = "statusline")]
pub use ratkit_statusline::*;

#[cfg(feature = "scroll")]
pub use ratkit_scroll::*;

#[cfg(feature = "menu-bar")]
pub use ratkit_menu_bar::*;

#[cfg(feature = "resizable-grid")]
pub use ratkit_resizable_grid::*;

#[cfg(feature = "tree-view")]
pub use ratkit_tree_view::*;

#[cfg(feature = "widget-event")]
pub use ratkit_widget_event::*;

#[cfg(feature = "termtui")]
pub use ratkit_termtui::*;

// Advanced widgets
#[cfg(feature = "markdown-preview")]
pub use ratkit_markdown_preview::*;

#[cfg(feature = "code-diff")]
pub use ratkit_code_diff::*;

#[cfg(feature = "ai-chat")]
pub use ratkit_ai_chat::*;

#[cfg(feature = "hotkey-footer")]
pub use ratkit_hotkey_footer::*;

#[cfg(feature = "file-system-tree")]
pub use ratkit_file_system_tree::*;

#[cfg(feature = "theme-picker")]
pub use ratkit_theme_picker::*;

// Services
#[cfg(feature = "file-watcher")]
pub use ratkit_file_watcher::*;

#[cfg(feature = "git-watcher")]
pub use ratkit_git_watcher::*;

#[cfg(feature = "repo-watcher")]
pub use ratkit_repo_watcher::*;

#[cfg(feature = "hotkey-service")]
pub use ratkit_hotkey_service::*;

// Theme support
#[cfg(feature = "theme")]
pub use ratkit_theme::*;
