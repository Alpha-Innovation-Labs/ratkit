//! Showcase Demo - Demonstrates all ratatui-toolkit components
//!
//! Run with: cargo run --example showcase
//! Or: just dev

use crossterm::{
    event::{
        self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind, MouseButton,
        MouseEventKind,
    },
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Paragraph, Tabs, Widget},
    Terminal,
};
use ratatui_toolkit::{
    copy_selection_to_clipboard, handle_mouse_event, handle_mouse_event_with_double_click,
    handle_mouse_event_with_selection, render_hotkey_modal, render_markdown_statusline,
    render_toasts, render_markdown_interactive_with_selection, ClickableScrollbar,
    ClickableScrollbarState, ClickableScrollbarStateMouseExt, ClickableScrollbarStateScrollExt,
    Dialog, DialogType, DialogWidget, DoubleClickState, GitStats, Hotkey, HotkeyFooter,
    HotkeyItem, HotkeyModalConfig, HotkeySection, MarkdownScrollManager, MarkdownWidgetMode,
    MenuBar, MenuItem, ResizableSplit, ScrollbarEvent, SelectionMouseResult, SelectionState,
    StatusBar, StatusItem, StatusLineStacked, TermTui, Toast, ToastLevel, ToastManager,
    TreeNavigator, TreeNode, TreeView, TreeViewState, SLANT_BL_TR, SLANT_TL_BR,
};
use ratatui_toolkit::markdown_renderer::{CodeBlockTheme, MarkdownFileWatcher};
use std::io;
use std::process::Command;
use std::time::Instant;

/// Get git diff stats (additions, modified_files, deletions) for a specific file.
fn get_git_stats_for_file(file_path: Option<&std::path::Path>) -> (usize, usize, usize) {
    let args = match file_path {
        Some(path) => vec!["diff", "--numstat", "HEAD", "--", path.to_str().unwrap_or("")],
        None => vec!["diff", "--numstat", "HEAD"],
    };

    let output = Command::new("git")
        .args(&args)
        .output()
        .ok();

    if let Some(output) = output {
        if output.status.success() {
            let stdout = String::from_utf8_lossy(&output.stdout);
            let (mut adds, mut dels, mut modified) = (0usize, 0usize, 0usize);
            for line in stdout.lines() {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() >= 2 {
                    let line_adds = parts[0].parse::<usize>().unwrap_or(0);
                    let line_dels = parts[1].parse::<usize>().unwrap_or(0);
                    adds += line_adds;
                    dels += line_dels;
                    // Count as modified if file has changes
                    if line_adds > 0 || line_dels > 0 {
                        modified += 1;
                    }
                }
            }
            return (adds, modified, dels);
        }
    }
    (0, 0, 0)
}

/// Demo vim-like mode for statusline
#[derive(Clone, Copy, PartialEq, Default)]
enum DemoMode {
    #[default]
    Normal,
    Insert,
    Visual,
    Command,
}

#[derive(Clone, Copy, PartialEq)]
enum DemoTab {
    Markdown,
    Tree,
    Dialogs,
    Scrollbar,
    StatusLine,
    Terminal,
}

impl DemoTab {
    fn all() -> Vec<Self> {
        vec![
            Self::Markdown,
            Self::Tree,
            Self::Dialogs,
            Self::Scrollbar,
            Self::StatusLine,
            Self::Terminal,
        ]
    }

    fn name(&self) -> &'static str {
        match self {
            Self::Markdown => "Markdown",
            Self::Tree => "Tree View",
            Self::Dialogs => "Dialogs",
            Self::Scrollbar => "Scrollbar",
            Self::StatusLine => "StatusLine",
            Self::Terminal => "Terminal",
        }
    }
}

struct App {
    // Navigation
    current_tab: DemoTab,
    menu_bar: MenuBar,

    // Tree demo
    tree_state: TreeViewState,
    tree_navigator: TreeNavigator,

    // Dialog demo
    show_dialog: bool,
    dialog_type: DialogType,

    // Hotkey modal
    show_hotkey_modal: bool,

    // Markdown demo
    markdown_split: ResizableSplit,
    markdown_scroll: MarkdownScrollManager,
    markdown_file_watcher: Option<MarkdownFileWatcher>,
    markdown_double_click: DoubleClickState,
    markdown_selection: SelectionState,
    /// Cached rendered lines for selection
    markdown_rendered_lines: Vec<ratatui::text::Line<'static>>,
    /// Pending single-click for deferred processing (mouse_event, inner_area, timestamp)
    markdown_pending_click: Option<(crossterm::event::MouseEvent, Rect, Instant)>,
    /// Show theme picker popup
    show_theme_picker: bool,
    /// Currently selected theme index for the picker
    theme_picker_index: usize,

    // Scrollbar demo
    scrollbar_state: ClickableScrollbarState,
    scroll_content: Vec<String>,

    // StatusLine demo
    status_mode: DemoMode,

    // Terminal demo
    terminal: Option<TermTui>,

    // Toast notifications
    toast_manager: ToastManager,

    // Mouse capture toggle
    mouse_capture_enabled: bool,

    // Timing
    start_time: Instant,

    // Cached git stats for the markdown file (adds, modified, dels)
    cached_git_stats: (usize, usize, usize),
    git_stats_last_update: Instant,
}

