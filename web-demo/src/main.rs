//! ratatui-toolkit Web Demo
//!
//! A web-based showcase of ratatui-toolkit components using Ratzilla.
//! Run with: trunk serve

use std::cell::RefCell;
use std::rc::Rc;

use ratzilla::event::{KeyCode, MouseEventKind};
use ratzilla::ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    symbols,
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Scrollbar, ScrollbarOrientation, ScrollbarState, Tabs},
    Frame, Terminal,
};
use ratzilla::{DomBackend, WebRenderer};

// ============================================================================
// Demo Tab Enum
// ============================================================================

#[derive(Clone, Copy, PartialEq, Default)]
enum DemoTab {
    #[default]
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

    fn icon(&self) -> &'static str {
        match self {
            Self::Layout => "󰙀",
            Self::Tree => "",
            Self::Dialogs => "󰍉",
            Self::Markdown => "",
            Self::Scrollbar => "󰍻",
            Self::StatusLine => "",
            Self::Terminal => "",
        }
    }
}

// ============================================================================
// Demo Mode (for StatusLine)
// ============================================================================

#[derive(Clone, Copy, PartialEq, Default)]
enum DemoMode {
    #[default]
    Normal,
    Insert,
    Visual,
    Command,
}

// ============================================================================
// Dialog Type
// ============================================================================

#[derive(Clone, Copy, PartialEq, Default)]
enum DialogType {
    #[default]
    Info,
    Success,
    Warning,
    Error,
    Confirm,
}

// ============================================================================
// Tree Node (simplified)
// ============================================================================

struct TreeNode {
    label: &'static str,
    children: Vec<TreeNode>,
    expanded: bool,
}

impl TreeNode {
    fn new(label: &'static str) -> Self {
        Self {
            label,
            children: vec![],
            expanded: false,
        }
    }

    fn with_children(label: &'static str, children: Vec<TreeNode>) -> Self {
        Self {
            label,
            children,
            expanded: true,
        }
    }
}

// ============================================================================
// App State
// ============================================================================

struct App {
    // Navigation
    current_tab: DemoTab,

    // Layout demo
    split_percent: u8,
    is_dragging: bool,

    // Tree demo
    tree_nodes: Vec<TreeNode>,
    tree_selected: usize,

    // Dialog demo
    show_dialog: bool,
    dialog_type: DialogType,

    // Markdown demo
    markdown_scroll: u16,

    // Scrollbar demo
    scroll_offset: usize,
    scroll_content_len: usize,

    // StatusLine demo
    status_mode: DemoMode,

    // Help modal
    show_help: bool,

    // Frame counter
    frame_count: u32,

    // Debug: last mouse position and event
    last_mouse_x: u32,
    last_mouse_y: u32,
    last_mouse_event: String,
    click_count: u32,
}

impl App {
    fn new() -> Self {
        let tree_nodes = vec![
            TreeNode::with_children(
                " Components",
                vec![
                    TreeNode::new(" Button"),
                    TreeNode::new("󰍉 Dialog"),
                    TreeNode::new(" Toast"),
                    TreeNode::new("󱒅 Pane"),
                ],
            ),
            TreeNode::with_children(
                "󰙀 Layout",
                vec![
                    TreeNode::new("󰯋 ResizableSplit"),
                    TreeNode::new("󰕰 MasterLayout"),
                ],
            ),
            TreeNode::with_children(
                " Widgets",
                vec![
                    TreeNode::new(" TreeView"),
                    TreeNode::new("󰍻 ClickableScrollbar"),
                    TreeNode::new(" MenuBar"),
                    TreeNode::new("󰌌 HotkeyFooter"),
                ],
            ),
            TreeNode::with_children(
                " Rendering",
                vec![TreeNode::new(" MarkdownRenderer")],
            ),
            TreeNode::with_children(
                " Terminal",
                vec![
                    TreeNode::new(" AlacTerm"),
                    TreeNode::new(" VT100Term"),
                ],
            ),
        ];

        Self {
            current_tab: DemoTab::Layout,
            split_percent: 50,
            is_dragging: false,
            tree_nodes,
            tree_selected: 0,
            show_dialog: false,
            dialog_type: DialogType::Info,
            markdown_scroll: 0,
            scroll_offset: 0,
            scroll_content_len: 100,
            status_mode: DemoMode::Normal,
            show_help: false,
            frame_count: 0,
            last_mouse_x: 0,
            last_mouse_y: 0,
            last_mouse_event: String::from("none"),
            click_count: 0,
        }
    }

