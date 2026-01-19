//! A scrollable, interactive markdown widget.

mod constructors;
pub mod enums;
mod helpers;
mod methods;
mod traits;

pub use enums::MarkdownWidgetMode;
pub use methods::WidgetStateSync;

use crate::markdown_widget::extensions::scrollbar::ScrollbarConfig;
use crate::markdown_widget::extensions::toc::TocConfig;
use crate::markdown_widget::foundation::types::GitStats;
use crate::markdown_widget::state::{
    CacheState, CollapseState, DisplaySettings, DoubleClickState, ExpandableState, GitStatsState,
    ScrollState, SelectionState, SourceState, TocState, VimState,
};

/// A scrollable, interactive markdown widget.
///
/// This widget renders markdown content with:
/// - Scroll support (keyboard and mouse)
/// - Click-to-highlight line selection
/// - Clickable headings to collapse/expand sections
/// - Clickable frontmatter to collapse/expand
/// - Expandable content blocks ("Show more"/"Show less")
/// - Text selection and copy support (drag to select)
/// - Double-click detection
/// - Statusline showing mode and scroll position
///
/// The widget handles ALL event processing internally and returns `MarkdownEvent`
/// variants so the parent application can react appropriately.
///
/// # Mouse Capture Requirement
///
/// For click events to work (line highlighting, TOC navigation, text selection),
/// you must enable mouse capture in your terminal setup:
///
/// ```rust,ignore
/// use crossterm::{
///     event::{EnableMouseCapture, DisableMouseCapture},
///     execute,
///     terminal::{EnterAlternateScreen, LeaveAlternateScreen},
/// };
///
/// // On startup:
/// execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
///
/// // On cleanup:
/// execute!(stdout, LeaveAlternateScreen, DisableMouseCapture)?;
/// ```
///
/// Without `EnableMouseCapture`, scroll wheel events may still work (terminal-dependent),
/// but click events will not be received by the application.
pub struct MarkdownWidget<'a> {
    /// The markdown content to render.
    pub(crate) content: &'a str,
    /// Scroll state (position, viewport, current line).
    pub(crate) scroll: &'a mut ScrollState,
    /// Content source state.
    pub(crate) source: &'a mut SourceState,
    /// Render cache state.
    pub(crate) cache: &'a mut CacheState,
    /// Display settings (line numbers, themes).
    pub(crate) display: &'a DisplaySettings,
    /// Section collapse state.
    pub(crate) collapse: &'a mut CollapseState,
    /// Expandable content state.
    pub(crate) expandable: &'a mut ExpandableState,
    /// Git stats state.
    pub(crate) git_stats_state: &'a mut GitStatsState,
    /// Vim keybinding state.
    pub(crate) vim: &'a mut VimState,
    /// Selection state for text selection/copy.
    pub(crate) selection: &'a mut SelectionState,
    /// Double-click state for double-click detection.
    pub(crate) double_click: &'a mut DoubleClickState,
    /// Optional TOC state for table of contents.
    pub(crate) toc_state: Option<&'a TocState>,
    /// When true, use stale cache for smoother resize during drag operations.
    pub(crate) is_resizing: bool,
    /// Current mode for the statusline.
    pub(crate) mode: MarkdownWidgetMode,
    /// Whether to show the statusline.
    pub(crate) show_statusline: bool,
    /// Whether to show the scrollbar.
    pub(crate) show_scrollbar: bool,
    /// Configuration for the scrollbar.
    pub(crate) scrollbar_config: ScrollbarConfig,
    /// Whether selection mode is active (affects statusline mode display).
    pub(crate) selection_active: bool,
    /// Git statistics for the file (optional, from git_stats_state).
    pub(crate) git_stats: Option<GitStats>,
    /// Whether to show the TOC.
    pub(crate) show_toc: bool,
    /// Configuration for the TOC.
    pub(crate) toc_config: TocConfig,
    /// Whether the TOC is currently hovered (expands to show text).
    pub(crate) toc_hovered: bool,
    /// Index of the hovered TOC entry.
    pub(crate) toc_hovered_entry: Option<usize>,
    /// Scroll offset for the TOC list.
    pub(crate) toc_scroll_offset: usize,
    /// Cached rendered lines for selection text extraction.
    pub(crate) rendered_lines: Vec<ratatui::text::Line<'static>>,
    /// Optional application theme for styling.
    pub(crate) app_theme: Option<&'a crate::services::theme::AppTheme>,
    /// Last double-click info (line number, kind, content) for app to retrieve.
    pub(crate) last_double_click: Option<(usize, String, String)>,
}