impl App {
    fn new() -> Self {
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
            dialog_type: DialogType::Info,
            show_hotkey_modal: false,
            markdown_split: ResizableSplit::new(60),
            markdown_scroll: {
                let mut scroll = MarkdownScrollManager::new();
                scroll.set_show_line_numbers(true);
                scroll.set_show_document_line_numbers(true);
                // Set file-based source for live reload support
                if let Err(e) = scroll.set_source_file(SAMPLE_MARKDOWN_FILE) {
                    eprintln!("Warning: Could not load markdown file: {}", e);
                    scroll.set_source_string("# Markdown Rendering\n\nError: Could not load markdown demo file.");
                }
                scroll
            },
            markdown_file_watcher: {
                // Set up file watcher for live reload
                MarkdownFileWatcher::new().ok().and_then(|mut watcher| {
                    watcher.watch(std::path::Path::new(SAMPLE_MARKDOWN_FILE)).ok()?;
                    Some(watcher)
                })
            },
            markdown_double_click: DoubleClickState::new(),
            markdown_selection: SelectionState::new(),
            markdown_rendered_lines: Vec::new(),
            markdown_pending_click: None,
            show_theme_picker: false,
            theme_picker_index: 0,
            scrollbar_state,
            scroll_content,
            status_mode: DemoMode::Normal,
            terminal,
            toast_manager: ToastManager::new(),
            mouse_capture_enabled: true,
            start_time: Instant::now(),
            cached_git_stats: (0, 0, 0),
            git_stats_last_update: Instant::now(),
        }
    }

    /// Update cached git stats if enough time has passed (every 2 seconds).
    fn update_git_stats(&mut self) {
        if self.git_stats_last_update.elapsed().as_secs() >= 2 {
            self.cached_git_stats = get_git_stats_for_file(self.markdown_scroll.source_path());
            self.git_stats_last_update = Instant::now();
        }
    }

    fn build_tree(&self) -> Vec<TreeNode<String>> {
        vec![
            TreeNode::with_children(
                " Components".to_string(),
                vec![
                    TreeNode::new(" Button".to_string()),
                    TreeNode::new("󰍉 Dialog".to_string()),
                    TreeNode::new(" Toast".to_string()),
                    TreeNode::new("󱒅 Pane".to_string()),
                ],
            ),
            TreeNode::with_children(
                "󰙀 Layout".to_string(),
                vec![
                    TreeNode::new("󰯋 ResizableSplit".to_string()),
                    TreeNode::new("󰕰 MasterLayout".to_string()),
                ],
            ),
            TreeNode::with_children(
                " Widgets".to_string(),
                vec![
                    TreeNode::new(" TreeView".to_string()),
                    TreeNode::new("󰍻 ClickableScrollbar".to_string()),
                    TreeNode::new(" MenuBar".to_string()),
                    TreeNode::new("󰌌 HotkeyFooter".to_string()),
                ],
            ),
            TreeNode::with_children(
                " Rendering".to_string(),
                vec![TreeNode::new(" MarkdownRenderer".to_string())],
            ),
            TreeNode::with_children(
                " Terminal".to_string(),
                vec![TreeNode::new(" TermTui".to_string())],
            ),
        ]
    }

    fn select_tab(&mut self, tab: DemoTab) {
        self.current_tab = tab;
        let idx = DemoTab::all().iter().position(|t| *t == tab).unwrap_or(0);
        for (i, item) in self.menu_bar.items.iter_mut().enumerate() {
            item.selected = i == idx;
        }
    }
}

const SAMPLE_MARKDOWN_FILE: &str = "examples/markdown_demo_full.md";

