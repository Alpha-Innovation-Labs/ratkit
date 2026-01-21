//! Constructor for MarkdownWidget.

use crate::widgets::markdown_widget::extensions::scrollbar::ScrollbarConfig;
use crate::widgets::markdown_widget::extensions::toc::TocConfig;
use crate::widgets::markdown_widget::state::{
    CacheState, CollapseState, DisplaySettings, DoubleClickState, ExpandableState, GitStatsState,
    ScrollState, SelectionState, SourceState, VimState,
};
use crate::widgets::markdown_widget::widget::enums::MarkdownWidgetMode;
use crate::widgets::markdown_widget::widget::MarkdownWidget;

impl<'a> MarkdownWidget<'a> {
    /// Create a new MarkdownWidget with the given content and state managers.
    ///
    /// # Arguments
    ///
    /// * `content` - The markdown content to render
    /// * `scroll` - Scroll state (position, viewport, current line)
    /// * `source` - Content source state
    /// * `cache` - Render cache state
    /// * `display` - Display settings (line numbers, themes)
    /// * `collapse` - Section collapse state
    /// * `expandable` - Expandable content state
    /// * `git_stats_state` - Git stats state
    /// * `vim` - Vim keybinding state
    /// * `selection` - Selection state for text selection/copy
    /// * `double_click` - Double-click state for detection
    ///
    /// # Returns
    ///
    /// A new `MarkdownWidget` instance.
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        content: &'a str,
        scroll: &'a mut ScrollState,
        source: &'a mut SourceState,
        cache: &'a mut CacheState,
        display: &'a DisplaySettings,
        collapse: &'a mut CollapseState,
        expandable: &'a mut ExpandableState,
        git_stats_state: &'a mut GitStatsState,
        vim: &'a mut VimState,
        selection: &'a mut SelectionState,
        double_click: &'a mut DoubleClickState,
    ) -> Self {
        Self {
            content,
            scroll,
            source,
            cache,
            display,
            collapse,
            expandable,
            git_stats_state,
            vim,
            selection,
            double_click,
            toc_state: None,
            is_resizing: false,
            mode: MarkdownWidgetMode::Normal,
            show_statusline: true,
            show_scrollbar: false,
            scrollbar_config: ScrollbarConfig::default(),
            selection_active: false,
            git_stats: None,
            show_toc: false,
            toc_config: TocConfig::default(),
            toc_hovered: false,
            toc_hovered_entry: None,
            toc_scroll_offset: 0,
            rendered_lines: Vec::new(),
            app_theme: None,
            last_double_click: None,
            filter: None,
            filter_mode: false,
            bordered: false,
            has_pane: true,
            pane: None,
            pane_title: None,
            pane_color: None,
        }
    }
}
