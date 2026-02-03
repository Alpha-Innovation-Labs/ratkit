//! Enums module for markdown widget.

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum MarkdownWidgetMode {
    #[default]
    Navigation,
    Selection,
}
