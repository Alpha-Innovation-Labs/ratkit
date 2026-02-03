//! Theme variant selection enum.
//!
//! The [`ThemeVariant`] enum allows users to explicitly select dark or light
//! theme, or let the system auto-detect based on terminal settings.

/// Theme variant selection.
///
/// Controls which color scheme to use for rendering. The `Auto` variant
/// will attempt to detect the terminal's color scheme, falling back to
/// dark mode if detection is not available.
///
/// # Variants
///
/// * `Dark` - Use dark mode colors (light text on dark background)
/// * `Light` - Use light mode colors (dark text on light background)
/// * `Auto` - Detect from terminal settings (requires `termenv` feature)
///
/// # Example
///
/// ```rust,ignore
/// use ratatui_toolkit::markdown_widget::extensions::theme::{ThemeVariant, get_effective_theme_variant};
///
/// // Explicitly use dark mode
/// let variant = ThemeVariant::Dark;
///
/// // Auto-detect (falls back to dark if detection unavailable)
/// let auto_variant = ThemeVariant::Auto;
/// let effective = get_effective_theme_variant(auto_variant);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ThemeVariant {
    /// Use dark mode colors (light text on dark background).
    #[default]
    Dark,

    /// Use light mode colors (dark text on light background).
    Light,

    /// Detect from terminal settings.
    ///
    /// When the `termenv` feature is enabled, this will attempt to detect
    /// the terminal's color scheme. Without the feature, defaults to `Dark`.
    Auto,
}
