//! Default implementation for MarkdownState.

use crate::widgets::markdown_widget::state::markdown_state::MarkdownState;
use crate::widgets::markdown_widget::state::{
    CacheState, CollapseState, DisplaySettings, DoubleClickState, ExpandableState, GitStatsState,
    ScrollState, SelectionState, SourceState, VimState,
};

impl Default for MarkdownState {
    fn default() -> Self {
        Self {
            scroll: ScrollState::default(),
            source: SourceState::default(),
            cache: CacheState::default(),
            display: DisplaySettings::default(),
            collapse: CollapseState::default(),
            expandable: ExpandableState::default(),
            git_stats: GitStatsState::default(),
            vim: VimState::default(),
            selection: SelectionState::default(),
            double_click: DoubleClickState::default(),
            toc_hovered: false,
            toc_hovered_entry: None,
            toc_scroll_offset: 0,
            selection_active: false,
            cached_git_stats: None,
            rendered_lines: Vec::new(),
            filter: None,
            filter_mode: false,
        }
    }
}
