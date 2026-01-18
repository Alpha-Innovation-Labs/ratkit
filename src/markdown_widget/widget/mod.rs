//! A scrollable, interactive markdown widget.

mod constructors;
pub mod enums;
mod methods;
mod traits;

pub use enums::MarkdownWidgetMode;

use crate::markdown_widget::extensions::minimap::MinimapConfig;
use crate::markdown_widget::extensions::toc::TocConfig;
use crate::markdown_widget::foundation::types::GitStats;
use crate::markdown_widget::state::double_click_state::DoubleClickState;
use crate::markdown_widget::state::scroll_manager::MarkdownScrollManager;
use crate::markdown_widget::state::selection_state::SelectionState;
use crate::markdown_widget::state::toc_state::TocState;

/// A scrollable, interactive markdown widget.
///
/// This widget renders markdown content with:
/// - Scroll support (keyboard and mouse)
/// - Clickable headings to collapse/expand sections
/// - Clickable frontmatter to collapse/expand
/// - Expandable content blocks ("Show more"/"Show less")
/// - Text selection and copy support
/// - Double-click detection
/// - Statusline showing mode and scroll position
///
/// The widget handles ALL event processing internally and returns `MarkdownEvent`
/// variants so the parent application can react appropriately.
pub struct MarkdownWidget<'a> {
    /// The markdown content to render.
    pub(crate) content: &'a str,
    /// The scroll manager for handling scroll state.
    pub(crate) scroll: &'a mut MarkdownScrollManager,
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
    /// Whether selection mode is active (affects statusline mode display).
    pub(crate) selection_active: bool,
    /// Git statistics for the file (optional).
    pub(crate) git_stats: Option<GitStats>,
    /// Whether to show the minimap.
    pub(crate) show_minimap: bool,
    /// Configuration for the minimap.
    pub(crate) minimap_config: MinimapConfig,
    /// Whether the minimap is currently hovered.
    pub(crate) minimap_hovered: bool,
    /// Whether to show the TOC (replaces minimap).
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
}