fn main() -> io::Result<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new();
    app.toast_manager
        .info("Welcome to ratatui-toolkit showcase!");
    app.toast_manager
        .add(Toast::new("Press Tab to switch demos", ToastLevel::Info));

    loop {
        let tree_nodes = app.build_tree();

        terminal.draw(|frame| {
            let area = frame.area();

            // Main layout
            let main_chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Length(3), // Menu bar
                    Constraint::Length(1), // Tab bar
                    Constraint::Min(0),    // Content
                    Constraint::Length(1), // Status bar
                    Constraint::Length(1), // Hotkey footer
                ])
                .split(area);

            // Menu bar
            app.menu_bar.render(frame, main_chunks[0]);

            // Tab bar
            let tabs: Vec<Line> = DemoTab::all()
                .iter()
                .map(|t| {
                    let style = if *t == app.current_tab {
                        Style::default()
                            .fg(Color::Cyan)
                            .add_modifier(Modifier::BOLD)
                    } else {
                        Style::default().fg(Color::DarkGray)
                    };
                    Line::styled(t.name(), style)
                })
                .collect();
            let tab_widget = Tabs::new(tabs)
                .select(
                    DemoTab::all()
                        .iter()
                        .position(|t| *t == app.current_tab)
                        .unwrap_or(0),
                )
                .divider(" │ ");
            frame.render_widget(tab_widget, main_chunks[1]);

            // Content area based on selected tab
            let content_area = main_chunks[2];

            match app.current_tab {
                DemoTab::Markdown => render_markdown_demo(frame, content_area, &mut app),
                DemoTab::Tree => render_tree_demo(frame, content_area, &mut app, &tree_nodes),
                DemoTab::Dialogs => render_dialogs_demo(frame, content_area, &mut app),
                DemoTab::Scrollbar => render_scrollbar_demo(frame, content_area, &mut app),
                DemoTab::StatusLine => render_statusline_demo(frame, content_area, &mut app),
                DemoTab::Terminal => render_terminal_demo(frame, content_area, &mut app),
            }

            // Status bar
            let elapsed = app.start_time.elapsed().as_secs();
            let status = if app.current_tab == DemoTab::Markdown {
                // Use source_line_count for accurate display (falls back to total_lines if 0)
                let display_total = if app.markdown_scroll.source_line_count > 0 {
                    app.markdown_scroll.source_line_count
                } else {
                    app.markdown_scroll.total_lines
                };
                let line_info = format!(
                    "Ln {}/{}",
                    app.markdown_scroll.current_line,
                    display_total
                );
                let theme_name = get_theme_name(app.markdown_scroll.code_block_theme);
                StatusBar::new()
                    .add_left(StatusItem::bold(format!(" {}", app.current_tab.name())))
                    .add_center(StatusItem::new(line_info))
                    .add_right(StatusItem::new(format!(" {} [T]", theme_name)))
            } else {
                StatusBar::new()
                    .add_left(StatusItem::bold(format!(" {}", app.current_tab.name())))
                    .add_center(StatusItem::new("ratatui-toolkit v0.1.0"))
                    .add_right(StatusItem::dimmed(format!("{}s", elapsed)))
            };
            frame.render_widget(status, main_chunks[3]);

            // Hotkey footer
            let mut footer_items = vec![
                HotkeyItem::new("Tab", "switch"),
                HotkeyItem::new("1-6", "tabs"),
            ];
            if app.current_tab == DemoTab::Markdown {
                footer_items.push(HotkeyItem::new("T", "theme"));
            }
            footer_items.extend([
                HotkeyItem::new("t", "toast"),
                HotkeyItem::new("?", "help"),
                HotkeyItem::new("q", "quit"),
            ]);
            let footer = HotkeyFooter::new(footer_items);
            frame.render_widget(&footer, main_chunks[4]);

            // Toasts
            render_toasts(frame, &app.toast_manager);

            // Dialog overlay
            if app.show_dialog {
                let (title, message) = match app.dialog_type {
                    DialogType::Info => (
                        "Information",
                        "This is an info dialog.\n\nPress Enter or Esc to close.",
                    ),
                    DialogType::Success => ("Success!", "Operation completed successfully!"),
                    DialogType::Warning => ("Warning", "This action may have consequences."),
                    DialogType::Error => ("Error", "Something went wrong!"),
                    DialogType::Confirm => ("Confirm", "Do you want to proceed?"),
                };
                let mut dialog = Dialog::new(title, message)
                    .dialog_type(app.dialog_type)
                    .width_percent(0.5)
                    .height_percent(0.35);
                let dialog_widget = DialogWidget::new(&mut dialog);
                frame.render_widget(dialog_widget, area);
            }

            // Hotkey modal overlay
            if app.show_hotkey_modal {
                let sections = vec![
                    HotkeySection {
                        title: "Navigation".to_string(),
                        hotkeys: vec![
                            Hotkey::new("Tab", "Next tab"),
                            Hotkey::new("Shift+Tab", "Previous tab"),
                            Hotkey::new("1-7", "Jump to tab"),
                        ],
                    },
                    HotkeySection {
                        title: "Tree View".to_string(),
                        hotkeys: vec![
                            Hotkey::new("j/↓", "Move down"),
                            Hotkey::new("k/↑", "Move up"),
                            Hotkey::new("l/→", "Expand"),
                            Hotkey::new("h/←", "Collapse"),
                        ],
                    },
                    HotkeySection {
                        title: "General".to_string(),
                        hotkeys: vec![
                            Hotkey::new("t", "Show toast"),
                            Hotkey::new("?", "Toggle help"),
                            Hotkey::new("q", "Quit"),
                        ],
                    },
                ];
                let config = HotkeyModalConfig {
                    title: "Keyboard Shortcuts".to_string(),
                    ..Default::default()
                };
                render_hotkey_modal(frame, &sections, &config);
            }

            // Theme picker popup
            if app.show_theme_picker {
                render_theme_picker(frame, &app);
            }
        })?;

        app.toast_manager.remove_expired();

        // Update cached git stats periodically
        app.update_git_stats();

        // Check for markdown file changes and reload if needed
        if let Some(ref watcher) = app.markdown_file_watcher {
            if watcher.check_for_changes() {
                if let Ok(true) = app.markdown_scroll.reload_source() {
                    app.toast_manager.add(Toast::new("Markdown file reloaded", ToastLevel::Info));
                }
            }
        }

        // Check for pending markdown single-click timeout (for deferred processing)
        const DOUBLE_CLICK_THRESHOLD_MS: u64 = 150;
        if let Some((mouse_event, inner_area, click_time)) = app.markdown_pending_click.take() {
            if click_time.elapsed().as_millis() as u64 >= DOUBLE_CLICK_THRESHOLD_MS {
                // Timeout expired - process as single-click (heading collapse)
                let content = app.markdown_scroll.content().unwrap_or("").to_string();
                handle_mouse_event(
                    &mouse_event,
                    inner_area,
                    &content,
                    &mut app.markdown_scroll,
                );
            } else {
                // Not timed out yet - put it back
                app.markdown_pending_click = Some((mouse_event, inner_area, click_time));
            }
        }

        // Adaptive polling: fast during drag for smooth resize, slower otherwise to save CPU
        let poll_timeout = if app.markdown_split.is_dragging {
            std::time::Duration::from_millis(8) // ~120fps for smooth dragging
        } else {
            std::time::Duration::from_millis(50) // Normal rate
        };

        if event::poll(poll_timeout)? {
            match event::read()? {
                Event::Key(key) if key.kind == KeyEventKind::Press => {
                    // Handle modal dismissals first
                    if app.show_hotkey_modal {
                        app.show_hotkey_modal = false;
                        continue;
                    }
                    if app.show_dialog {
                        app.show_dialog = false;
                        continue;
                    }
                    // Handle theme picker
                    if app.show_theme_picker {
                        let themes = all_themes();
                        match key.code {
                            KeyCode::Esc | KeyCode::Char('T') => {
                                app.show_theme_picker = false;
                            }
                            KeyCode::Char('j') | KeyCode::Down => {
                                app.theme_picker_index = (app.theme_picker_index + 1) % themes.len();
                            }
                            KeyCode::Char('k') | KeyCode::Up => {
                                app.theme_picker_index = if app.theme_picker_index == 0 {
                                    themes.len() - 1
                                } else {
                                    app.theme_picker_index - 1
                                };
                            }
                            KeyCode::Enter => {
                                let selected_theme = themes[app.theme_picker_index];
                                app.markdown_scroll.set_code_block_theme(selected_theme);
                                app.show_theme_picker = false;
                                app.toast_manager.add(Toast::new(
                                    &format!("Theme: {}", get_theme_name(selected_theme)),
                                    ToastLevel::Success,
                                ));
                            }
                            _ => {}
                        }
                        continue;
                    }

                    match key.code {
                        KeyCode::Char('q') | KeyCode::Char('Q') => break,
                        KeyCode::Tab => {
                            let tabs = DemoTab::all();
                            let idx = tabs.iter().position(|t| *t == app.current_tab).unwrap_or(0);
                            let next = (idx + 1) % tabs.len();
                            app.select_tab(tabs[next]);
                        }
                        KeyCode::BackTab => {
                            let tabs = DemoTab::all();
                            let idx = tabs.iter().position(|t| *t == app.current_tab).unwrap_or(0);
                            let prev = if idx == 0 { tabs.len() - 1 } else { idx - 1 };
                            app.select_tab(tabs[prev]);
                        }
                        KeyCode::Char('1') => app.select_tab(DemoTab::Markdown),
                        KeyCode::Char('2') => app.select_tab(DemoTab::Tree),
                        KeyCode::Char('3') => app.select_tab(DemoTab::Dialogs),
                        KeyCode::Char('4') => app.select_tab(DemoTab::Scrollbar),
                        KeyCode::Char('5') => app.select_tab(DemoTab::StatusLine),
                        KeyCode::Char('6') => app.select_tab(DemoTab::Terminal),
                        KeyCode::Char('t') => {
                            let messages = [
                                ("Info toast", ToastLevel::Info),
                                ("Success!", ToastLevel::Success),
                                ("Warning message", ToastLevel::Warning),
                                ("Error occurred", ToastLevel::Error),
                            ];
                            let idx =
                                (app.start_time.elapsed().as_millis() as usize) % messages.len();
                            app.toast_manager
                                .add(Toast::new(messages[idx].0, messages[idx].1));
                        }
                        KeyCode::Char('?') => {
                            app.show_hotkey_modal = !app.show_hotkey_modal;
                        }
                        KeyCode::Char('T') => {
                            if app.current_tab == DemoTab::Markdown {
                                app.show_theme_picker = true;
                                // Set picker index to current theme
                                let themes = all_themes();
                                app.theme_picker_index = themes
                                    .iter()
                                    .position(|t| *t == app.markdown_scroll.code_block_theme)
                                    .unwrap_or(0);
                            }
                        }
                        // Tab-specific keys
                        _ => match app.current_tab {
                            DemoTab::Tree => {
                                let tree_nodes = app.build_tree();
                                app.tree_navigator.handle_key(
                                    key,
                                    &tree_nodes,
                                    &mut app.tree_state,
                                );
                            }
                            DemoTab::Dialogs => match key.code {
                                KeyCode::Char('i') => {
                                    app.dialog_type = DialogType::Info;
                                    app.show_dialog = true;
                                }
                                KeyCode::Char('s') => {
                                    app.dialog_type = DialogType::Success;
                                    app.show_dialog = true;
                                }
                                KeyCode::Char('w') => {
                                    app.dialog_type = DialogType::Warning;
                                    app.show_dialog = true;
                                }
                                KeyCode::Char('e') => {
                                    app.dialog_type = DialogType::Error;
                                    app.show_dialog = true;
                                }
                                KeyCode::Char('c') => {
                                    app.dialog_type = DialogType::Confirm;
                                    app.show_dialog = true;
                                }
                                _ => {}
                            },
                            DemoTab::Markdown => {
                                // Handle Escape to exit selection mode
                                if key.code == KeyCode::Esc && app.markdown_selection.is_active() {
                                    app.markdown_selection.exit();
                                }
                                // Handle Ctrl+Shift+C to copy selection
                                else if key.code == KeyCode::Char('C')
                                    && key.modifiers.contains(event::KeyModifiers::CONTROL)
                                    && key.modifiers.contains(event::KeyModifiers::SHIFT)
                                {
                                    if copy_selection_to_clipboard(&app.markdown_selection) {
                                        app.toast_manager.add(Toast::new(
                                            "Copied to clipboard!",
                                            ToastLevel::Success,
                                        ));
                                        app.markdown_selection.exit();
                                    }
                                }
                                // Handle 'y' to copy selection (vim-style)
                                else if key.code == KeyCode::Char('y')
                                    && app.markdown_selection.has_selection()
                                {
                                    if copy_selection_to_clipboard(&app.markdown_selection) {
                                        app.toast_manager.add(Toast::new(
                                            "Copied to clipboard!",
                                            ToastLevel::Success,
                                        ));
                                        app.markdown_selection.exit();
                                    }
                                } else {
                                    match key.code {
                                        KeyCode::Char('j') | KeyCode::Down => {
                                            app.markdown_scroll.scroll_down(1);
                                        }
                                        KeyCode::Char('k') | KeyCode::Up => {
                                            app.markdown_scroll.scroll_up(1);
                                        }
                                        KeyCode::PageDown => {
                                            app.markdown_scroll.scroll_down(10);
                                        }
                                        KeyCode::PageUp => {
                                            app.markdown_scroll.scroll_up(10);
                                        }
                                        KeyCode::Home => {
                                            app.markdown_scroll.scroll_to_top();
                                        }
                                        KeyCode::End => {
                                            app.markdown_scroll.scroll_to_bottom();
                                        }
                                        _ => {}
                                    }
                                }
                            }
                            DemoTab::Scrollbar => match key.code {
                                KeyCode::Char('j') | KeyCode::Down => {
                                    app.scrollbar_state.scroll_down(1);
                                }
                                KeyCode::Char('k') | KeyCode::Up => {
                                    app.scrollbar_state.scroll_up(1);
                                }
                                _ => {}
                            },
                            DemoTab::StatusLine => match key.code {
                                KeyCode::Char('n') => {
                                    app.status_mode = DemoMode::Normal;
                                }
                                KeyCode::Char('i') => {
                                    app.status_mode = DemoMode::Insert;
                                }
                                KeyCode::Char('v') => {
                                    app.status_mode = DemoMode::Visual;
                                }
                                KeyCode::Char('c') => {
                                    app.status_mode = DemoMode::Command;
                                }
                                _ => {}
                            },
                            DemoTab::Terminal => {
                                // Forward key to terminal
                                if let Some(ref mut term) = app.terminal {
                                    term.handle_key(key);
                                }
                            }
                        },
                    }
                }
                Event::Mouse(mouse) => {
                    let area = terminal.get_frame().area();

                    // Handle menu bar clicks
                    if mouse.kind == MouseEventKind::Down(MouseButton::Left) {
                        if let Some(idx) = app.menu_bar.handle_click(mouse.column, mouse.row) {
                            app.select_tab(DemoTab::all()[idx]);
                        }
                    }

                    // Handle scrollbar in Scrollbar tab
                    if app.current_tab == DemoTab::Scrollbar {
                        match app.scrollbar_state.handle_mouse_event(&mouse) {
                            ScrollbarEvent::Up(n) => {
                                app.scrollbar_state.scroll_up(n);
                            }
                            ScrollbarEvent::Down(n) => {
                                app.scrollbar_state.scroll_down(n);
                            }
                            ScrollbarEvent::Position(pos) => {
                                app.scrollbar_state.set_offset(pos);
                            }
                            ScrollbarEvent::None => {}
                        }
                    }

                    // Handle markdown widget interactions
                    if app.current_tab == DemoTab::Markdown {
                        // Calculate the same area as render_markdown_demo receives
                        let main_chunks = Layout::default()
                            .direction(Direction::Vertical)
                            .constraints([
                                Constraint::Length(3), // Menu bar
                                Constraint::Length(1), // Tab bar
                                Constraint::Min(0),    // Content
                                Constraint::Length(1), // Status bar
                                Constraint::Length(1), // Hotkey footer
                            ])
                            .split(area);
                        let content_area = main_chunks[2];

                        // Handle split divider first
                        match mouse.kind {
                            MouseEventKind::Down(MouseButton::Left) => {
                                if app.markdown_split.is_on_divider(mouse.column, mouse.row, content_area) {
                                    app.markdown_split.start_drag();
                                }
                            }
                            MouseEventKind::Drag(MouseButton::Left) => {
                                if app.markdown_split.is_dragging {
                                    app.markdown_split.update_from_mouse(mouse.column, mouse.row, content_area);
                                }
                            }
                            MouseEventKind::Up(MouseButton::Left) => {
                                if app.markdown_split.is_dragging {
                                    app.markdown_split.stop_drag();
                                    // Invalidate cache after resize
                                    app.markdown_scroll.invalidate_cache();
                                }
                            }
                            MouseEventKind::Moved => {
                                app.markdown_split.is_hovering = app.markdown_split.is_on_divider(mouse.column, mouse.row, content_area);
                            }
                            _ => {}
                        }

                        // Only handle markdown interactions when NOT dragging divider
                        if !app.markdown_split.is_dragging {
                            // Calculate markdown area based on split percentage
                            let left_width = (content_area.width as u32 * app.markdown_split.split_percent as u32 / 100) as u16;
                            let markdown_area = Rect {
                                x: content_area.x,
                                y: content_area.y,
                                width: left_width,
                                height: content_area.height,
                            };

                            // Account for the block border (1 pixel on each side)
                            let inner_area = Rect {
                                x: markdown_area.x + 1,
                                y: markdown_area.y + 1,
                                width: markdown_area.width.saturating_sub(2),
                                height: markdown_area.height.saturating_sub(2),
                            };

                            // Get content before mutable borrows
                            let content = app.markdown_scroll.content().unwrap_or("").to_string();

                            // Handle selection with mouse drag
                            let result = handle_mouse_event_with_selection(
                                &mouse,
                                inner_area,
                                &content,
                                &mut app.markdown_scroll,
                                &mut app.markdown_selection,
                                &app.markdown_rendered_lines,
                            );

                            // Show toast if text was auto-copied
                            if result.copied {
                                app.toast_manager.add(Toast::new(
                                    "Copied to clipboard!",
                                    ToastLevel::Success,
                                ));
                            }

                            // If not a drag/selection event, check for double-click
                            if !result.handled || mouse.kind == MouseEventKind::Down(MouseButton::Left) {
                                let (_handled, double_click_event) = handle_mouse_event_with_double_click(
                                    &mouse,
                                    inner_area,
                                    &content,
                                    &mut app.markdown_scroll,
                                    &mut app.markdown_double_click,
                                );

                                // Show toast on double-click
                                if let Some(evt) = double_click_event {
                                    // Clear pending click - it was part of a double-click
                                    app.markdown_pending_click = None;
                                    let msg = format!(
                                        "Line {}: {} - \"{}\"",
                                        evt.line_number,
                                        evt.line_kind,
                                        if evt.content.len() > 40 {
                                            format!("{}...", &evt.content[..40])
                                        } else {
                                            evt.content
                                        }
                                    );
                                    app.toast_manager.add(Toast::new(&msg, ToastLevel::Info));
                                } else if mouse.kind == MouseEventKind::Down(MouseButton::Left) {
                                    // Store pending single-click for deferred processing
                                    app.markdown_pending_click = Some((mouse, inner_area, Instant::now()));
                                }
                            }
                        }
                    }

                    // Handle terminal mouse events (scroll, selection, drag)
                    if app.current_tab == DemoTab::Terminal {
                        if let Some(ref mut term) = app.terminal {
                            // Calculate terminal area (60% of content area)
                            let content_area = Rect {
                                x: area.x,
                                y: area.y + 4,
                                width: (area.width * 60) / 100,
                                height: area.height.saturating_sub(6),
                            };
                            term.handle_mouse(mouse, content_area);
                        }
                    }

                    app.menu_bar.update_hover(mouse.column, mouse.row);
                }
                _ => {}
            }
        }
    }

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    Ok(())
}

