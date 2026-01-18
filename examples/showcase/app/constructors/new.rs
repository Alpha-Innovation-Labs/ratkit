//! App constructor.

use ratatui::layout::Rect;
use ratatui_toolkit::{
    ClickableScrollbarState, ClickableScrollbarStateScrollExt, DoubleClickState,
    MarkdownScrollManager, MenuBar, MenuItem, ResizableSplit, SelectionState, TermTui,
    ToastManager, TreeNavigator, TreeViewState,
};
use ratatui_toolkit::markdown_renderer::MarkdownFileWatcher;
use std::time::Instant;

use super::super::App;
use crate::constants::SAMPLE_MARKDOWN_FILE;
use crate::demo_mode::DemoMode;
use crate::demo_tab::DemoTab;

impl App {
    /// Create a new App instance with default state.
    pub fn new() -> Self {
        let menu_bar = MenuBar::new(vec![
            MenuItem::with_icon("Markdown", "", 0),
            MenuItem::with_icon("Tree", "", 1),
            MenuItem::with_icon("Dialogs", "󰍉", 2),
            MenuItem::with_icon("Scrollbar", "󰍻", 3),
            MenuItem::with_icon("StatusLine", "", 4),
            MenuItem::with_icon("Terminal", "", 5),
        ])
        .with_selected(0);

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
        let terminal = TermTui::spawn_with_command("Demo Terminal", &shell, &[]).ok();

        Self {
            current_tab: DemoTab::Markdown,
            menu_bar,
            tree_state,
            tree_navigator: TreeNavigator::new(),
            show_dialog: false,
            dialog_type: ratatui_toolkit::DialogType::Info,
            show_hotkey_modal: false,
            markdown_split: ResizableSplit::new(60),
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
            theme_picker_index: 0,
            minimap_hovered: false,
            scrollbar_state,
            scroll_content,
            status_mode: DemoMode::Normal,
            terminal,
            toast_manager: ToastManager::new(),
            start_time: Instant::now(),
        }
    }
}