    fn select_tab(&mut self, tab: DemoTab) {
        self.current_tab = tab;
    }

    fn next_tab(&mut self) {
        let tabs = DemoTab::all();
        let idx = tabs.iter().position(|t| *t == self.current_tab).unwrap_or(0);
        let next = (idx + 1) % tabs.len();
        self.current_tab = tabs[next];
    }

    fn prev_tab(&mut self) {
        let tabs = DemoTab::all();
        let idx = tabs.iter().position(|t| *t == self.current_tab).unwrap_or(0);
        let prev = if idx == 0 { tabs.len() - 1 } else { idx - 1 };
        self.current_tab = tabs[prev];
    }
}

// ============================================================================
// Main
// ============================================================================

fn main() -> std::io::Result<()> {
    let app = Rc::new(RefCell::new(App::new()));

    let backend = DomBackend::new()?;
    let terminal = Terminal::new(backend)?;

    // Key event handler
    terminal.on_key_event({
        let app = app.clone();
        move |event| {
            let mut app = app.borrow_mut();

            // Handle help modal dismissal
            if app.show_help {
                app.show_help = false;
                return;
            }

            // Handle dialog dismissal
            if app.show_dialog {
                app.show_dialog = false;
                return;
            }

            match event.code {
                // Global navigation
                KeyCode::Tab => app.next_tab(),
                KeyCode::Char('`') => app.prev_tab(), // Use backtick for prev tab
                KeyCode::Char('1') => app.select_tab(DemoTab::Layout),
                KeyCode::Char('2') => app.select_tab(DemoTab::Tree),
                KeyCode::Char('3') => app.select_tab(DemoTab::Dialogs),
                KeyCode::Char('4') => app.select_tab(DemoTab::Markdown),
                KeyCode::Char('5') => app.select_tab(DemoTab::Scrollbar),
                KeyCode::Char('6') => app.select_tab(DemoTab::StatusLine),
                KeyCode::Char('7') => app.select_tab(DemoTab::Terminal),
                KeyCode::Char('?') => app.show_help = !app.show_help,

                // Tab-specific keys
                _ => match app.current_tab {
                    DemoTab::Layout => match event.code {
                        KeyCode::Left => {
                            app.split_percent = app.split_percent.saturating_sub(5).max(10);
                        }
                        KeyCode::Right => {
                            app.split_percent = (app.split_percent + 5).min(90);
                        }
                        KeyCode::Char('h') => {
                            app.split_percent = app.split_percent.saturating_sub(5).max(10);
                        }
                        KeyCode::Char('l') => {
                            app.split_percent = (app.split_percent + 5).min(90);
                        }
                        _ => {}
                    },
                    DemoTab::Tree => match event.code {
                        KeyCode::Down | KeyCode::Char('j') => {
                            app.tree_selected = (app.tree_selected + 1).min(15);
                        }
                        KeyCode::Up | KeyCode::Char('k') => {
                            app.tree_selected = app.tree_selected.saturating_sub(1);
                        }
                        _ => {}
                    },
                    DemoTab::Dialogs => match event.code {
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
                    DemoTab::Markdown => match event.code {
                        KeyCode::Down | KeyCode::Char('j') => {
                            app.markdown_scroll = app.markdown_scroll.saturating_add(1);
                        }
                        KeyCode::Up | KeyCode::Char('k') => {
                            app.markdown_scroll = app.markdown_scroll.saturating_sub(1);
                        }
                        _ => {}
                    },
                    DemoTab::Scrollbar => match event.code {
                        KeyCode::Down | KeyCode::Char('j') => {
                            app.scroll_offset = (app.scroll_offset + 1).min(app.scroll_content_len - 1);
                        }
                        KeyCode::Up | KeyCode::Char('k') => {
                            app.scroll_offset = app.scroll_offset.saturating_sub(1);
                        }
                        KeyCode::PageDown => {
                            app.scroll_offset = (app.scroll_offset + 10).min(app.scroll_content_len - 1);
                        }
                        KeyCode::PageUp => {
                            app.scroll_offset = app.scroll_offset.saturating_sub(10);
                        }
                        _ => {}
                    },
                    DemoTab::StatusLine => match event.code {
                        KeyCode::Char('n') => app.status_mode = DemoMode::Normal,
                        KeyCode::Char('i') => app.status_mode = DemoMode::Insert,
                        KeyCode::Char('v') => app.status_mode = DemoMode::Visual,
                        KeyCode::Char('c') => app.status_mode = DemoMode::Command,
                        _ => {}
                    },
                    DemoTab::Terminal => {
                        // Terminal tab - info only in web version
                    }
                },
            }
        }
    });

    // Mouse event handler
    terminal.on_mouse_event({
        let app = app.clone();
        move |event| {
            let mut app = app.borrow_mut();

            // Track all mouse events for debugging
            app.last_mouse_x = event.x;
            app.last_mouse_y = event.y;

            match event.event {
                MouseEventKind::Pressed => {
                    app.last_mouse_event = format!("CLICK at ({},{})", event.x, event.y);
                    app.click_count += 1;

                    // Check menu bar clicks (row 0-2)
                    if event.y < 3 {
                        let tab_width = 12u32;
                        let tab_idx = (event.x / tab_width) as usize;
                        let tabs = DemoTab::all();
                        if tab_idx < tabs.len() {
                            app.select_tab(tabs[tab_idx]);
                        }
                    }

                    // Layout tab - check divider
                    if app.current_tab == DemoTab::Layout && event.y >= 5 {
                        app.is_dragging = true;
                    }
                }
                MouseEventKind::Released => {
                    app.last_mouse_event = format!("RELEASE at ({},{})", event.x, event.y);
                    app.is_dragging = false;
                }
                MouseEventKind::Moved => {
                    app.last_mouse_event = format!("MOVE at ({},{})", event.x, event.y);

                    // Handle dragging while mouse is moving
                    if app.current_tab == DemoTab::Layout && app.is_dragging {
                        // Update split based on mouse X position
                        // Approximate: assume 120 char width
                        let percent = (event.x as u32 * 100 / 120) as u8;
                        app.split_percent = percent.clamp(10, 90);
                    }
                }
                _ => {
                    app.last_mouse_event = String::from("OTHER");
                }
            }
        }
    });

    // Render loop
    terminal.draw_web({
        let app = app.clone();
        move |frame| {
            let mut app = app.borrow_mut();
            app.frame_count += 1;

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
            render_menu_bar(frame, main_chunks[0], &app);

            // Tab bar
            render_tab_bar(frame, main_chunks[1], &app);

            // Content
            let content_area = main_chunks[2];
            match app.current_tab {
                DemoTab::Layout => render_layout_demo(frame, content_area, &app),
                DemoTab::Tree => render_tree_demo(frame, content_area, &app),
                DemoTab::Dialogs => render_dialogs_demo(frame, content_area, &app),
                DemoTab::Markdown => render_markdown_demo(frame, content_area, &app),
                DemoTab::Scrollbar => render_scrollbar_demo(frame, content_area, &mut app),
                DemoTab::StatusLine => render_statusline_demo(frame, content_area, &app),
                DemoTab::Terminal => render_terminal_demo(frame, content_area, &app),
            }

            // Status bar
            render_status_bar(frame, main_chunks[3], &app);

            // Hotkey footer
            render_hotkey_footer(frame, main_chunks[4]);

            // Dialog overlay
            if app.show_dialog {
                render_dialog(frame, area, &app);
            }

            // Help modal overlay
            if app.show_help {
                render_help_modal(frame, area);
            }
        }
    });

    Ok(())
}

// ============================================================================
// Render Functions
// ============================================================================

fn render_menu_bar(frame: &mut Frame, area: Rect, app: &App) {
    let tabs = DemoTab::all();
    let menu_items: Vec<Span> = tabs
        .iter()
        .map(|tab| {
            let is_selected = *tab == app.current_tab;
            let style = if is_selected {
                Style::default()
                    .fg(Color::Black)
                    .bg(Color::Cyan)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Color::White)
            };
            Span::styled(format!(" {} {} ", tab.icon(), tab.name()), style)
        })
        .collect();

    let menu_line = Line::from(menu_items);
    let menu = Paragraph::new(menu_line).block(
        Block::default()
            .borders(Borders::BOTTOM)
            .border_style(Style::default().fg(Color::DarkGray)),
    );
    frame.render_widget(menu, area);
}