fn render_tree_demo(
    frame: &mut ratatui::Frame,
    area: Rect,
    app: &mut App,
    tree_nodes: &[TreeNode<String>],
) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(area);

    let tree = TreeView::new(tree_nodes.to_vec())
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .title(" TreeView - Component Browser "),
        )
        .highlight_style(Style::default().bg(Color::DarkGray))
        .render_fn(|data: &String, state| {
            let style = if state.is_selected {
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default()
            };
            Line::styled(data.clone(), style)
        });

    frame.render_stateful_widget(tree, chunks[0], &mut app.tree_state);

    let info = Paragraph::new(vec![
        Line::from(""),
        Line::from(Span::styled(
            "  TreeView Features",
            Style::default().add_modifier(Modifier::BOLD),
        )),
        Line::from(""),
        Line::from("  Navigation:"),
        Line::from("    j/↓  Move down"),
        Line::from("    k/↑  Move up"),
        Line::from("    l/→  Expand node"),
        Line::from("    h/←  Collapse node"),
        Line::from("    Enter Toggle expand"),
        Line::from(""),
        Line::from("  Features:"),
        Line::from("    • Generic data type"),
        Line::from("    • Custom render function"),
        Line::from("    • Configurable keybindings"),
        Line::from("    • Full-row selection"),
    ])
    .block(
        Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .title(" Controls "),
    );

    frame.render_widget(info, chunks[1]);
}

