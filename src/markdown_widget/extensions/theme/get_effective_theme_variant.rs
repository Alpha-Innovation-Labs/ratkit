//! Function to resolve the effective theme variant.

use crate::markdown_widget::extensions::theme::ThemeVariant;

/// Get the effective color scheme based on variant and terminal detection.
///
/// Resolves a [`ThemeVariant`] to its effective value. For `Dark` and `Light`
/// variants, returns them unchanged. For `Auto`, attempts to detect the
/// terminal's color scheme.
///
/// # Arguments
///
/// * `variant` - The [`ThemeVariant`] to resolve
///
/// # Returns
///
/// The effective [`ThemeVariant`] after resolution. Note that `Auto` will
/// be resolved to either `Dark` or `Light`.
///
/// # Features
///
/// When the `termenv` feature is enabled, auto-detection will use the
/// terminal environment to determine the color scheme. Without this feature,
/// `Auto` defaults to `Dark`.
///
/// # Example
///
/// ```rust,ignore
/// use ratatui_toolkit::markdown_widget::extensions::theme::{ThemeVariant, get_effective_theme_variant};
///
/// // Explicit variant is returned unchanged
/// assert_eq!(
///     get_effective_theme_variant(ThemeVariant::Dark),
///     ThemeVariant::Dark
/// );
///
/// // Auto resolves to Dark or Light based on terminal
/// let effective = get_effective_theme_variant(ThemeVariant::Auto);
/// assert!(matches!(effective, ThemeVariant::Dark | ThemeVariant::Light));
/// ```
#[allow(unexpected_cfgs)]
pub fn get_effective_theme_variant(variant: ThemeVariant) -> ThemeVariant {
    match variant {
        ThemeVariant::Auto => {
            // Simple terminal detection: check for dark terminal indicators
            // This is a basic implementation that can be enhanced
            #[cfg(feature = "termenv")]
            {
                use termenv::Config;
                let config = Config::default();
                if config.profile() == Some(termenv::Profile::Dark) {
                    ThemeVariant::Dark
                } else {
                    ThemeVariant::Light
                }
            }
            #[cfg(not(feature = "termenv"))]
            {
                // Default to dark mode if termenv is not available
                ThemeVariant::Dark
            }
        }
        _ => variant,
    }
}
