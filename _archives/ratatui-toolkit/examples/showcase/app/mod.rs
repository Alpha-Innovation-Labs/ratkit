//! Application state for the showcase demo.

mod constructors;
mod methods;
mod tree_widgets;

use ratatui::layout::Rect;
use ratatui_toolkit::{
    AIChat, AppTheme, CodeDiff, InputState, MarkdownWidget, MenuBar, MessageRole, MessageStore,
    ResizableGridWidget, TermTui, ThemePicker, ToastManager,
};
use std::time::Instant;

use super::demo_tab::DemoTab;
use crate::app::tree_widgets::{FileSystemTreeWidget, TreeViewWidget};

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

    // File system tree demo - now owns its state internally
    pub file_tree: FileSystemTreeWidget,

    // Tree demo - now owns its state internally
    pub component_tree: TreeViewWidget,
    pub tree_focus: TreePaneFocus,

    // Dialog demo
    pub show_dialog: bool,
    pub dialog_type: ratatui_toolkit::DialogType,

    // Markdown demo - widget owns its state internally
    pub markdown_widget: MarkdownWidget<'static>,
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

    /// Theme picker state
    pub theme_picker: ThemePicker,

    // Terminal demo
    pub terminal: Option<TermTui>,
    pub terminal2: Option<TermTui>,
    pub terminal_split: ResizableGridWidget,
    pub terminal_content_area: Option<Rect>,

    // Grid demo splits - now a single unified grid
    pub grid_split_widget: ResizableGridWidget,
    pub grid_content_area: Option<Rect>,

    // AI Chat demo
    pub ai_chat: AIChat,

    // Toast notifications
    pub toast_manager: ToastManager,

    // Timing
    pub start_time: Instant,
}