fn render_dialogs_demo(frame: &mut ratatui::Frame, area: Rect, _app: &mut App) {
    let content = Paragraph::new(vec![
        Line::from(""),
        Line::from(Span::styled(
            "  Dialog Types",
            Style::default().add_modifier(Modifier::BOLD),
        )),
        Line::from(""),
        Line::from("  Press a key to show dialog:"),
        Line::from(""),
        Line::styled("    [i] Info Dialog", Style::default().fg(Color::Cyan)),
        Line::styled("    [s] Success Dialog", Style::default().fg(Color::Green)),
        Line::styled("    [w] Warning Dialog", Style::default().fg(Color::Yellow)),
        Line::styled("    [e] Error Dialog", Style::default().fg(Color::Red)),
        Line::styled("    [c] Confirm Dialog", Style::default().fg(Color::Blue)),
        Line::from(""),
        Line::from("  Dialog features:"),
        Line::from("    • Modal overlay with dimmed background"),
        Line::from("    • Customizable width/height"),
        Line::from("    • Multiple button support"),
        Line::from("    • Click detection on buttons"),
        Line::from("    • Tab navigation between buttons"),
    ])
    .block(
        Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .title(" Dialogs Demo "),
    );

    frame.render_widget(content, area);
}

fn render_markdown_demo(frame: &mut ratatui::Frame, area: Rect, app: &mut App) {
    // Update divider position for resize handling
    app.markdown_split.update_divider_position(area);

    // Calculate panel areas based on split percentage
    let left_width = (area.width as u32 * app.markdown_split.split_percent as u32 / 100) as u16;
    let left_area = Rect {
        x: area.x,
        y: area.y,
        width: left_width,
        height: area.height,
    };
    let right_area = Rect {
        x: area.x + left_width,
        y: area.y,
        width: area.width - left_width,
        height: area.height,
    };

    // Left panel - Markdown renderer with selection highlighting
    let selection_active = app.markdown_selection.is_active();
    let title = if selection_active {
        " Markdown Renderer (Selection Mode - y to copy, Esc to exit) "
    } else {
        " Markdown Renderer "
    };
    let border_style = if selection_active {
        Style::default().fg(Color::Cyan)
    } else if app.markdown_split.is_hovering || app.markdown_split.is_dragging {
        Style::default().fg(Color::Cyan)
    } else {
        Style::default()
    };
    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .title(title)
        .border_style(border_style);

    let inner_area = block.inner(left_area);
    block.render(left_area, frame.buffer_mut());

    // Reserve space for statusline
    let (content_area, statusline_area) = if inner_area.height > 1 {
        (
            Rect {
                height: inner_area.height.saturating_sub(1),
                ..inner_area
            },
            Rect {
                y: inner_area.y + inner_area.height.saturating_sub(1),
                height: 1,
                ..inner_area
            },
        )
    } else {
        (inner_area, inner_area)
    };

    // Render with selection highlighting
    let content = app.markdown_scroll.content().unwrap_or("").to_string();
    let (text, all_lines) = render_markdown_interactive_with_selection(
        &content,
        &mut app.markdown_scroll,
        content_area,
        app.markdown_split.is_dragging,
        &app.markdown_selection,
    );

    // Store rendered lines for selection text extraction
    app.markdown_rendered_lines = all_lines;

    // Render the text
    let paragraph = Paragraph::new(text);
    frame.render_widget(paragraph, content_area);

    // Render statusline using the widget's built-in function
    let mode = if selection_active {
        MarkdownWidgetMode::Drag
    } else {
        MarkdownWidgetMode::Normal
    };
    let (adds, modified, dels) = app.cached_git_stats;
    let git_stats = Some(GitStats {
        additions: adds,
        modified,
        deletions: dels,
    });
    // Get filename from source path
    let filename = app.markdown_scroll.source_path()
        .and_then(|p| p.file_name())
        .and_then(|n| n.to_str());
    // Use source_line_count for accurate display (falls back to total_lines if 0)
    let display_total = if app.markdown_scroll.source_line_count > 0 {
        app.markdown_scroll.source_line_count
    } else {
        app.markdown_scroll.total_lines
    };
    render_markdown_statusline(
        statusline_area,
        frame.buffer_mut(),
        mode,
        filename,
        git_stats,
        app.markdown_scroll.current_line,
        display_total,
    );

    // Right panel - Info/controls
    let info = Paragraph::new(vec![
        Line::from(""),
        Line::from(Span::styled(
            "  Markdown Features",
            Style::default().add_modifier(Modifier::BOLD),
        )),
        Line::from(""),
        Line::from("  Navigation:"),
        Line::from("    j/↓  Scroll down"),
        Line::from("    k/↑  Scroll up"),
        Line::from("    PgDn/PgUp Page scroll"),
        Line::from("    Home/End  Top/Bottom"),
        Line::from(""),
        Line::from("  Selection:"),
        Line::from("    Drag   Select text"),
        Line::from("    y      Copy selection"),
        Line::from("    Ctrl+Shift+C  Copy"),
        Line::from("    Esc    Exit selection"),
        Line::from(""),
        Line::from("  Interactions:"),
        Line::from("    • Click headers to collapse"),
        Line::from("    • Mouse wheel to scroll"),
        Line::from("    • Drag divider to resize"),
        Line::from(""),
        Line::from("  Rendering:"),
        Line::from("    • Syntax highlighting"),
        Line::from("    • Line numbers in code"),
        Line::from("    • Tables, links, blockquotes"),
    ])
    .block(
        Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .title(" Controls ")
            .border_style(border_style),
    );

    frame.render_widget(info, right_area);
}

