//! Construct MarkdownWidget from a unified MarkdownState.

use crate::markdown_widget::extensions::scrollbar::ScrollbarConfig;
use crate::markdown_widget::extensions::toc::TocConfig;
use crate::markdown_widget::state::markdown_state::MarkdownState;
use crate::markdown_widget::widget::enums::MarkdownWidgetMode;
use crate::markdown_widget::widget::MarkdownWidget;

impl<'a> MarkdownWidget<'a> {
    /// Create a new MarkdownWidget from a unified MarkdownState.
    ///
    /// This is the preferred constructor as it simplifies state management
    /// by bundling all component states into a single struct.
    ///
    /// # Arguments
    ///
    /// * `content` - The markdown content to render (pass `state.content()`)
    /// * `state` - The unified markdown state containing all component states
    ///
    /// # Returns
    ///
    /// A new `MarkdownWidget` instance.
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
    /// let content = state.content().to_string();
    /// let widget = MarkdownWidget::from_state(&content, &mut state)
    ///     .show_toc(true)
    ///     .show_statusline(true);
    /// ```
    pub fn from_state(content: &'a str, state: &'a mut MarkdownState) -> Self {
        // Use cached rendered lines if available, otherwise use state's rendered_lines
        let rendered_lines = state
            .cache
            .render
            .as_ref()
            .map(|c| c.lines.clone())
            .unwrap_or_else(|| state.rendered_lines.clone());

        // Determine mode based on filter_mode state
        let mode = if state.filter_mode {
            MarkdownWidgetMode::Filter
        } else {
            MarkdownWidgetMode::Normal
        };

        Self {
            content,
            scroll: &mut state.scroll,
            source: &mut state.source,
            cache: &mut state.cache,
            display: &state.display,
            collapse: &mut state.collapse,
            expandable: &mut state.expandable,
            git_stats_state: &mut state.git_stats,
            vim: &mut state.vim,
            selection: &mut state.selection,
            double_click: &mut state.double_click,
            toc_state: None,
            is_resizing: false,
            mode,
            show_statusline: true,
            show_scrollbar: false,
            scrollbar_config: ScrollbarConfig::default(),
            selection_active: state.selection_active,
            git_stats: state.cached_git_stats,
            show_toc: false,
            toc_config: TocConfig::default(),
            toc_hovered: state.toc_hovered,
            toc_hovered_entry: state.toc_hovered_entry,
            toc_scroll_offset: state.toc_scroll_offset,
            rendered_lines,
            app_theme: None,
            last_double_click: None,
            filter: state.filter.clone(),
            filter_mode: state.filter_mode,
            bordered: false,
        }
    }
}
