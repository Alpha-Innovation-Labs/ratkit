//! Events module for markdown widget.

use ratatui::layout::Rect;
use ratatui::style::Style;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MarkdownEvent {
    Click { row: u16, col: u16 },
    DoubleClick { row: u16, col: u16 },
    ScrollUp,
    ScrollDown,
    ScrollLeft,
    ScrollRight,
    ScrollToLine(usize),
    Select { start_line: usize, end_line: usize },
    CopySelection,
    ToggleCollapsedHeading(usize),
    NavigateToTocEntry(usize),
    EnterVimMode,
    ExitVimMode,
    VimNormal,
    VimInsert,
    VimVisual,
    MoveCursorUp,
    MoveCursorDown,
    MoveCursorLeft,
    MoveCursorRight,
    Noop,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MarkdownDoubleClickEvent {
    None,
    ToggleCollapsed { line: usize },
    ToggleFrontmatter,
    SelectWord { line: usize, col: usize },
    SelectParagraph { line: usize },
}
