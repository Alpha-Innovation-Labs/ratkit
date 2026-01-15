//! Showcase Demo - Demonstrates all ratatui-toolkit components
//!
//! Run with: cargo run --example showcase --features full
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
    widgets::{Block, BorderType, Borders, Paragraph, Tabs},
    Terminal,
};
use ratatui_toolkit::{
    render_hotkey_modal, render_markdown, render_toasts, ClickableScrollbar,
    ClickableScrollbarState, Dialog, DialogType, Hotkey, HotkeyFooter, HotkeyItem,
    HotkeyModalConfig, HotkeySection, MenuBar, MenuItem, ResizableSplit, ScrollbarEvent, StatusBar,
    StatusItem, StatusLineStacked, Toast, ToastLevel, ToastManager, TreeNavigator, TreeNode,
    TreeView, TreeViewState, VT100Term, SLANT_BL_TR, SLANT_TL_BR,
};
use std::io;
use std::time::Instant;

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
    Layout,
    Tree,
    Dialogs,
    Markdown,
    Scrollbar,
    StatusLine,
    Terminal,
}

impl DemoTab {
    fn all() -> Vec<Self> {
        vec![
            Self::Layout,
            Self::Tree,
            Self::Dialogs,
            Self::Markdown,
            Self::Scrollbar,
            Self::StatusLine,
            Self::Terminal,
        ]
    }

