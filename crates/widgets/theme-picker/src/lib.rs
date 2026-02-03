//! Theme picker widget for ratatui.
//!
//! A centered modal dialog with search filtering and live preview for selecting themes.
//!
//! # Features
//!
//! - Search/filter themes by name
//! - Keyboard navigation (j/k/Up/Down)
//! - Live theme preview as you navigate
//! - Enter to confirm, Esc to cancel
//!
//! # Example
//!
//! ```rust,no_run
//! use ratkit_theme_picker::{ThemePicker, ThemePickerEvent, ThemeColors};
//!
//! let mut picker = ThemePicker::new();
//! picker.show();
//! ```

mod state;
mod theme_colors;
mod picker;
mod builtin_themes;

pub use builtin_themes::BUILTIN_THEMES;
pub use picker::ThemePicker;
pub use state::{ThemePickerState, ThemePickerStateSnapshot};
pub use theme_colors::ThemeColors;