fn render_scrollbar_demo(frame: &mut ratatui::Frame, area: Rect, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Min(0), Constraint::Length(1)])
        .split(area);

    // Update page length based on visible area
    let visible_height = chunks[0].height.saturating_sub(2) as usize;
    app.scrollbar_state = app
        .scrollbar_state
        .clone()
        .set_content(app.scroll_content.len(), visible_height);

    // Content
    let visible_lines: Vec<Line> = app
        .scroll_content
        .iter()
        .skip(app.scrollbar_state.offset())
        .take(visible_height)
        .map(|s| Line::from(s.as_str()))
        .collect();

    let content = Paragraph::new(visible_lines).block(
        Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .title(format!(
                " ClickableScrollbar - Line {}/{} ",
                app.scrollbar_state.offset() + 1,
                app.scroll_content.len()
            )),
    );

    frame.render_widget(content, chunks[0]);

    // Scrollbar
    let scrollbar = ClickableScrollbar::vertical();
    frame.render_stateful_widget(scrollbar, chunks[1], &mut app.scrollbar_state);
}

fn render_statusline_demo(frame: &mut ratatui::Frame, area: Rect, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(0),    // Instructions
            Constraint::Length(1), // StatusLineStacked demo
            Constraint::Length(1), // Another style
            Constraint::Length(1), // Yet another style
        ])
        .split(area);

    // Instructions
    let mode_name = match app.status_mode {
        DemoMode::Normal => "NORMAL",
        DemoMode::Insert => "INSERT",
        DemoMode::Visual => "VISUAL",
        DemoMode::Command => "COMMAND",
    };

    let content = Paragraph::new(vec![
        Line::from(""),
        Line::from(Span::styled(
            "  StatusLineStacked - Neovim-style Powerline",
            Style::default().add_modifier(Modifier::BOLD),
        )),
        Line::from(""),
        Line::from(format!("  Current mode: {}", mode_name)),
        Line::from(""),
        Line::from("  Press a key to change mode:"),
        Line::from(""),
        Line::styled("    [n] Normal mode", Style::default().fg(Color::Blue)),
        Line::styled("    [i] Insert mode", Style::default().fg(Color::Green)),
        Line::styled("    [v] Visual mode", Style::default().fg(Color::Magenta)),
        Line::styled("    [c] Command mode", Style::default().fg(Color::Yellow)),
        Line::from(""),
        Line::from("  Features:"),
        Line::from("    • Powerline-style diagonal separators"),
        Line::from("    • Stacked indicators left & right"),
        Line::from("    • Requires Nerd Font for glyphs"),
    ])
    .block(
        Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .title(" StatusLineStacked Demo "),
    );
    frame.render_widget(content, chunks[0]);

    // StatusLineStacked - Style 1 (mode indicator)
    let (mode_color, mode_text) = match app.status_mode {
        DemoMode::Normal => (Color::Blue, " NORMAL "),
        DemoMode::Insert => (Color::Green, " INSERT "),
        DemoMode::Visual => (Color::Magenta, " VISUAL "),
        DemoMode::Command => (Color::Yellow, " COMMAND "),
    };

    let status1 = StatusLineStacked::new()
        .start(
            Span::from(mode_text).style(Style::new().fg(Color::Black).bg(mode_color)),
            Span::from(SLANT_TL_BR).style(Style::new().fg(mode_color).bg(Color::DarkGray)),
        )
        .start(
            Span::from(" main ").style(Style::new().fg(Color::White).bg(Color::DarkGray)),
            Span::from(SLANT_TL_BR).style(Style::new().fg(Color::DarkGray)),
        )
        .center("showcase.rs")
        .end(
            Span::from(" UTF-8 ").style(Style::new().fg(Color::Black).bg(Color::Cyan)),
            Span::from(SLANT_BL_TR).style(Style::new().fg(Color::Cyan)),
        );
    frame.render_widget(status1, chunks[1]);

    // StatusLineStacked - Style 2
    let status2 = StatusLineStacked::new()
        .start(
            Span::from("  rust ").style(Style::new().fg(Color::Black).bg(Color::Red)),
            Span::from(SLANT_TL_BR).style(Style::new().fg(Color::Red).bg(Color::Gray)),
        )
        .start(
            Span::from(" src/lib.rs ").style(Style::new().fg(Color::Black).bg(Color::Gray)),
            Span::from(SLANT_TL_BR).style(Style::new().fg(Color::Gray)),
        )
        .center("ratatui-toolkit v0.1.0")
        .end(
            Span::from(" Ln 42 ").style(Style::new().fg(Color::Black).bg(Color::Green)),
            Span::from(SLANT_BL_TR).style(Style::new().fg(Color::Green).bg(Color::Gray)),
        )
        .end(
            Span::from(" Col 8 ").style(Style::new().fg(Color::Black).bg(Color::Gray)),
            Span::from(SLANT_BL_TR).style(Style::new().fg(Color::Gray)),
        );
    frame.render_widget(status2, chunks[2]);

    // StatusLineStacked - Style 3 (minimal)
    let status3 = StatusLineStacked::new()
        .start(
            Span::from(" 󰈙 ").style(Style::new().fg(Color::Cyan)),
            Span::from("").style(Style::new()),
        )
        .center("Press ? for help")
        .end(
            Span::from(" 100% ").style(Style::new().fg(Color::Green)),
            Span::from("").style(Style::new()),
        );
    frame.render_widget(status3, chunks[3]);
}