fn render_tab_bar(frame: &mut Frame, area: Rect, app: &App) {
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
    frame.render_widget(tab_widget, area);
}

fn render_status_bar(frame: &mut Frame, area: Rect, app: &App) {
    let left = Span::styled(
        format!("  {} ", app.current_tab.name()),
        Style::default()
            .fg(Color::Black)
            .bg(Color::Cyan)
            .add_modifier(Modifier::BOLD),
    );

    // Show mouse debug info
    let mouse_info = Span::styled(
        format!(
            " Mouse: ({},{}) | {} | Clicks: {} ",
            app.last_mouse_x, app.last_mouse_y, app.last_mouse_event, app.click_count
        ),
        Style::default().fg(Color::Yellow),
    );

    let right = Span::styled(
        format!(" Frame {} ", app.frame_count),
        Style::default().fg(Color::DarkGray),
    );

    let status = Paragraph::new(Line::from(vec![
        left,
        Span::raw(" "),
        mouse_info,
        Span::raw(format!(
            "{:width$}",
            "",
            width = area.width.saturating_sub(80) as usize
        )),
        right,
    ]));
    frame.render_widget(status, area);
}

fn render_hotkey_footer(frame: &mut Frame, area: Rect) {
    let hotkeys = vec![
        ("Tab", "switch"),
        ("1-7", "tabs"),
        ("?", "help"),
        ("h/l", "resize"),
    ];

    let spans: Vec<Span> = hotkeys
        .iter()
        .flat_map(|(key, desc)| {
            vec![
                Span::styled(
                    format!(" {} ", key),
                    Style::default().fg(Color::Black).bg(Color::DarkGray),
                ),
                Span::styled(format!(" {} ", desc), Style::default().fg(Color::DarkGray)),
            ]
        })
        .collect();

    let footer = Paragraph::new(Line::from(spans));
    frame.render_widget(footer, area);
}

