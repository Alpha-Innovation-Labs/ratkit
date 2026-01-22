//! Application state for the showcase demo.

mod constructors;
mod methods;

use ratatui::layout::Rect;
use ratatui_toolkit::services::file_watcher::FileWatcher;
use ratatui_toolkit::{
    AppTheme, CodeDiff, DoubleClickState, FileSystemTree, InputState, MenuBar, MessageRole,
    MessageStore, ResizableGrid, SelectionState, TermTui, ToastManager, TreeNavigator,
    TreeViewState,
};
use ratatui_toolkit::{
    CacheState, CollapseState, DisplaySettings, ExpandableState, GitStatsState, ScrollState,
    SourceState, VimState,
};
use std::time::Instant;

use super::demo_tab::DemoTab;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum TreePaneFocus {
    FileTree,
    ComponentTree,
}

/// Main application state for the showcase.
pub struct App {
    // Navigation
    pub current_tab: DemoTab,
    pub menu_bar: MenuBar,

    // Code diff demo - now just a single CodeDiff widget with integrated sidebar
    pub code_diff: CodeDiff,

    // File system tree demo
    pub file_tree: Option<FileSystemTree<'static>>,
    pub file_tree_state: TreeViewState,
    pub file_tree_navigator: TreeNavigator,

    // Tree demo
    pub tree_state: TreeViewState,
    pub tree_navigator: TreeNavigator,
    pub tree_focus: TreePaneFocus,

    // Dialog demo
    pub show_dialog: bool,
    pub dialog_type: ratatui_toolkit::DialogType,

    // Markdown demo - new focused state modules
    pub markdown_scroll: ScrollState,
    pub markdown_source: SourceState,
    pub markdown_cache: CacheState,
    pub markdown_display: DisplaySettings,
    pub markdown_collapse: CollapseState,
    pub markdown_expandable: ExpandableState,
    pub markdown_git_stats: GitStatsState,
    pub markdown_vim: VimState,
    pub markdown_file_watcher: Option<FileWatcher>,
    pub markdown_double_click: DoubleClickState,
    pub markdown_selection: SelectionState,
    /// Cached rendered lines for selection
    pub markdown_rendered_lines: Vec<ratatui::text::Line<'static>>,
    /// Cached inner area for pending click checks
    pub markdown_inner_area: Rect,
    /// Show theme picker popup
    pub show_theme_picker: bool,
    /// Currently selected theme index for the picker (within filtered list)
    pub theme_picker_index: usize,
    /// Filter text for theme search
    pub theme_filter: String,
    /// Index of the currently saved/active theme (in full list)
    pub saved_theme_index: usize,
    /// Current application theme
    pub current_theme: AppTheme,
    /// Original theme to restore if theme picker is cancelled
    pub original_theme: Option<AppTheme>,
    /// Whether the TOC is currently hovered (expands when hovered)
    pub toc_hovered: bool,
    /// Index of the currently hovered TOC entry
    pub toc_hovered_entry: Option<usize>,
    pub toc_scroll_offset: usize,

    // Terminal demo
    pub terminal: Option<TermTui>,
    pub terminal2: Option<TermTui>,
    pub terminal_split: ResizableGrid,

    // Grid demo splits - now a single unified grid
    pub grid_split: ResizableGrid,

    // AI Chat demo
    pub ai_chat_messages: MessageStore,
    pub ai_chat_input: InputState,
    pub ai_chat_loading: bool,

    // Toast notifications
    pub toast_manager: ToastManager,

    // Timing
    pub start_time: Instant,
}