fn render_terminal_demo(frame: &mut ratatui::Frame, area: Rect, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(60), Constraint::Percentage(40)])
        .split(area);

    // Terminal - render directly in the area
    if let Some(ref mut term) = app.terminal {
        term.render(frame, chunks[0]);
    } else {
        // Fallback if terminal failed to spawn
        let fallback = Paragraph::new("Terminal failed to spawn").block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .title(" Terminal "),
        );
        frame.render_widget(fallback, chunks[0]);
    }

    // Copy mode indicator
    let copy_mode_info = if let Some(ref term) = app.terminal {
        if term.copy_mode.is_active() {
            "COPY MODE (hjkl/arrows to move, v to select, y to copy, Esc to exit)"
        } else {
            "Press Ctrl+X to enter copy mode"
        }
    } else {
        ""
    };

    // Info panel
    let info = Paragraph::new(vec![
        Line::from(""),
        Line::from(Span::styled(
            "  TermTui Features",
            Style::default().add_modifier(Modifier::BOLD),
        )),
        Line::from(""),
        Line::from("  A terminal emulator using"),
        Line::from("  termwiz + mprocs architecture."),
        Line::from(""),
        Line::from("  Features:"),
        Line::from("    • VT100 escape sequences"),
        Line::from("    • Full color support (256/RGB)"),
        Line::from("    • VecDeque scrollback buffer"),
        Line::from("    • Copy mode (Ctrl+X)"),
        Line::from("    • Vim-style navigation (hjkl)"),
        Line::from("    • Visual selection (v + y)"),
        Line::from(""),
        Line::from(Span::styled(
            format!("  {}", copy_mode_info),
            Style::default().fg(Color::Yellow),
        )),
    ])
    .block(
        Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .title(" Info "),
    );
    frame.render_widget(info, chunks[1]);
}