fn render_layout_demo(frame: &mut Frame, area: Rect, app: &App) {
    let left_width = (area.width as u32 * app.split_percent as u32 / 100) as u16;
    let left_area = Rect {
        x: area.x,
        y: area.y,
        width: left_width.saturating_sub(1),
        height: area.height,
    };
    let right_area = Rect {
        x: area.x + left_width,
        y: area.y,
        width: area.width.saturating_sub(left_width),
        height: area.height,
    };

    let border_style = if app.is_dragging {
        Style::default().fg(Color::Cyan)
    } else {
        Style::default().fg(Color::Yellow)
    };

    let left = Paragraph::new(vec![
        Line::from(""),
        Line::from(Span::styled(
            "  ResizableSplit Demo",
            Style::default().add_modifier(Modifier::BOLD),
        )),
        Line::from(""),
        Line::from(format!("  Left panel: {}%", app.split_percent)),
        Line::from(""),
        Line::from("  Drag the divider with mouse"),
        Line::from("  to resize panels."),
        Line::from(""),
        Line::from("  Or use h/l or ←/→ keys"),
    ])
    .block(
        Block::default()
            .borders(Borders::ALL)
            .border_set(symbols::border::ROUNDED)
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
        Line::from(format!("  Right panel: {}%", 100 - app.split_percent)),
        Line::from(""),
        Line::from("  The divider highlights when"),
        Line::from("  you hover over it."),
    ])
    .block(
        Block::default()
            .borders(Borders::ALL)
            .border_set(symbols::border::ROUNDED)
            .title(" Right Panel ")
            .border_style(border_style),
    );

    frame.render_widget(left, left_area);
    frame.render_widget(right, right_area);
}

