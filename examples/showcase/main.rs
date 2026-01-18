//! Showcase Demo - Demonstrates all ratatui-toolkit components
//!
//! Run with: cargo run --example showcase
//! Or: just dev

mod app;
mod constants;
mod demo_mode;
mod demo_tab;
mod helpers;
mod render;

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
    text::Line,
    widgets::Tabs,
    Terminal,
};
use ratatui_toolkit::{
    render_hotkey_modal, render_toasts, ClickableScrollbarStateMouseExt,
    ClickableScrollbarStateScrollExt, Dialog, DialogType, DialogWidget, Hotkey, HotkeyFooter,
    HotkeyItem, HotkeyModalConfig, HotkeySection, MarkdownEvent, MarkdownWidget, ScrollbarEvent,
    StatusBar, StatusItem, Toast, ToastLevel,
};
use std::io;

use app::App;
use demo_tab::DemoTab;
use helpers::{all_themes, get_theme_name};
use render::{
    render_dialogs_demo, render_markdown_demo, render_scrollbar_demo, render_statusline_demo,
    render_terminal_demo, render_theme_picker, render_tree_demo,
};

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
                    app.markdown_scroll.current_line, display_total
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

        // Update cached git stats periodically (handled by scroll manager)
        app.markdown_scroll.update_git_stats();

        // Check for markdown file changes and reload if needed
        if let Some(ref watcher) = app.markdown_file_watcher {
            if watcher.check_for_changes() {
                if let Ok(true) = app.markdown_scroll.reload_source() {
                    app.toast_manager
                        .add(Toast::new("Markdown file reloaded", ToastLevel::Info));
                }
            }
        }

        // Check for pending markdown single-click timeout (for deferred processing)
        if app.current_tab == DemoTab::Markdown {
            let content = app.markdown_scroll.content().unwrap_or("").to_string();
            let mut widget = MarkdownWidget::new(
                &content,
                &mut app.markdown_scroll,
                &mut app.markdown_selection,
                &mut app.markdown_double_click,
            );
            widget.set_rendered_lines(app.markdown_rendered_lines.clone());

            match widget.check_pending_click(app.markdown_inner_area) {
                MarkdownEvent::HeadingToggled { text, .. } => {
                    let display_text = if text.len() > 30 {
                        format!("{}...", &text[..30])
                    } else {
                        text
                    };
                    app.toast_manager.add(Toast::new(
                        &format!("Toggled: {}", display_text),
                        ToastLevel::Info,
                    ));
                }
                MarkdownEvent::FocusedLine { .. } => {
                    // Line focus is handled internally by the widget
                }
                _ => {}
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
                                // Use widget's unified key event handler
                                let content =
                                    app.markdown_scroll.content().unwrap_or("").to_string();
                                let mut widget = MarkdownWidget::new(
                                    &content,
                                    &mut app.markdown_scroll,
                                    &mut app.markdown_selection,
                                    &mut app.markdown_double_click,
                                );
                                widget.set_rendered_lines(app.markdown_rendered_lines.clone());

                                match widget.handle_key_event(key) {
                                    MarkdownEvent::Copied { .. } => {
                                        app.toast_manager.add(Toast::new(
                                            "Copied to clipboard!",
                                            ToastLevel::Success,
                                        ));
                                    }
                                    MarkdownEvent::SelectionEnded => {
                                        // Selection mode exited
                                    }
                                    _ => {}
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
                                    app.status_mode = demo_mode::DemoMode::Normal;
                                }
                                KeyCode::Char('i') => {
                                    app.status_mode = demo_mode::DemoMode::Insert;
                                }
                                KeyCode::Char('v') => {
                                    app.status_mode = demo_mode::DemoMode::Visual;
                                }
                                KeyCode::Char('c') => {
                                    app.status_mode = demo_mode::DemoMode::Command;
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
                                if app
                                    .markdown_split
                                    .is_on_divider(mouse.column, mouse.row, content_area)
                                {
                                    app.markdown_split.start_drag();
                                }
                            }
                            MouseEventKind::Drag(MouseButton::Left) => {
                                if app.markdown_split.is_dragging {
                                    app.markdown_split.update_from_mouse(
                                        mouse.column,
                                        mouse.row,
                                        content_area,
                                    );
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
                                app.markdown_split.is_hovering = app
                                    .markdown_split
                                    .is_on_divider(mouse.column, mouse.row, content_area);

                                // Handle minimap hover
                                let left_width = (content_area.width as u32
                                    * app.markdown_split.split_percent as u32
                                    / 100) as u16;
                                let markdown_area = Rect {
                                    x: content_area.x,
                                    y: content_area.y,
                                    width: left_width,
                                    height: content_area.height,
                                };
                                let inner_area = Rect {
                                    x: markdown_area.x + 1,
                                    y: markdown_area.y + 1,
                                    width: markdown_area.width.saturating_sub(2),
                                    height: markdown_area.height.saturating_sub(2),
                                };

                                let content =
                                    app.markdown_scroll.content().unwrap_or("").to_string();
                                let mut widget = MarkdownWidget::new(
                                    &content,
                                    &mut app.markdown_scroll,
                                    &mut app.markdown_selection,
                                    &mut app.markdown_double_click,
                                )
                                .show_minimap(true);
                                widget.set_minimap_hovered(app.minimap_hovered);

                                if widget.handle_minimap_hover(&mouse, inner_area) {
                                    app.minimap_hovered = widget.is_minimap_hovered();
                                }
                            }
                            _ => {}
                        }

                        // Only handle markdown interactions when NOT dragging divider
                        if !app.markdown_split.is_dragging {
                            // Calculate markdown area based on split percentage
                            let left_width = (content_area.width as u32
                                * app.markdown_split.split_percent as u32
                                / 100) as u16;
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

                            // Store inner area for pending click checks
                            app.markdown_inner_area = inner_area;

                            // Get content before mutable borrows
                            let content =
                                app.markdown_scroll.content().unwrap_or("").to_string();

                            // Use widget's unified mouse event handler
                            let mut widget = MarkdownWidget::new(
                                &content,
                                &mut app.markdown_scroll,
                                &mut app.markdown_selection,
                                &mut app.markdown_double_click,
                            );
                            widget.set_rendered_lines(app.markdown_rendered_lines.clone());

                            match widget.handle_mouse_event(&mouse, inner_area) {
                                MarkdownEvent::Copied { .. } => {
                                    app.toast_manager.add(Toast::new(
                                        "Copied to clipboard!",
                                        ToastLevel::Success,
                                    ));
                                }
                                MarkdownEvent::DoubleClick {
                                    line_number,
                                    line_kind,
                                    content,
                                } => {
                                    let display_content = if content.len() > 40 {
                                        format!("{}...", &content[..40])
                                    } else {
                                        content
                                    };
                                    let msg = format!(
                                        "Line {}: {} - \"{}\"",
                                        line_number, line_kind, display_content
                                    );
                                    app.toast_manager.add(Toast::new(&msg, ToastLevel::Info));
                                }
                                MarkdownEvent::SelectionStarted => {
                                    // Selection mode started
                                }
                                _ => {}
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
