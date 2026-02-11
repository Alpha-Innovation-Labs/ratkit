//! Unified state management for the markdown widget.
//!
//! `MarkdownState` bundles all component states into a single struct,
//! simplifying widget construction and state management.

use crate::widgets::markdown_preview::widgets::markdown_widget::foundation::types::GitStats;
use crate::widgets::markdown_preview::widgets::markdown_widget::state::{
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
#[derive(Default, Clone)]
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
    /// Current filter text (when in filter mode).
    pub filter: Option<String>,
    /// Whether filter mode is currently active.
    pub filter_mode: bool,
    /// Inner area for mouse event handling (set during render).
    inner_area: ratatui::layout::Rect,
}

impl MarkdownState {
    /// Get the inner area for mouse event handling.
    ///
    /// Returns the inner area that was set during the last render.
    pub fn inner_area(&self) -> ratatui::layout::Rect {
        self.inner_area
    }

    /// Set the inner area for mouse event handling.
    ///
    /// This is typically called by the widget during render.
    pub fn set_inner_area(&mut self, area: ratatui::layout::Rect) {
        self.inner_area = area;
    }

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

    /// Reload file content if the watcher detected changes.
    ///
    /// Returns `true` when content changed and caches were invalidated.
    pub fn reload_source_if_changed(&mut self) -> std::io::Result<bool> {
        if self.source.reload_if_changed()? {
            self.cache.invalidate();
            self.rendered_lines.clear();
            Ok(true)
        } else {
            Ok(false)
        }
    }
}

/// Default implementation for MarkdownState.
///
/// This module is kept for compatibility but Default is now derived
/// on the MarkdownState struct directly.

/// Constructor for MarkdownState.

impl MarkdownState {
    /// Create a new MarkdownState with all default values.
    ///
    /// This is equivalent to `MarkdownState::default()`.
    pub fn new() -> Self {
        Self::default()
    }

    /// Create a new MarkdownState with custom display settings.
    pub fn with_display(display: DisplaySettings) -> Self {
        Self {
            display,
            ..Default::default()
        }
    }
}
