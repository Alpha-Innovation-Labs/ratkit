//! Predefined color palettes for common themes.
//!
//! This module provides ready-to-use [`ColorPalette`] instances for popular
//! color schemes. These can be used directly or as a starting point for
//! custom themes.
//!
//! # Available Palettes
//!
//! * [`dark_default`] - Based on One Dark Pro / base16-ocean.dark
//! * [`light_default`] - Based on GitHub Light theme
//! * [`opencode_dark`] - OpenCode dark theme
//!
//! # Example
//!
//! ```rust,ignore
//! use ratatui_toolkit::markdown_widget::extensions::theme::palettes;
//!
//! let palette = palettes::dark_default();
//! let blue = palette.get_or_default("blue");
//! ```

mod dark_default;
mod light_default;
mod opencode_dark;

pub use dark_default::dark_default;
pub use light_default::light_default;
pub use opencode_dark::opencode_dark;