fn render_tree_demo(frame: &mut Frame, area: Rect, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(area);

    // Tree content - flattened view
    let mut tree_lines: Vec<Line> = vec![];
    let mut line_idx = 0usize;

    for node in &app.tree_nodes {
        let is_selected = line_idx == app.tree_selected;
        let style = if is_selected {
            Style::default()
                .fg(Color::Cyan)
                .bg(Color::DarkGray)
                .add_modifier(Modifier::BOLD)
        } else {
            Style::default()
        };
        let prefix = if node.children.is_empty() { "  " } else { "▸ " };
        tree_lines.push(Line::styled(format!("{}{}", prefix, node.label), style));
        line_idx += 1;

        if node.expanded {
            for child in &node.children {
                let is_selected = line_idx == app.tree_selected;
                let style = if is_selected {
                    Style::default()
                        .fg(Color::Cyan)
                        .bg(Color::DarkGray)
                        .add_modifier(Modifier::BOLD)
                } else {
                    Style::default().fg(Color::White)
                };
                tree_lines.push(Line::styled(format!("    {}", child.label), style));
                line_idx += 1;
            }
        }
    }

    let tree = Paragraph::new(tree_lines).block(
        Block::default()
            .borders(Borders::ALL)
            .border_set(symbols::border::ROUNDED)
            .title(" TreeView - Component Browser ")
            .border_style(Style::default().fg(Color::Yellow)),
    );

    frame.render_widget(tree, chunks[0]);

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
        Line::from(""),
        Line::from("  Features:"),
        Line::from("    • Generic data type"),
        Line::from("    • Custom render function"),
        Line::from("    • Configurable keybindings"),
    ])
    .block(
        Block::default()
            .borders(Borders::ALL)
            .border_set(symbols::border::ROUNDED)
            .title(" Controls ")
            .border_style(Style::default().fg(Color::Yellow)),
    );

    frame.render_widget(info, chunks[1]);
}

fn render_dialogs_demo(frame: &mut Frame, area: Rect, _app: &App) {
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
    ])
    .block(
        Block::default()
            .borders(Borders::ALL)
            .border_set(symbols::border::ROUNDED)
            .title(" Dialogs Demo ")
            .border_style(Style::default().fg(Color::Yellow)),
    );

    frame.render_widget(content, area);
}