/// Get the display name for a code block theme
fn get_theme_name(theme: CodeBlockTheme) -> &'static str {
    match theme {
        CodeBlockTheme::AyuDark => "Ayu Dark",
        CodeBlockTheme::GitHubDark => "GitHub Dark",
        CodeBlockTheme::Dracula => "Dracula",
        CodeBlockTheme::Nord => "Nord",
        CodeBlockTheme::Monokai => "Monokai",
        CodeBlockTheme::OneDark => "One Dark",
        CodeBlockTheme::Gruvbox => "Gruvbox",
        CodeBlockTheme::TokyoNight => "Tokyo Night",
        CodeBlockTheme::Catppuccin => "Catppuccin",
    }
}

/// Get all available themes
fn all_themes() -> Vec<CodeBlockTheme> {
    vec![
        CodeBlockTheme::AyuDark,
        CodeBlockTheme::GitHubDark,
        CodeBlockTheme::Dracula,
        CodeBlockTheme::Nord,
        CodeBlockTheme::Monokai,
        CodeBlockTheme::OneDark,
        CodeBlockTheme::Gruvbox,
        CodeBlockTheme::TokyoNight,
        CodeBlockTheme::Catppuccin,
    ]
}

/// Render the theme picker popup
fn render_theme_picker(frame: &mut ratatui::Frame, app: &App) {
    let area = frame.area();
    let themes = all_themes();

    // Calculate popup size
    let popup_width = 30u16;
    let popup_height = (themes.len() + 2) as u16; // +2 for borders

    // Center the popup
    let popup_area = Rect {
        x: area.width.saturating_sub(popup_width) / 2,
        y: area.height.saturating_sub(popup_height) / 2,
        width: popup_width.min(area.width),
        height: popup_height.min(area.height),
    };

    // Clear the popup area
    frame.render_widget(ratatui::widgets::Clear, popup_area);

    // Build theme list
    let items: Vec<Line> = themes
        .iter()
        .enumerate()
        .map(|(i, theme)| {
            let name = get_theme_name(*theme);
            let is_selected = i == app.theme_picker_index;
            let is_current = *theme == app.markdown_scroll.code_block_theme;

            let prefix = if is_selected { "▶ " } else { "  " };
            let suffix = if is_current { " ✓" } else { "" };

            let style = if is_selected {
                Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)
            } else if is_current {
                Style::default().fg(Color::Green)
            } else {
                Style::default()
            };

            Line::from(Span::styled(format!("{}{}{}", prefix, name, suffix), style))
        })
        .collect();

    let popup = Paragraph::new(items).block(
        Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Style::default().fg(Color::Cyan))
            .title(" Select Theme (j/k, Enter) "),
    );

    frame.render_widget(popup, popup_area);
}
