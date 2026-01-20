//! App constructor.

use ratatui::layout::Rect;
use ratatui_toolkit::markdown_widget::state::{
    CacheState, CollapseState, DisplaySettings, ExpandableState, GitStatsState, ScrollState,
    SourceState, VimState,
};
use ratatui_toolkit::services::file_watcher::FileWatcher;
use ratatui_toolkit::services::theme::loader::load_builtin_theme;
use ratatui_toolkit::services::theme::persistence::load_saved_theme;
use ratatui_toolkit::ThemeVariant;
use ratatui_toolkit::{
    AppTheme, CodeDiff, DiffConfig, DoubleClickState, FileSystemTree, MenuBar, MenuItem,
    ResizableSplit, SelectionState, TermTui, ToastManager, TreeNavigator, TreeViewState,
};
use std::path::PathBuf;
use std::time::Instant;

use super::super::App;
use crate::app::TreePaneFocus;
use crate::constants::SAMPLE_MARKDOWN_FILE;
use crate::demo_mode::DemoMode;
use crate::demo_tab::DemoTab;
use crate::helpers::all_app_themes;

impl App {
    /// Create a new App instance with default state.
    pub fn new() -> Self {
        // Load saved theme or use default
        let themes = all_app_themes();
        let (theme, theme_picker_index) = if let Some(saved_name) = load_saved_theme(None) {
            // Find the index of the saved theme
            let index = themes.iter().position(|&t| t == saved_name).unwrap_or(0);
            // Load the theme
            if let Ok(loaded_theme) = load_builtin_theme(&saved_name, ThemeVariant::Dark) {
                (loaded_theme, index)
            } else {
                (AppTheme::default(), 0)
            }
        } else {
            (AppTheme::default(), 0)
        };

        let menu_bar = MenuBar::new(vec![
            MenuItem::with_icon("Markdown", "", 0),
            MenuItem::with_icon("Code Diff", "", 1),
            MenuItem::with_icon("Trees", "", 2),
            MenuItem::with_icon("Dialogs", "󰍉", 3),
            MenuItem::with_icon("StatusLine", "", 4),
            MenuItem::with_icon("Terminal", "", 5),
            MenuItem::with_icon("Theme", "", 6),
        ])
        .with_selected(0)
        .with_theme(&theme);

        let mut tree_state = TreeViewState::new();
        tree_state.select(vec![0]);

        // Create file system tree for current directory
        let file_tree = FileSystemTree::new(PathBuf::from(".")).ok();
        let mut file_tree_state = TreeViewState::new();
        file_tree_state.select(vec![0]);

        // Create terminal - spawn a shell
        let shell = std::env::var("SHELL").unwrap_or_else(|_| "/bin/sh".to_string());
        let terminal = TermTui::spawn_with_command("Terminal 1", &shell, &[]).ok();
        let terminal2 = TermTui::spawn_with_command("Terminal 2", &shell, &[]).ok();

        // Create CodeDiff from git - the component fetches its own data
        let code_diff = CodeDiff::from_git_with_config(
            DiffConfig::new()
                .show_line_numbers(true)
                .sidebar_enabled(true),
        )
        .with_theme(&theme);

        // Create markdown state modules
        let mut markdown_source = SourceState::default();
        if let Err(e) = markdown_source.set_source_file(SAMPLE_MARKDOWN_FILE) {
            eprintln!("Warning: Could not load markdown file: {}", e);
            markdown_source.set_source_string(
                "# Markdown Rendering\n\nError: Could not load markdown demo file.",
            );
        }

        let mut markdown_display = DisplaySettings::default();
        markdown_display.set_show_line_numbers(true);
        markdown_display.set_show_document_line_numbers(true);

        let mut markdown_git_stats = GitStatsState::default();
        markdown_git_stats.set_show(true);

        Self {
            current_tab: DemoTab::Markdown,
            menu_bar,
            code_diff,
            file_tree,
            file_tree_state,
            file_tree_navigator: TreeNavigator::new(),
            tree_state,
            tree_navigator: TreeNavigator::new(),
            tree_focus: TreePaneFocus::FileTree,
            show_dialog: false,
            dialog_type: ratatui_toolkit::DialogType::Info,
            markdown_scroll: ScrollState::default(),
            markdown_source,
            markdown_cache: CacheState::default(),
            markdown_display,
            markdown_collapse: CollapseState::default(),
            markdown_expandable: ExpandableState::default(),
            markdown_git_stats,
            markdown_vim: VimState::default(),
            markdown_file_watcher: {
                FileWatcher::for_file().ok().and_then(|mut watcher| {
                    watcher
                        .watch(std::path::Path::new(SAMPLE_MARKDOWN_FILE))
                        .ok()?;
                    Some(watcher)
                })
            },
            markdown_double_click: DoubleClickState::new(),
            markdown_selection: SelectionState::new(),
            markdown_rendered_lines: Vec::new(),
            markdown_inner_area: Rect::default(),
            show_theme_picker: false,
            theme_picker_index,
            theme_filter: String::new(),
            saved_theme_index: theme_picker_index,
            current_theme: theme.clone(),
            original_theme: None,
            toc_hovered: false,
            toc_hovered_entry: None,
            toc_scroll_offset: 0,
            status_mode: DemoMode::Normal,
            terminal,
            terminal2,
            terminal_split: ResizableSplit::new(50),
            toast_manager: ToastManager::new(),
            start_time: Instant::now(),
        }
    }
}
