#![cfg_attr(docsrs, feature(doc_cfg))]

//! # markdown-viewer
//!
//! A full-featured markdown viewer application component for ratatui.
//!
//! This crate provides a ready-to-use viewer with:
//! - File tree navigation (filtered for .md files)
//! - Markdown content rendering
//! - Resizable split panes
//! - State persistence

pub mod config;
pub mod event;
pub mod markdown_panel;
pub mod persistence;
pub mod resizable_panes;
pub mod state;
pub mod viewer;

pub use config::ViewerConfig;
pub use event::ViewerEvent;
pub use markdown_panel::MarkdownPanel;
pub use resizable_panes::{ResizablePanes, ResizablePanesState};
pub use state::ViewerState;
pub use viewer::Viewer;

// Re-export types from ratatui-toolkit
pub use ratatui_toolkit::{render_markdown, FileSystemEntry, FileSystemTree, FileSystemTreeConfig};

/// Prelude module for convenient imports
pub mod prelude {
    pub use crate::config::ViewerConfig;
    pub use crate::event::ViewerEvent;
    pub use crate::markdown_panel::MarkdownPanel;
    pub use crate::resizable_panes::{ResizablePanes, ResizablePanesState};
    pub use crate::state::ViewerState;
    pub use crate::viewer::Viewer;

    pub use ratatui_toolkit::{render_markdown, FileSystemEntry, FileSystemTree, FileSystemTreeConfig};
}