fn render_markdown_demo(frame: &mut Frame, area: Rect, app: &App) {
    let markdown_content = vec![
        Line::from(Span::styled(
            "# Markdown Rendering",
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )),
        Line::from(""),
        Line::from(vec![
            Span::raw("The "),
            Span::styled("**MarkdownRenderer**", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw(" converts markdown to styled "),
            Span::styled("`ratatui::Text`", Style::default().fg(Color::Green)),
            Span::raw("."),
        ]),
        Line::from(""),
        Line::from(Span::styled(
            "## Features",
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )),
        Line::from(""),
        Line::from(vec![
            Span::raw("- "),
            Span::styled("**Bold**", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw(" and "),
            Span::styled("*italic*", Style::default().add_modifier(Modifier::ITALIC)),
            Span::raw(" text"),
        ]),
        Line::from(vec![
            Span::raw("- "),
            Span::styled("`Inline code`", Style::default().fg(Color::Green)),
            Span::raw(" snippets"),
        ]),
        Line::from("- Code blocks with syntax hints"),
        Line::from(""),
        Line::from(Span::styled(
            "```rust",
            Style::default().fg(Color::DarkGray),
        )),
        Line::from(Span::styled(
            "fn main() {",
            Style::default().fg(Color::Green),
        )),
        Line::from(Span::styled(
            "    println!(\"Hello, ratatui!\");",
            Style::default().fg(Color::Green),
        )),
        Line::from(Span::styled("}", Style::default().fg(Color::Green))),
        Line::from(Span::styled("```", Style::default().fg(Color::DarkGray))),
        Line::from(""),
        Line::from(Span::styled(
            "## Lists",
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )),
        Line::from(""),
        Line::from("- First item"),
        Line::from("- Second item"),
        Line::from("  - Nested item"),
        Line::from(""),
        Line::from(Span::styled(
            "> Block quotes are also supported!",
            Style::default().fg(Color::DarkGray).add_modifier(Modifier::ITALIC),
        )),
        Line::from(""),
        Line::from("───────────────────────────────────"),
        Line::from(""),
        Line::from(vec![
            Span::raw("Press "),
            Span::styled("j/k", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw(" to scroll, "),
            Span::styled("Tab", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw(" to switch tabs."),
        ]),
    ];

    let paragraph = Paragraph::new(markdown_content)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_set(symbols::border::ROUNDED)
                .title(format!(" Markdown Renderer (scroll: {}) ", app.markdown_scroll))
                .border_style(Style::default().fg(Color::Yellow)),
        )
        .scroll((app.markdown_scroll, 0));

    frame.render_widget(paragraph, area);
}

fn render_scrollbar_demo(frame: &mut Frame, area: Rect, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Min(0), Constraint::Length(1)])
        .split(area);

    let visible_height = chunks[0].height.saturating_sub(2) as usize;

    // Content
    let visible_lines: Vec<Line> = (0..app.scroll_content_len)
        .skip(app.scroll_offset)
        .take(visible_height)
        .map(|i| {
            Line::from(format!(
                "Line {}: This is content for the scrollbar demonstration",
                i + 1
            ))
        })
        .collect();

    let content = Paragraph::new(visible_lines).block(
        Block::default()
            .borders(Borders::ALL)
            .border_set(symbols::border::ROUNDED)
            .title(format!(
                " ClickableScrollbar - Line {}/{} ",
                app.scroll_offset + 1,
                app.scroll_content_len
            ))
            .border_style(Style::default().fg(Color::Yellow)),
    );

    frame.render_widget(content, chunks[0]);

    // Scrollbar
    let mut scrollbar_state = ScrollbarState::new(app.scroll_content_len)
        .position(app.scroll_offset)
        .viewport_content_length(visible_height);

    let scrollbar = Scrollbar::new(ScrollbarOrientation::VerticalRight)
        .begin_symbol(Some("↑"))
        .end_symbol(Some("↓"));

    frame.render_stateful_widget(scrollbar, chunks[1], &mut scrollbar_state);
}

fn render_statusline_demo(frame: &mut Frame, area: Rect, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(0),
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Length(1),
        ])
        .split(area);

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
            .border_set(symbols::border::ROUNDED)
            .title(" StatusLineStacked Demo ")
            .border_style(Style::default().fg(Color::Yellow)),
    );
    frame.render_widget(content, chunks[0]);

    // Status line style 1
    let (mode_color, mode_text) = match app.status_mode {
        DemoMode::Normal => (Color::Blue, " NORMAL "),
        DemoMode::Insert => (Color::Green, " INSERT "),
        DemoMode::Visual => (Color::Magenta, " VISUAL "),
        DemoMode::Command => (Color::Yellow, " COMMAND "),
    };

    let status1 = Paragraph::new(Line::from(vec![
        Span::styled(
            mode_text,
            Style::default().fg(Color::Black).bg(mode_color),
        ),
        Span::styled("", Style::default().fg(mode_color).bg(Color::DarkGray)),
        Span::styled(" main ", Style::default().fg(Color::White).bg(Color::DarkGray)),
        Span::styled("", Style::default().fg(Color::DarkGray)),
        Span::raw("  showcase.rs"),
        Span::raw(format!("{:width$}", "", width = 60)),
        Span::styled("", Style::default().fg(Color::Cyan)),
        Span::styled(" UTF-8 ", Style::default().fg(Color::Black).bg(Color::Cyan)),
    ]));
    frame.render_widget(status1, chunks[1]);

    // Status line style 2
    let status2 = Paragraph::new(Line::from(vec![
        Span::styled("  rust ", Style::default().fg(Color::Black).bg(Color::Red)),
        Span::styled("", Style::default().fg(Color::Red).bg(Color::Gray)),
        Span::styled(" src/lib.rs ", Style::default().fg(Color::Black).bg(Color::Gray)),
        Span::styled("", Style::default().fg(Color::Gray)),
        Span::raw("  ratatui-toolkit v0.1.0"),
        Span::raw(format!("{:width$}", "", width = 50)),
        Span::styled("", Style::default().fg(Color::Green).bg(Color::Gray)),
        Span::styled(" Ln 42 ", Style::default().fg(Color::Black).bg(Color::Green)),
        Span::styled("", Style::default().fg(Color::Gray).bg(Color::Green)),
        Span::styled(" Col 8 ", Style::default().fg(Color::Black).bg(Color::Gray)),
    ]));
    frame.render_widget(status2, chunks[2]);

    // Status line style 3 (minimal)
    let status3 = Paragraph::new(Line::from(vec![
        Span::styled(" 󰈙 ", Style::default().fg(Color::Cyan)),
        Span::raw("  Press ? for help"),
        Span::raw(format!("{:width$}", "", width = 70)),
        Span::styled(" 100% ", Style::default().fg(Color::Green)),
    ]));
    frame.render_widget(status3, chunks[3]);
}

