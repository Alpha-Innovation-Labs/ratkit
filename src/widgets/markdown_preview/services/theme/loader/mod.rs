//! Theme loader module for parsing opencode theme JSON files.
//!
//! This module provides functionality to load themes from JSON files in the
//! opencode theme format, which uses a `defs` section for named color definitions
//! and a `theme` section with semantic color mappings.
//!
//! # JSON Format
//!
//! The opencode theme format consists of:
//!
//! ```json
//! {
//!   "defs": {
//!     "colorName": "#hexcode",
//!     ...
//!   },
//!   "theme": {
//!     "primary": { "dark": "colorName", "light": "colorName" },
//!     "error": "#ff0000",
//!     ...
//!   }
//! }
//! ```
//!
//! # Resolution
//!
//! Color values in the `theme` section can be:
//! - Direct hex colors: `"#ff0000"`
//! - References to defs: `"colorName"` (resolved from `defs`)
//! - Variant objects: `{ "dark": "value", "light": "value" }`
//!
//! # Example
//!
//! ```rust,ignore,no_run
//! use ratatui_toolkit::services::theme::{loader, ThemeVariant, AppTheme};
//!
//! // Load from file path
//! let theme = loader::load_theme_file("themes/gruvbox.json", ThemeVariant::Dark)
//!     .expect("Failed to load theme");
//!
//! // Load from JSON string
//! let json = r#"{"defs": {"bg": "#282828"}, "theme": {"background": "bg"}}"#;
//! let theme = loader::load_theme_str(json, ThemeVariant::Dark)
//!     .expect("Failed to parse theme");
//! ```

mod load_builtin;
mod load_theme_file;
mod load_theme_str;
mod parse_color;
mod resolve_defs;
mod theme_json;

pub use load_builtin::{load_builtin_theme, BUILTIN_THEMES};
pub use load_theme_file::load_theme_file;
pub use load_theme_str::load_theme_str;
pub use parse_color::parse_hex_color;
pub use resolve_defs::resolve_color_value;
pub use theme_json::{ColorValue, ThemeJson};
