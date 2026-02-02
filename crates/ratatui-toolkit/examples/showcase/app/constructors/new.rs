//! App constructor.

use ratatui::layout::Rect;
use ratatui_toolkit::services::file_watcher::FileWatcher;
use ratatui_toolkit::services::theme::loader::load_builtin_theme;
use ratatui_toolkit::services::theme::persistence::load_saved_theme;
use ratatui_toolkit::ThemeVariant;
use ratatui_toolkit::{
    AIChat, AppTheme, CodeDiff, DiffConfig, InputState, MarkdownState, MarkdownWidget, MenuBar,
    MenuItem, MessageStore, ResizableGrid, ResizableGridWidget, TermTui, ThemePicker, ToastManager,
};
use std::path::PathBuf;
use std::time::Instant;

use super::super::App;
use crate::app::tree_widgets::{FileSystemTreeWidget, TreeViewWidget};
use crate::app::TreePaneFocus;
use crate::constants::SAMPLE_MARKDOWN_FILE;
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
            MenuItem::with_icon("Terminal", "", 3),
            MenuItem::with_icon("Split Grid", "", 4),
            MenuItem::with_icon("AI Chat", "", 5),
            MenuItem::with_icon("Primitives", "", 6),
            MenuItem::with_icon("Theme", "", 7),
        ])
        .with_selected(0)
        .with_theme(&theme);

        // Create file system tree widget that owns its state
        let file_tree = FileSystemTreeWidget::new(PathBuf::from(".")).with_theme(&theme);

        // Create component tree widget that owns its state
        let component_tree_nodes = vec![
            ratatui_toolkit::TreeNode::with_children(
                " Components".to_string(),
                vec![
                    ratatui_toolkit::TreeNode::new(" Button".to_string()),
                    ratatui_toolkit::TreeNode::new(" Dialog".to_string()),
                    ratatui_toolkit::TreeNode::new(" Toast".to_string()),
                    ratatui_toolkit::TreeNode::new(" Pane".to_string()),
                ],
            ),
            ratatui_toolkit::TreeNode::with_children(
                " Layout".to_string(),
                vec![ratatui_toolkit::TreeNode::new(" ResizableGrid".to_string())],
            ),
            ratatui_toolkit::TreeNode::with_children(
                " Widgets".to_string(),
                vec![
                    ratatui_toolkit::TreeNode::new(" TreeView".to_string()),
                    ratatui_toolkit::TreeNode::new(" MenuBar".to_string()),
                    ratatui_toolkit::TreeNode::new(" HotkeyFooter".to_string()),
                ],
            ),
            ratatui_toolkit::TreeNode::with_children(
                " Rendering".to_string(),
                vec![ratatui_toolkit::TreeNode::new(
                    " MarkdownRenderer".to_string(),
                )],
            ),
            ratatui_toolkit::TreeNode::with_children(
                " Terminal".to_string(),
                vec![ratatui_toolkit::TreeNode::new(" TermTui".to_string())],
            ),
        ];
        let component_tree = TreeViewWidget::new(&component_tree_nodes).with_theme(&theme);

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

        // Create resizable grid splits
        // Terminal split: simple vertical split between terminal 1 and 2
        let mut terminal_grid = ResizableGridWidget::new(ResizableGrid::new(0));
        terminal_grid.layout_mut().split_pane_vertically(0);
        terminal_grid.layout_mut().resize_split(0, 50);

        // Grid demo: create a grid with 5 panes (2x2 + 1)
        // Structure: row split (top/bottom) -> left is pane 0, right is another split
        // Right split -> left is pane 1, right is another split
        // Right-right split -> left is pane 2, right is split for panes 3 and 4
        let mut grid_split_widget = ResizableGridWidget::new(ResizableGrid::new(0));
        grid_split_widget.layout_mut().split_pane_horizontally(0); // Split 0: row split (pane 0 | split 1)
        grid_split_widget
            .layout_mut()
            .split_pane_vertically(0)
            .unwrap(); // Split 1: left vertical (pane 1 | split 2)
        let right_of_split1 = grid_split_widget
            .layout_mut()
            .split_pane_vertically(1)
            .unwrap(); // Split 2: right vertical (pane 2 | split 3)
        let _ = grid_split_widget
            .layout_mut()
            .split_pane_vertically(right_of_split1)
            .unwrap(); // Split 3: bottom-right vertical (pane 3 | pane 4)

        // Set initial split ratios
        let _ = grid_split_widget.layout_mut().resize_split(0, 60); // Row split at 60%
        let _ = grid_split_widget.layout_mut().resize_split(1, 33); // Left split at 33%
        let _ = grid_split_widget.layout_mut().resize_split(2, 50); // Middle split at 50%
        let _ = grid_split_widget.layout_mut().resize_split(3, 50); // Bottom-right split at 50%

        // Grid demo splits - we use the same grid but track different parts
        // These are just references to pane IDs for display purposes in the tree
        let _grid_left_pane_id = 0; // Top-left
        let _grid_right_pane_id = 2; // Top-middle

        // Create unified markdown state
        let mut markdown_state = MarkdownState::default();
        if let Err(e) = markdown_state.source.set_source_file(SAMPLE_MARKDOWN_FILE) {
            eprintln!("Warning: Could not load markdown file: {}", e);
            markdown_state.source.set_source_string(
                "# Markdown Rendering\n\nError: Could not load markdown demo file.",
            );
        }
        markdown_state.display.set_show_line_numbers(true);
        markdown_state.display.set_show_document_line_numbers(true);
        markdown_state.git_stats.set_show(true);

        // Create MarkdownWidget from state (widget now owns its state)
        let markdown_widget = MarkdownWidget::from_state(&markdown_state);

        Self {
            current_tab: DemoTab::Markdown,
            menu_bar,
            code_diff,
            file_tree,
            component_tree,
            tree_focus: TreePaneFocus::FileTree,
            show_dialog: false,
            dialog_type: ratatui_toolkit::DialogType::Info,
            markdown_widget,
            show_theme_picker: false,
            theme_picker_index,
            theme_filter: String::new(),
            saved_theme_index: theme_picker_index,
            current_theme: theme.clone(),
            original_theme: None,
            theme_picker: ThemePicker::new(),
            terminal,
            terminal2,
            terminal_split: terminal_grid,
            terminal_content_area: None,
            grid_split_widget,
            grid_content_area: None,
            ai_chat: AIChat::new_ai_chat(),
            toast_manager: ToastManager::new(),
            start_time: Instant::now(),
        }
    }
}
