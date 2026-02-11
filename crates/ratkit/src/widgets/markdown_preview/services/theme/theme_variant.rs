/// Theme variant module for light and dark mode support.
///
/// This module provides the [`ThemeVariant`] enum which represents whether
/// to use dark or light mode colors from a theme definition.
///
/// # Example
///
/// ```rust,ignore
/// use ratatui_toolkit::services::theme::ThemeVariant;
///
/// let variant = ThemeVariant::Dark;
/// assert_eq!(variant, ThemeVariant::Dark);
///
/// let light = ThemeVariant::Light;
/// assert_eq!(light, ThemeVariant::Light);
/// ```

/// Represents the color scheme variant for a theme.
///
/// Themes typically define two sets of colors - one optimized for dark
/// backgrounds and one for light backgrounds. This enum allows selecting
/// which set to use.
///
/// # Variants
///
/// * `Dark` - Use colors optimized for dark backgrounds (light text on dark)
/// * `Light` - Use colors optimized for light backgrounds (dark text on light)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum ThemeVariant {
    /// Dark mode colors - light text on dark background.
    ///
    /// This is typically the default for terminal applications.
    #[default]
    Dark,

    /// Light mode colors - dark text on light background.
    ///
    /// Suitable for terminals with light backgrounds or when
    /// matching a light system theme.
    Light,
}

/// Display trait implementation for [`ThemeVariant`].
use std::fmt;

impl fmt::Display for ThemeVariant {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ThemeVariant::Dark => write!(f, "dark"),
            ThemeVariant::Light => write!(f, "light"),
        }
    }
}