    fn name(&self) -> &'static str {
        match self {
            Self::Layout => "Layout",
            Self::Tree => "Tree View",
            Self::Dialogs => "Dialogs",
            Self::Markdown => "Markdown",
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

    // Layout demo
    split: ResizableSplit,

    // Tree demo
    tree_state: TreeViewState,
    tree_navigator: TreeNavigator,

    // Dialog demo
    show_dialog: bool,
    dialog_type: DialogType,

    // Hotkey modal
    show_hotkey_modal: bool,

    // Markdown demo
    markdown_scroll: u16,

    // Scrollbar demo
    scrollbar_state: ClickableScrollbarState,
    scroll_content: Vec<String>,

    // StatusLine demo
    status_mode: DemoMode,

    // Terminal demo
    terminal: VT100Term,

    // Toast notifications
    toast_manager: ToastManager,

    // Timing
    start_time: Instant,
}

impl App {
    fn new() -> Self {
        let menu_bar = MenuBar::new(vec![
            MenuItem::with_icon("Layout", "󰙀", 0),
            MenuItem::with_icon("Tree", "", 1),
            MenuItem::with_icon("Dialogs", "󰍉", 2),
            MenuItem::with_icon("Markdown", "", 3),
            MenuItem::with_icon("Scrollbar", "󰍻", 4),
            MenuItem::with_icon("StatusLine", "", 5),
            MenuItem::with_icon("Terminal", "", 6),
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

        // Create terminal
        let terminal = VT100Term::new("Demo Terminal");

        Self {
            current_tab: DemoTab::Layout,
            menu_bar,
            split: ResizableSplit::new(35),
            tree_state,
            tree_navigator: TreeNavigator::new(),
            show_dialog: false,
            dialog_type: DialogType::Info,
            show_hotkey_modal: false,
            markdown_scroll: 0,
            scrollbar_state,
            scroll_content,
            status_mode: DemoMode::Normal,
            terminal,
            toast_manager: ToastManager::new(),
            start_time: Instant::now(),
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
                vec![
                    TreeNode::new(" AlacTerm".to_string()),
                    TreeNode::new(" VT100Term".to_string()),
                ],
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

const SAMPLE_MARKDOWN: &str = r#"# Markdown Rendering

The **MarkdownRenderer** converts markdown to styled `ratatui::Text`.

## Features

- **Bold** and *italic* text
- `Inline code` snippets
- Code blocks with syntax hints

```rust
fn main() {
    println!("Hello, ratatui!");
}
```

## Lists

- First item
- Second item
  - Nested item

> Block quotes are also supported!

---

Press **j/k** to scroll, **Tab** to switch tabs.
"#;

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
                DemoTab::Layout => render_layout_demo(frame, content_area, &mut app),
                DemoTab::Tree => render_tree_demo(frame, content_area, &mut app, &tree_nodes),
                DemoTab::Dialogs => render_dialogs_demo(frame, content_area, &mut app),
                DemoTab::Markdown => render_markdown_demo(frame, content_area, &mut app),
                DemoTab::Scrollbar => render_scrollbar_demo(frame, content_area, &mut app),
                DemoTab::StatusLine => render_statusline_demo(frame, content_area, &mut app),
                DemoTab::Terminal => render_terminal_demo(frame, content_area, &mut app),
            }

            // Status bar
            let elapsed = app.start_time.elapsed().as_secs();
            let status = StatusBar::new()
                .add_left(StatusItem::bold(format!(" {}", app.current_tab.name())))
                .add_center(StatusItem::new("ratatui-toolkit v0.1.0"))
                .add_right(StatusItem::dimmed(format!("{}s", elapsed)));
            frame.render_widget(status, main_chunks[3]);

            // Hotkey footer
            let footer = HotkeyFooter::new(vec![
                HotkeyItem::new("Tab", "switch"),
                HotkeyItem::new("1-7", "tabs"),
                HotkeyItem::new("t", "toast"),
                HotkeyItem::new("?", "help"),
                HotkeyItem::new("q", "quit"),
            ]);
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
                let dialog = Dialog::new(title, message)
                    .dialog_type(app.dialog_type)
                    .width_percent(0.5)
                    .height_percent(0.35);
                frame.render_widget(dialog, area);
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
        })?;

        app.toast_manager.remove_expired();

        if event::poll(std::time::Duration::from_millis(50))? {
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
                        KeyCode::Char('1') => app.select_tab(DemoTab::Layout),
                        KeyCode::Char('2') => app.select_tab(DemoTab::Tree),
                        KeyCode::Char('3') => app.select_tab(DemoTab::Dialogs),
                        KeyCode::Char('4') => app.select_tab(DemoTab::Markdown),
                        KeyCode::Char('5') => app.select_tab(DemoTab::Scrollbar),
                        KeyCode::Char('6') => app.select_tab(DemoTab::StatusLine),
                        KeyCode::Char('7') => app.select_tab(DemoTab::Terminal),
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
                            DemoTab::Markdown => match key.code {
                                KeyCode::Char('j') | KeyCode::Down => {
                                    app.markdown_scroll = app.markdown_scroll.saturating_add(1);
                                }
                                KeyCode::Char('k') | KeyCode::Up => {
                                    app.markdown_scroll = app.markdown_scroll.saturating_sub(1);
                                }
                                _ => {}
                            },
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
                                app.terminal.handle_key(key);
                            }
                            _ => {}
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

                    // Handle split dragging in Layout tab
                    if app.current_tab == DemoTab::Layout {
                        let content_area = Rect {
                            x: area.x,
                            y: area.y + 4,
                            width: area.width,
                            height: area.height.saturating_sub(6),
                        };

                        match mouse.kind {
                            MouseEventKind::Down(MouseButton::Left) => {
                                if app
                                    .split
                                    .is_on_divider(mouse.column, mouse.row, content_area)
                                {
                                    app.split.start_drag();
                                }
                            }
                            MouseEventKind::Drag(MouseButton::Left) => {
                                app.split
                                    .update_from_mouse(mouse.column, mouse.row, content_area);
                            }
                            MouseEventKind::Up(MouseButton::Left) => {
                                app.split.stop_drag();
                            }
                            MouseEventKind::Moved => {
                                app.split.is_hovering =
                                    app.split
                                        .is_on_divider(mouse.column, mouse.row, content_area);
                            }
                            _ => {}
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

fn render_layout_demo(frame: &mut ratatui::Frame, area: Rect, app: &mut App) {
    app.split.update_divider_position(area);

    let left_width = (area.width as u32 * app.split.split_percent as u32 / 100) as u16;
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

    let border_style = if app.split.is_hovering || app.split.is_dragging {
        Style::default().fg(Color::Cyan)
    } else {
        Style::default()
    };

    let left = Paragraph::new(vec![
        Line::from(""),
        Line::from(Span::styled(
            "  ResizableSplit Demo",
            Style::default().add_modifier(Modifier::BOLD),
        )),
        Line::from(""),
        Line::from(format!("  Left panel: {}%", app.split.split_percent)),
        Line::from(""),
        Line::from("  Drag the divider with mouse"),
        Line::from("  to resize panels."),
    ])
    .block(
        Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .title(" Left Panel ")
            .border_style(border_style),
    );

    let right = Paragraph::new(vec![
        Line::from(""),
        Line::from(Span::styled(
            "  Right Panel",
            Style::default().add_modifier(Modifier::BOLD),
        )),
        Line::from(""),
        Line::from(format!("  Right panel: {}%", 100 - app.split.split_percent)),
        Line::from(""),
        Line::from("  The divider highlights when"),
        Line::from("  you hover over it."),
    ])
    .block(
        Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .title(" Right Panel ")
            .border_style(border_style),
    );

    frame.render_widget(left, left_area);
    frame.render_widget(right, right_area);
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
    let text = render_markdown(SAMPLE_MARKDOWN, Some(area.width as usize - 4));

    let paragraph = Paragraph::new(text)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .title(" Markdown Renderer "),
        )
        .scroll((app.markdown_scroll, 0));

    frame.render_widget(paragraph, area);
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
    app.terminal.render(frame, chunks[0]);

    // Info panel
    let info = Paragraph::new(vec![
        Line::from(""),
        Line::from(Span::styled(
            "  VT100Term Features",
            Style::default().add_modifier(Modifier::BOLD),
        )),
        Line::from(""),
        Line::from("  A VT100-compatible terminal"),
        Line::from("  emulator widget."),
        Line::from(""),
        Line::from("  Features:"),
        Line::from("    • ANSI escape sequences"),
        Line::from("    • Color support (16/256)"),
        Line::from("    • Scrollback buffer"),
        Line::from("    • Copy mode (Ctrl+Shift+C)"),
        Line::from("    • Mouse selection"),
        Line::from(""),
        Line::from("  The VT100Term can be connected"),
        Line::from("  to a real PTY process using"),
        Line::from("  spawn_with_command()."),
    ])
    .block(
        Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .title(" Info "),
    );
    frame.render_widget(info, chunks[1]);
}
