//! Unified state management for the markdown widget.
//!
//! `MarkdownState` bundles all component states into a single struct,
//! simplifying widget construction and state management.

mod constructors;

use crate::markdown_widget::foundation::types::GitStats;
use crate::markdown_widget::state::{
    CacheState, CollapseState, DisplaySettings, DoubleClickState, ExpandableState, GitStatsState,
    ScrollState, SelectionState, SourceState, VimState,
};

/// Unified state for the markdown widget.
///
/// This struct bundles all component states together, making it easier to
/// manage widget state without passing many individual references.
///
/// # Example
///
/// ```rust,ignore
/// use ratatui_toolkit::markdown_widget::state::MarkdownState;
/// use ratatui_toolkit::MarkdownWidget;
///
/// let mut state = MarkdownState::default();
/// state.source.set_content("# Hello World");
///
/// let widget = MarkdownWidget::from_state(&mut state)
///     .show_toc(true)
///     .show_statusline(true);
/// ```
pub struct MarkdownState {
    /// Core scroll state (position, viewport, current line).
    pub scroll: ScrollState,
    /// Content source state.
    pub source: SourceState,
    /// Render cache state.
    pub cache: CacheState,
    /// Display settings (line numbers, themes).
    pub display: DisplaySettings,
    /// Section collapse state.
    pub collapse: CollapseState,
    /// Expandable content state.
    pub expandable: ExpandableState,
    /// Git stats state.
    pub git_stats: GitStatsState,
    /// Vim keybinding state.
    pub vim: VimState,
    /// Selection state for text selection/copy.
    pub selection: SelectionState,
    /// Double-click detection state.
    pub double_click: DoubleClickState,
    /// Whether the TOC is currently hovered.
    pub toc_hovered: bool,
    /// Index of the hovered TOC entry.
    pub toc_hovered_entry: Option<usize>,
    /// Scroll offset for the TOC list.
    pub toc_scroll_offset: usize,
    /// Whether selection mode is active.
    pub selection_active: bool,
    /// Git statistics for the file (cached from git_stats state).
    pub cached_git_stats: Option<GitStats>,
    /// Cached rendered lines for selection text extraction.
    /// This persists between frames so mouse events can access line data.
    pub rendered_lines: Vec<ratatui::text::Line<'static>>,
}

impl MarkdownState {
    /// Get the content from the source state.
    ///
    /// Returns the content if set, or an empty string.
    pub fn content(&self) -> &str {
        self.source.content().unwrap_or("")
    }

    /// Update git stats from the source path.
    ///
    /// This should be called periodically to refresh git information.
    pub fn update_git_stats(&mut self) {
        self.git_stats.update(self.source.source_path());
        self.cached_git_stats = self.git_stats.git_stats();
    }
}