fn render_terminal_demo(frame: &mut Frame, area: Rect, _app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(60), Constraint::Percentage(40)])
        .split(area);

    // Terminal placeholder (can't run real terminal in WASM)
    let terminal_content = vec![
        Line::from(""),
        Line::from(Span::styled(
            "  Demo Terminal",
            Style::default().add_modifier(Modifier::BOLD),
        )),
        Line::from(""),
        Line::styled(
            "  $ echo 'Hello from ratatui-toolkit!'",
            Style::default().fg(Color::Green),
        ),
        Line::styled(
            "  Hello from ratatui-toolkit!",
            Style::default().fg(Color::White),
        ),
        Line::from(""),
        Line::styled(
            "  $ cargo run --example showcase",
            Style::default().fg(Color::Green),
        ),
        Line::styled(
            "     Compiling ratatui-toolkit v0.1.0",
            Style::default().fg(Color::Cyan),
        ),
        Line::styled(
            "      Finished dev [unoptimized]",
            Style::default().fg(Color::Green),
        ),
        Line::styled(
            "       Running `showcase`",
            Style::default().fg(Color::Green),
        ),
        Line::from(""),
        Line::styled("  █", Style::default().fg(Color::White)),
    ];

    let terminal = Paragraph::new(terminal_content).block(
        Block::default()
            .borders(Borders::ALL)
            .border_set(symbols::border::ROUNDED)
            .title(" VT100Term ")
            .border_style(Style::default().fg(Color::Yellow)),
    );

    frame.render_widget(terminal, chunks[0]);

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
        Line::from("    • Copy mode"),
        Line::from("    • Mouse selection"),
        Line::from(""),
        Line::from(Span::styled(
            "  Note: Real terminal emulation",
            Style::default().fg(Color::DarkGray),
        )),
        Line::from(Span::styled(
            "  requires native environment.",
            Style::default().fg(Color::DarkGray),
        )),
    ])
    .block(
        Block::default()
            .borders(Borders::ALL)
            .border_set(symbols::border::ROUNDED)
            .title(" Info ")
            .border_style(Style::default().fg(Color::Yellow)),
    );

    frame.render_widget(info, chunks[1]);
}

