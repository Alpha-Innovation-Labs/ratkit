//! Application state for the showcase demo.

mod constructors;
mod methods;

use ratatui::layout::Rect;
use ratatui_toolkit::{
    AppTheme, ClickableScrollbarState, CodeDiff, DoubleClickState, MarkdownScrollManager, MenuBar,
    ResizableSplit, SelectionState, TermTui, ToastManager, TreeNavigator, TreeViewState,
};
use std::time::Instant;

use super::demo_mode::DemoMode;
use super::demo_tab::DemoTab;
use ratatui_toolkit::MarkdownFileWatcher;

/// Main application state for the showcase.
pub struct App {
    // Navigation
    pub current_tab: DemoTab,
    pub menu_bar: MenuBar,

    // Code diff demo - now just a single CodeDiff widget with integrated sidebar
    pub code_diff: CodeDiff,

    // Tree demo
    pub tree_state: TreeViewState,
    pub tree_navigator: TreeNavigator,

    // Dialog demo
    pub show_dialog: bool,
    pub dialog_type: ratatui_toolkit::DialogType,

    // Hotkey modal
    pub show_hotkey_modal: bool,

    // Markdown demo
    pub markdown_split: ResizableSplit,
    pub markdown_scroll: MarkdownScrollManager,
    pub markdown_file_watcher: Option<MarkdownFileWatcher>,
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
    /// Scroll offset for the TOC list
    pub toc_scroll_offset: usize,

    // Scrollbar demo
    pub scrollbar_state: ClickableScrollbarState,
    pub scroll_content: Vec<String>,

    // StatusLine demo
    pub status_mode: DemoMode,

    // Terminal demo
    pub terminal: Option<TermTui>,
    pub terminal2: Option<TermTui>,
    pub terminal_split: ResizableSplit,

    // Toast notifications
    pub toast_manager: ToastManager,

    // Timing
    pub start_time: Instant,
}
