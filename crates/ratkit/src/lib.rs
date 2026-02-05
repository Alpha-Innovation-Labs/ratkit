//! # ratkit - Core runtime for ratkit TUI components
//!
//! ratkit provides the core runtime (Runner + Layout Manager) and optional
//! re-exports for ratkit TUI components. Enable only the features you need or
//! use the `all` feature for the full bundle.
//!
//! # Installation
//!
//! ```toml
//! [dependencies]
//! ratkit = "0.1"
//! ```
//!
//! For the core runtime only:
//!
//! ```toml
//! ratkit = "0.1"
//! ```
//!
//! For selected components:
//!
//! ```toml
//! ratkit = { version = "0.1", default-features = false, features = ["tree-view", "toast"] }
//! ```
//!
//! For the full bundle:
//!
//! ```toml
//! ratkit = { version = "0.1", features = ["all"] }
//! ```
//!
//! # Quick Start
//!
//! ```rust,no_run
//! use ratkit::{Button, Dialog, Pane};
//! use ratatui::prelude::*;
//!
//! fn main() {
//!     // Your app code here
//! }
//! ```
//!
//! # Feature Flags
//!
//! - `default`: Core runtime only (Runner + Layout Manager)
//! - `all`: All widgets and services
//! - `full`: Alias for `all`
//! - `widgets`: All UI widgets
//! - `services`: All service components
//! - Individual feature flags for each component

#![doc(html_root_url = "https://docs.rs/ratkit/0.1")]
#![warn(missing_docs, clippy::cargo)]
#![cfg_attr(doc, cfg(feature = "docsrs"))]

mod coordinator;
mod error;
mod events;
mod focus;
mod layout;
mod mouse_router;
mod registry;
mod types;

/// Core runtime pieces for ratkit.
pub mod core;

pub use core::{
    CoordinatorAction, CoordinatorApp, CoordinatorConfig, CoordinatorEvent, Element, ElementHandle,
    ElementId, ElementMetadata, ElementRegistry, FocusManager, FocusRequest, KeyboardEvent,
    LayoutCoordinator, LayoutError, LayoutManager, LayoutResult, LayoutStats, MouseEvent,
    MouseRouter, MouseRouterConfig, Region, ResizeEvent, Runner, RunnerAction, RunnerConfig,
    RunnerEvent, TickEvent, Visibility,
};

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
