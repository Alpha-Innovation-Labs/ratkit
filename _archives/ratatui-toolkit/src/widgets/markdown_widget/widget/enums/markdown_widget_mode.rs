//! Mode enum for the markdown widget statusline.

/// Mode for the markdown widget statusline.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum MarkdownWidgetMode {
    /// Normal viewing mode.
    #[default]
    Normal,
    /// Drag/selection mode.
    Drag,
    /// Filter mode (search/filter document).
    Filter,
}
