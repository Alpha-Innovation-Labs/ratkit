//! Shared services and utilities for ratatui-toolkit.
//!
//! This module contains reusable infrastructure that can be used across
//! multiple components in the toolkit.
//!
//! ## Services
//!
//! - [`file_watcher`] - File system change detection
//! - [`theme`] - Theming system (requires `theme` feature)

pub mod file_watcher;

#[cfg(feature = "theme")]
pub mod theme;
