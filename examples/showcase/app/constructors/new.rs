//! App constructor.

use ratatui::layout::Rect;
use ratatui_toolkit::markdown_renderer::MarkdownFileWatcher;
use ratatui_toolkit::theme::loader::load_builtin_theme;
use ratatui_toolkit::theme::persistence::load_saved_theme;
use ratatui_toolkit::ThemeVariant;
use ratatui_toolkit::{
    AppTheme, ClickableScrollbarState, ClickableScrollbarStateScrollExt, CodeDiff, DiffConfig,
    DoubleClickState, MarkdownScrollManager, MenuBar, MenuItem, ResizableSplit, SelectionState,
    TermTui, ToastManager, TreeNavigator, TreeViewState,
};
use std::time::Instant;

use super::super::App;
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
            MenuItem::with_icon("Tree", "", 2),
            MenuItem::with_icon("Dialogs", "󰍉", 3),
            MenuItem::with_icon("Scrollbar", "󰍻", 4),
            MenuItem::with_icon("StatusLine", "", 5),
            MenuItem::with_icon("Terminal", "", 6),
            MenuItem::with_icon("Theme", "", 7),
        ])
        .with_selected(0)
        .with_theme(&theme);

        let mut tree_state = TreeViewState::new();
        tree_state.select(vec![0]);

        // Generate scroll content
        let scroll_content: Vec<String> = (1..=100)
            .map(|i| {
                format!(
                    "Line {}: This is content for the scrollbar demonstration",
                    i
                )
            })
            .collect();

        let scrollbar_state = ClickableScrollbarState::new()
            .set_content(scroll_content.len(), 20)
            .position(0);

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

        Self {
            current_tab: DemoTab::Markdown,
            menu_bar,
            code_diff,
            tree_state,
            tree_navigator: TreeNavigator::new(),
            show_dialog: false,
            dialog_type: ratatui_toolkit::DialogType::Info,
            show_hotkey_modal: false,
            markdown_split: ResizableSplit::new(80),
            markdown_scroll: {
                let mut scroll = MarkdownScrollManager::new();
                scroll.set_show_line_numbers(true);
                scroll.set_show_document_line_numbers(true);
                scroll.set_show_git_stats(true); // Enable git stats in statusline
                                                 // Set file-based source for live reload support
                if let Err(e) = scroll.set_source_file(SAMPLE_MARKDOWN_FILE) {
                    eprintln!("Warning: Could not load markdown file: {}", e);
                    scroll.set_source_string(
                        "# Markdown Rendering\n\nError: Could not load markdown demo file.",
                    );
                }
                scroll
            },
            markdown_file_watcher: {
                // Set up file watcher for live reload
                MarkdownFileWatcher::new().ok().and_then(|mut watcher| {
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
            scrollbar_state,
            scroll_content,
            status_mode: DemoMode::Normal,
            terminal,
            terminal2,
            terminal_split: ResizableSplit::new(50),
            toast_manager: ToastManager::new(),
            start_time: Instant::now(),
        }
    }
}
