//! Widget module for markdown widget.

use crate::foundation::GitStats;
use crate::state::{
    CacheState, CollapseState, DisplaySettings, DoubleClickState, ExpandableState, GitStatsState,
    MarkdownState, ScrollState, SelectionState, SourceState, TocState, VimState,
};

pub mod enums;

pub use enums::MarkdownWidgetMode;

use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::style::Style;
use ratatui::widgets::Widget;

#[derive(Debug, Clone)]
pub struct MarkdownWidget<'a> {
    content: &'a str,
    scroll: ScrollState,
    source: SourceState,
    cache: CacheState,
    display: DisplaySettings,
    collapse: CollapseState,
    expandable: ExpandableState,
    git_stats: Option<GitStats>,
    vim: VimState,
    selection: SelectionState,
    double_click: DoubleClickState,
    toc: Option<TocState>,
    show_toc: bool,
}

impl<'a> MarkdownWidget<'a> {
    pub fn new(content: &'a str) -> Self {
        let mut source = SourceState::new();
        source.set_content(content);

        Self {
            content,
            scroll: ScrollState::new(),
            source,
            cache: CacheState::default(),
            display: DisplaySettings::default(),
            collapse: CollapseState::default(),
            expandable: ExpandableState::default(),
            git_stats: None,
            vim: VimState::default(),
            selection: SelectionState::default(),
            double_click: DoubleClickState::default(),
            toc: None,
            show_toc: false,
        }
    }

    pub fn from_state(content: &'a str, state: &mut MarkdownState) -> Self {
        state.source.set_content(content);
        Self {
            content,
            scroll: state.scroll.clone(),
            source: state.source.clone(),
            cache: state.cache.clone(),
            display: state.display.clone(),
            collapse: state.collapse.clone(),
            expandable: state.expandable.clone(),
            git_stats: state.git_stats.clone(),
            vim: state.vim.clone(),
            selection: state.selection.clone(),
            double_click: state.double_click.clone(),
            toc: Some(state.toc.clone()),
            show_toc: false,
        }
    }

    pub fn show_toc(mut self, show: bool) -> Self {
        self.show_toc = show;
        self
    }

    pub fn with_syntax_highlighting(mut self, enabled: bool) -> Self {
        self.display.syntax_highlighting = enabled;
        self
    }

    pub fn with_line_numbers(mut self, show: bool) -> Self {
        self.display.show_line_numbers = show;
        self
    }
}

impl Widget for MarkdownWidget<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let content = format!("Markdown: {}", self.content);
        let widget = ratatui::widgets::Paragraph::new(content);
        widget.render(area, buf);
    }
}
