//! Comprehensive theme system for ratatui-toolkit widgets.
//!
//! This module provides a complete theming solution for TUI applications,
//! with support for loading themes from JSON files in the opencode format.
//!
//! # Overview
//!
//! The theme system consists of:
//!
//! - [`AppTheme`] - The main theme struct with all widget colors
//! - [`ThemeVariant`] - Dark/light mode selection
//! - [`DiffColors`] - Colors for CodeDiff widget
//! - [`MarkdownColors`] - Colors for MarkdownWidget
//! - [`SyntaxColors`] - Colors for syntax highlighting
//! - [`loader`] - JSON theme file loading utilities
//!
//! # Builtin Themes
//!
//! The crate includes 33 builtin themes that can be loaded by name:
//!
//! ```rust
//! use ratatui_toolkit::theme::{loader, ThemeVariant};
//!
//! // Load a builtin theme
//! let theme = loader::load_builtin_theme("gruvbox", ThemeVariant::Dark)
//!     .expect("Failed to load theme");
//!
//! // See all available themes
//! for name in loader::BUILTIN_THEMES {
//!     println!("{}", name);
//! }
//! ```
//!
//! # Custom Themes
//!
//! You can create custom themes from JSON files or strings:
//!
//! ```rust
//! use ratatui_toolkit::theme::{AppTheme, ThemeVariant};
//!
//! let json = r#"{
//!   "defs": {
//!     "myPrimary": "#ff6600"
//!   },
//!   "theme": {
//!     "primary": { "dark": "myPrimary", "light": "myPrimary" }
//!   }
//! }"#;
//!
//! let theme = AppTheme::from_json(json, ThemeVariant::Dark)
//!     .expect("Failed to parse theme");
//! ```
//!
//! # JSON Format
//!
//! The opencode theme format uses:
//!
//! - `defs`: Named color definitions (e.g., `"darkBg0": "#282828"`)
//! - `theme`: Semantic mappings with variant support
//!
//! ```json
//! {
//!   "defs": {
//!     "colorName": "#hexcode"
//!   },
//!   "theme": {
//!     "primary": { "dark": "colorName", "light": "colorName" },
//!     "error": "#ff0000"
//!   }
//! }
//! ```
//!
//! # Usage with Widgets
//!
//! ```rust,no_run
//! use ratatui::style::Style;
//! use ratatui_toolkit::theme::AppTheme;
//!
//! let theme = AppTheme::default();
//!
//! // Use UI colors
//! let primary_style = Style::default().fg(theme.primary);
//! let error_style = Style::default().fg(theme.error);
//!
//! // Use diff colors
//! let added_style = Style::default().fg(theme.diff.added);
//!
//! // Use markdown colors
//! let heading_style = Style::default().fg(theme.markdown.heading);
//! ```

pub mod app_theme;
pub mod diff_colors;
pub mod loader;
pub mod markdown_colors;
pub mod persistence;
pub mod syntax_colors;
pub mod theme_variant;

// Re-export main types at module level
pub use app_theme::AppTheme;
pub use diff_colors::DiffColors;
pub use markdown_colors::MarkdownColors;
pub use syntax_colors::SyntaxColors;
pub use theme_variant::ThemeVariant;