fn render_dialog(frame: &mut Frame, area: Rect, app: &App) {
    let (title, message, color) = match app.dialog_type {
        DialogType::Info => ("Information", "This is an info dialog.\n\nPress any key to close.", Color::Cyan),
        DialogType::Success => ("Success!", "Operation completed successfully!", Color::Green),
        DialogType::Warning => ("Warning", "This action may have consequences.", Color::Yellow),
        DialogType::Error => ("Error", "Something went wrong!", Color::Red),
        DialogType::Confirm => ("Confirm", "Do you want to proceed?", Color::Blue),
    };

    // Dialog dimensions
    let dialog_width = area.width * 50 / 100;
    let dialog_height = 10;
    let dialog_x = (area.width - dialog_width) / 2;
    let dialog_y = (area.height - dialog_height) / 2;

    let dialog_area = Rect {
        x: dialog_x,
        y: dialog_y,
        width: dialog_width,
        height: dialog_height,
    };

    // Clear background (semi-transparent effect via rendering over)
    let clear_bg = Block::default().style(Style::default().bg(Color::Black));
    frame.render_widget(clear_bg, area);

    // Dialog
    let dialog = Paragraph::new(vec![
        Line::from(""),
        Line::from(Span::styled(
            format!("  {}", title),
            Style::default().fg(color).add_modifier(Modifier::BOLD),
        )),
        Line::from(""),
        Line::from(format!("  {}", message)),
        Line::from(""),
        Line::from(""),
        Line::from(Span::styled(
            "  [ OK ]",
            Style::default().fg(Color::Black).bg(Color::White),
        )),
    ])
    .block(
        Block::default()
            .borders(Borders::ALL)
            .border_set(symbols::border::DOUBLE)
            .border_style(Style::default().fg(color)),
    );

    frame.render_widget(dialog, dialog_area);
}

fn render_help_modal(frame: &mut Frame, area: Rect) {
    let modal_width = area.width * 60 / 100;
    let modal_height = 20;
    let modal_x = (area.width - modal_width) / 2;
    let modal_y = (area.height - modal_height) / 2;

    let modal_area = Rect {
        x: modal_x,
        y: modal_y,
        width: modal_width,
        height: modal_height,
    };

    // Clear background
    let clear_bg = Block::default().style(Style::default().bg(Color::Black));
    frame.render_widget(clear_bg, area);

    // Help content
    let help = Paragraph::new(vec![
        Line::from(Span::styled(
            "  Keyboard Shortcuts",
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )),
        Line::from(""),
        Line::from(Span::styled("  Navigation", Style::default().add_modifier(Modifier::BOLD))),
        Line::from("    Tab        Next tab"),
        Line::from("    Shift+Tab  Previous tab"),
        Line::from("    1-7        Jump to tab"),
        Line::from(""),
        Line::from(Span::styled("  Tree View", Style::default().add_modifier(Modifier::BOLD))),
        Line::from("    j/↓        Move down"),
        Line::from("    k/↑        Move up"),
        Line::from("    l/→        Expand"),
        Line::from("    h/←        Collapse"),
        Line::from(""),
        Line::from(Span::styled("  General", Style::default().add_modifier(Modifier::BOLD))),
        Line::from("    ?          Toggle help"),
        Line::from(""),
        Line::from(Span::styled(
            "  Press any key to close",
            Style::default().fg(Color::DarkGray),
        )),
    ])
    .block(
        Block::default()
            .borders(Borders::ALL)
            .border_set(symbols::border::DOUBLE)
            .title(" Help ")
            .border_style(Style::default().fg(Color::Cyan)),
    );

    frame.render_widget(help, modal_area);
}
