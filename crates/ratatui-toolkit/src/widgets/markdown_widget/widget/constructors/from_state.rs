//! Construct MarkdownWidget from a unified MarkdownState.

use crate::widgets::markdown_widget::extensions::scrollbar::ScrollbarConfig;
use crate::widgets::markdown_widget::extensions::toc::TocConfig;
use crate::widgets::markdown_widget::state::markdown_state::MarkdownState;
use crate::widgets::markdown_widget::widget::enums::MarkdownWidgetMode;
use crate::widgets::markdown_widget::widget::MarkdownWidget;

impl<'a> MarkdownWidget<'a> {
    /// Create a new MarkdownWidget from a unified MarkdownState.
    ///
    /// This constructor clones the state into the widget, allowing the widget
    /// to own its state internally without holding references to the original state.
    ///
    /// # Arguments
    ///
    /// * `content` - The markdown content to render
    /// * `state` - The unified markdown state containing all component states
    ///
    /// # Returns
    ///
    /// A new `MarkdownWidget` instance that owns its state.
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
    /// let widget = MarkdownWidget::from_state(&state);
    /// ```
    pub fn from_state(state: &'a MarkdownState) -> Self {
        let content = state.content().to_string();
        let rendered_lines = state
            .cache
            .render
            .as_ref()
            .map(|c| c.lines.clone())
            .unwrap_or_else(|| state.rendered_lines.clone());

        let mode = if state.filter_mode {
            MarkdownWidgetMode::Filter
        } else {
            MarkdownWidgetMode::Normal
        };

        Self {
            content,
            scroll: state.scroll.clone(),
            source: state.source.clone(),
            cache: state.cache.clone(),
            display: state.display.clone(),
            collapse: state.collapse.clone(),
            expandable: state.expandable.clone(),
            git_stats_state: state.git_stats.clone(),
            vim: state.vim.clone(),
            selection: state.selection.clone(),
            double_click: state.double_click.clone(),
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
            has_pane: true,
            pane: None,
            pane_title: None,
            pane_color: None,
            inner_area: None,
        }
    }
}
