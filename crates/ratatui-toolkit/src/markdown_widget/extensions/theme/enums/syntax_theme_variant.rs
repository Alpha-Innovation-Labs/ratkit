//! Theme variant for syntax highlighting.

/// Theme variant for syntax highlighting.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SyntaxThemeVariant {
    /// Dark theme variant (default).
    #[default]
    Dark,
    /// Light theme variant.
    Light,
}
