//! State module for markdown widget.

use std::collections::HashMap;
use std::path::PathBuf;

use crate::foundation::elements::MarkdownElement;
use crate::foundation::elements::TextSegment;
use crate::foundation::GitStats;

#[derive(Debug, Clone, Default)]
pub struct ScrollState {
    pub vertical_scroll: usize,
    pub horizontal_scroll: usize,
    pub viewport_height: usize,
    pub viewport_width: usize,
    pub content_height: usize,
}

impl ScrollState {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn scroll_up(&mut self, amount: usize) {
        self.vertical_scroll = self.vertical_scroll.saturating_sub(amount);
    }

    pub fn scroll_down(&mut self, amount: usize) {
        self.vertical_scroll = self.vertical_scroll.saturating_add(amount);
    }
}

#[derive(Debug, Clone, Default)]
pub struct SourceState {
    pub content: String,
    pub file_path: Option<PathBuf>,
}

impl SourceState {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set_content(&mut self, content: &str) {
        self.content = content.to_string();
    }
}

#[derive(Debug, Clone, Default)]
pub struct CacheState {
    pub parsed_elements: Vec<MarkdownElement>,
    pub line_heights: Vec<usize>,
}

#[derive(Debug, Clone, Default)]
pub struct DisplaySettings {
    pub show_line_numbers: bool,
    pub syntax_highlighting: bool,
}

#[derive(Debug, Clone, Default)]
pub struct CollapseState {
    pub collapsed_headings: HashMap<usize, bool>,
}

#[derive(Debug, Clone, Default)]
pub struct ExpandableState {
    pub expandable_entries: HashMap<usize, ExpandableEntry>,
}

#[derive(Debug, Clone)]
pub struct ExpandableEntry {
    pub collapsed: bool,
    pub max_shown: usize,
    pub total: usize,
}

#[derive(Debug, Clone, Default)]
pub struct GitStatsState {
    pub additions: usize,
    pub deletions: usize,
}

#[derive(Debug, Clone, Default)]
pub struct VimState {
    pub mode: VimMode,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum VimMode {
    #[default]
    Normal,
    Insert,
    Visual,
}

#[derive(Debug, Clone, Default)]
pub struct SelectionState {
    pub start: Option<usize>,
    pub end: Option<usize>,
}

#[derive(Debug, Clone, Default)]
pub struct DoubleClickState {
    pub last_click_position: Option<(u16, u16)>,
    pub last_click_time: Option<std::time::SystemTime>,
}

#[derive(Debug, Clone, Default)]
pub struct TocState {
    pub entries: Vec<TocEntry>,
    pub selected_index: usize,
}

#[derive(Debug, Clone)]
pub struct TocEntry {
    pub level: usize,
    pub title: String,
    pub line_number: usize,
}

#[derive(Debug, Clone, Default)]
pub struct MarkdownState {
    pub scroll: ScrollState,
    pub source: SourceState,
    pub cache: CacheState,
    pub display: DisplaySettings,
    pub collapse: CollapseState,
    pub expandable: ExpandableState,
    pub git_stats: Option<GitStats>,
    pub vim: VimState,
    pub selection: SelectionState,
    pub double_click: DoubleClickState,
    pub toc: TocState,
}

impl MarkdownState {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn content(&self) -> &str {
        &self.source.content
    }
}

pub type ParsedCache = Vec<MarkdownElement>;
pub type RenderCache = HashMap<usize, Vec<TextSegment>>;
