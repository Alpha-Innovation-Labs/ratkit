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
    Terminal,
};
use ratatui_toolkit::services::theme::loader::load_builtin_theme;
use ratatui_toolkit::services::theme::persistence::save_theme;
use ratatui_toolkit::{
    render_toasts, Dialog, DialogType, DialogWidget, HotkeyFooter, HotkeyItem, MarkdownEvent,
    MarkdownWidget, ThemeVariant, Toast, ToastLevel,
};
use std::io;

use app::App;
use demo_tab::DemoTab;
use helpers::{all_app_themes, get_app_theme_display_name};
use render::{
    get_filtered_themes, render_code_diff_demo, render_dialogs_demo, render_markdown_demo,
    render_statusline_demo, render_terminal_demo, render_theme_picker, render_trees_demo,
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
    app.toast_manager.add(Toast::new(
        "Press Tab to switch demos",
        ToastLevel::Info,
        None,
    ));

    loop {
        let tree_nodes = app.build_tree();

        terminal.draw(|frame| {
            let area = frame.area();

            // Main layout
            let main_chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Length(3), // Menu bar
                    Constraint::Min(0),    // Content
                    Constraint::Length(1), // Hotkey footer
                ])
                .split(area);

            // Menu bar
            app.menu_bar.render(frame, main_chunks[0]);

            // Content area based on selected tab
            let content_area = main_chunks[1];

            let theme = app.current_theme.clone();
            match app.current_tab {
                DemoTab::Markdown => render_markdown_demo(frame, content_area, &mut app, &theme),
                DemoTab::CodeDiff => render_code_diff_demo(frame, content_area, &app),
                DemoTab::Tree => render_trees_demo(frame, content_area, &mut app, &tree_nodes, &theme),
                DemoTab::Dialogs => render_dialogs_demo(frame, content_area, &mut app, &theme),
                DemoTab::StatusLine => {
                    render_statusline_demo(frame, content_area, &mut app, &theme)
                }
                DemoTab::Terminal => render_terminal_demo(frame, content_area, &mut app, &theme),
            }

            // Hotkey footer with theme
            let footer_items = vec![
                HotkeyItem::new("Tab", "switch"),
                HotkeyItem::new("1-6", "tabs"),
                HotkeyItem::new("T", "theme"),
                HotkeyItem::new("t", "toast"),
                HotkeyItem::new("q", "quit"),
            ];
            let footer = HotkeyFooter::new(footer_items).with_theme(&app.current_theme);
            frame.render_widget(&footer, main_chunks[2]);

            // Toasts
            render_toasts(frame, &app.toast_manager);

            // Dialog overlay with theme
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
                    .height_percent(0.35)
                    .with_theme(&app.current_theme);
                let dialog_widget = DialogWidget::new(&mut dialog);
                frame.render_widget(dialog_widget, area);
            }

            // Theme picker popup
            if app.show_theme_picker {
                render_theme_picker(frame, &app);
            }
        })?;

        app.toast_manager.remove_expired();

        // Update cached git stats periodically
        app.markdown_git_stats
            .update(app.markdown_source.source_path());

        // Check for markdown file changes and reload if needed
        if let Some(ref mut watcher) = app.markdown_file_watcher {
            if watcher.check_for_changes() {
                if let Ok(true) = app.markdown_source.reload_source() {
                    app.markdown_cache.invalidate();
                    app.toast_manager.add(Toast::new(
                        "Markdown file reloaded",
                        ToastLevel::Info,
                        None,
                    ));
                }
            }
        }

        // Check for pending markdown single-click timeout (for deferred processing)
        if app.current_tab == DemoTab::Markdown {
            let content = app.markdown_source.content().unwrap_or("").to_string();
            let mut widget = MarkdownWidget::new(
                &content,
                &mut app.markdown_scroll,
                &mut app.markdown_source,
                &mut app.markdown_cache,
                &app.markdown_display,
                &mut app.markdown_collapse,
                &mut app.markdown_expandable,
                &mut app.markdown_git_stats,
                &mut app.markdown_vim,
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
                        None,
                    ));
                }
                MarkdownEvent::FocusedLine { .. } => {
                    // Line focus is handled internally by the widget
                }
                _ => {}
            }

            // Check for copied text (shows toast)
            if let Some(text) = widget.take_last_copied() {
                let display_text = if text.len() > 30 {
                    format!("{}...", &text[..30])
                } else {
                    text
                };
                app.toast_manager.add(Toast::new(
                    &format!("Copied: {}", display_text),
                    ToastLevel::Success,
                    None,
                ));
            }
        }

        // Adaptive polling: fast during drag for smooth resize, slower otherwise to save CPU
        let poll_timeout = if app.terminal_split.is_dragging || app.code_diff.is_sidebar_dragging()
        {
            std::time::Duration::from_millis(8) // ~120fps for smooth dragging
        } else {
            std::time::Duration::from_millis(50) // Normal rate
        };

        if event::poll(poll_timeout)? {
            match event::read()? {
                Event::Key(key) if key.kind == KeyEventKind::Press => {
                    // Handle modal dismissals first
                    if app.show_dialog {
                        app.show_dialog = false;
                        continue;
                    }
                    // Handle theme picker
                    if app.show_theme_picker {
                        let filtered = get_filtered_themes(&app.theme_filter);
                        match key.code {
                            KeyCode::Esc => {
                                // Cancel: restore original theme and clear filter
                                if let Some(original) = app.original_theme.take() {
                                    app.current_theme = original;
                                }
                                app.show_theme_picker = false;
                                app.theme_filter.clear();
                                app.theme_picker_index = 0;
                            }
                            KeyCode::Char('j') | KeyCode::Down => {
                                if !filtered.is_empty() {
                                    app.theme_picker_index =
                                        (app.theme_picker_index + 1) % filtered.len();
                                    // Apply theme live for preview using original index
                                    if let Some((original_idx, _)) =
                                        filtered.get(app.theme_picker_index)
                                    {
                                        apply_theme_at_index(&mut app, *original_idx);
                                    }
                                }
                            }
                            KeyCode::Char('k') | KeyCode::Up => {
                                if !filtered.is_empty() {
                                    app.theme_picker_index = if app.theme_picker_index == 0 {
                                        filtered.len() - 1
                                    } else {
                                        app.theme_picker_index - 1
                                    };
                                    // Apply theme live for preview using original index
                                    if let Some((original_idx, _)) =
                                        filtered.get(app.theme_picker_index)
                                    {
                                        apply_theme_at_index(&mut app, *original_idx);
                                    }
                                }
                            }
                            KeyCode::Enter => {
                                if let Some((original_idx, theme_name)) =
                                    filtered.get(app.theme_picker_index)
                                {
                                    // Confirm: keep current theme, clear original, save to disk
                                    app.original_theme = None;
                                    app.show_theme_picker = false;
                                    app.saved_theme_index = *original_idx;
                                    // Save the selected theme for persistence
                                    if let Err(e) = save_theme(theme_name, None) {
                                        app.toast_manager.add(Toast::new(
                                            &format!("Failed to save theme: {}", e),
                                            ToastLevel::Warning,
                                            None,
                                        ));
                                    }
                                    app.toast_manager.add(Toast::new(
                                        &format!(
                                            "Theme: {}",
                                            get_app_theme_display_name(theme_name)
                                        ),
                                        ToastLevel::Success,
                                        None,
                                    ));
                                    app.theme_filter.clear();
                                    app.theme_picker_index = 0;
                                }
                            }
                            KeyCode::Backspace => {
                                // Delete last character from filter
                                app.theme_filter.pop();
                                // Reset selection to first match
                                app.theme_picker_index = 0;
                                // Apply first filtered theme for preview
                                let new_filtered = get_filtered_themes(&app.theme_filter);
                                if let Some((original_idx, _)) = new_filtered.first() {
                                    apply_theme_at_index(&mut app, *original_idx);
                                }
                            }
                            KeyCode::Char(c) => {
                                // Add character to filter (except j/k which are navigation)
                                // Only filter on alphanumeric and space
                                if c.is_alphanumeric() || c == ' ' || c == '-' {
                                    app.theme_filter.push(c);
                                    // Reset selection to first match
                                    app.theme_picker_index = 0;
                                    // Apply first filtered theme for preview
                                    let new_filtered = get_filtered_themes(&app.theme_filter);
                                    if let Some((original_idx, _)) = new_filtered.first() {
                                        apply_theme_at_index(&mut app, *original_idx);
                                    }
                                }
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
                        KeyCode::Char('2') => app.select_tab(DemoTab::CodeDiff),
                        KeyCode::Char('3') => app.select_tab(DemoTab::Tree),
                        KeyCode::Char('4') => app.select_tab(DemoTab::Dialogs),
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
                            app.toast_manager.add(Toast::new(
                                messages[idx].0,
                                messages[idx].1,
                                None,
                            ));
                        }
                        KeyCode::Char('T') => {
                            // Open theme picker and store original theme for cancel
                            app.original_theme = Some(app.current_theme.clone());
                            app.show_theme_picker = true;
                            app.theme_filter.clear();
                            app.theme_picker_index = 0;
                        }
                        // Tab-specific keys
                        _ => match app.current_tab {
                            DemoTab::CodeDiff => {
                                // Delegate all key handling to the CodeDiff widget
                                // It handles: [=toggle sidebar, h/l=focus, j/k=nav, g/G=top/bottom, H/L=resize
                                app.code_diff.handle_key(key.code);
                            }
                            DemoTab::Tree => {
                                match app.tree_focus {
                                    app::TreePaneFocus::FileTree => {
                                        if key.code == KeyCode::Char('c') {
                                            app.tree_focus = app::TreePaneFocus::ComponentTree;
                                            continue;
                                        }

                                        if app.file_tree_state.is_filter_mode() {
                                            match key.code {
                                                KeyCode::Esc => {
                                                    app.file_tree_state.clear_filter();
                                                }
                                                KeyCode::Enter => {
                                                    app.file_tree_state.exit_filter_mode();
                                                }
                                                KeyCode::Backspace => {
                                                    app.file_tree_state.backspace_filter();
                                                }
                                                KeyCode::Char(c) => {
                                                    app.file_tree_state.append_to_filter(c);
                                                }
                                                _ => {}
                                            }
                                        } else if key.code == KeyCode::Char('/') {
                                            app.file_tree_state.enter_filter_mode();
                                        } else if let Some(ref file_tree) = app.file_tree {
                                            app.file_tree_navigator.handle_key(
                                                key,
                                                &file_tree.nodes,
                                                &mut app.file_tree_state,
                                            );
                                        }
                                    }
                                    app::TreePaneFocus::ComponentTree => {
                                        if key.code == KeyCode::Char('f') {
                                            app.tree_focus = app::TreePaneFocus::FileTree;
                                            continue;
                                        }

                                        if app.tree_state.is_filter_mode() {
                                            match key.code {
                                                KeyCode::Esc => {
                                                    app.tree_state.clear_filter();
                                                }
                                                KeyCode::Enter => {
                                                    app.tree_state.exit_filter_mode();
                                                }
                                                KeyCode::Backspace => {
                                                    app.tree_state.backspace_filter();
                                                }
                                                KeyCode::Char(c) => {
                                                    app.tree_state.append_to_filter(c);
                                                }
                                                _ => {}
                                            }
                                        } else if key.code == KeyCode::Char('/') {
                                            app.tree_state.enter_filter_mode();
                                        } else {
                                            let tree_nodes = app.build_tree();
                                            app.tree_navigator.handle_key(
                                                key,
                                                &tree_nodes,
                                                &mut app.tree_state,
                                            );
                                        }
                                    }
                                }
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
                                    app.markdown_source.content().unwrap_or("").to_string();
                                let mut widget = MarkdownWidget::new(
                                    &content,
                                    &mut app.markdown_scroll,
                                    &mut app.markdown_source,
                                    &mut app.markdown_cache,
                                    &app.markdown_display,
                                    &mut app.markdown_collapse,
                                    &mut app.markdown_expandable,
                                    &mut app.markdown_git_stats,
                                    &mut app.markdown_vim,
                                    &mut app.markdown_selection,
                                    &mut app.markdown_double_click,
                                );
                                widget.set_rendered_lines(app.markdown_rendered_lines.clone());

                                match widget.handle_key_event(key) {
                                    MarkdownEvent::Copied { .. } => {
                                        app.toast_manager.add(Toast::new(
                                            "Copied to clipboard!",
                                            ToastLevel::Success,
                                            None,
                                        ));
                                    }
                                    MarkdownEvent::SelectionEnded => {
                                        // Selection mode exited
                                    }
                                    _ => {}
                                }
                            }
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
                            if idx == 7 {
                                // Theme button clicked - open theme picker
                                app.original_theme = Some(app.current_theme.clone());
                                app.show_theme_picker = true;
                                app.theme_filter.clear();
                                app.theme_picker_index = 0;
                            } else if idx < DemoTab::all().len() {
                                app.select_tab(DemoTab::all()[idx]);
                            }
                        }
                    }

                    // Handle code diff mouse events (sidebar resize)
                    if app.current_tab == DemoTab::CodeDiff {
                        // Calculate the content area for the code diff
                        let main_chunks = Layout::default()
                            .direction(Direction::Vertical)
                            .constraints([
                                Constraint::Length(3), // Menu bar
                                Constraint::Min(0),    // Content
                                Constraint::Length(1), // Hotkey footer
                            ])
                            .split(area);
                        let content_area = main_chunks[1];

                        // Delegate mouse handling to CodeDiff
                        app.code_diff.handle_mouse(mouse, content_area);
                    }

                    // Handle markdown widget interactions
                    if app.current_tab == DemoTab::Markdown {
                        // Calculate the same area as render_markdown_demo receives
                        let main_chunks = Layout::default()
                            .direction(Direction::Vertical)
                            .constraints([
                                Constraint::Length(3), // Menu bar
                                Constraint::Min(0),    // Content
                                Constraint::Length(1), // Hotkey footer
                            ])
                            .split(area);
                        let content_area = main_chunks[1];

                        // Inner area accounts for widget border
                        let inner_area = Rect {
                            x: content_area.x + 1,
                            y: content_area.y + 1,
                            width: content_area.width.saturating_sub(2),
                            height: content_area.height.saturating_sub(2),
                        };

                        // Get content before mutable borrows
                        let content = app.markdown_source.content().unwrap_or("").to_string();

                        // Handle TOC hover on mouse move
                        if mouse.kind == MouseEventKind::Moved {
                            let mut widget = MarkdownWidget::new(
                                &content,
                                &mut app.markdown_scroll,
                                &mut app.markdown_source,
                                &mut app.markdown_cache,
                                &app.markdown_display,
                                &mut app.markdown_collapse,
                                &mut app.markdown_expandable,
                                &mut app.markdown_git_stats,
                                &mut app.markdown_vim,
                                &mut app.markdown_selection,
                                &mut app.markdown_double_click,
                            )
                            .show_toc(true);
                            widget.set_toc_hovered(app.toc_hovered);
                            widget.set_toc_scroll_offset(app.toc_scroll_offset);

                            if widget.handle_toc_hover(&mouse, inner_area) {
                                app.toc_hovered = widget.is_toc_hovered();
                                app.toc_hovered_entry = widget.get_toc_hovered_entry();
                            }
                        }

                        // Handle TOC click first (for scroll-to-heading navigation)
                        if mouse.kind == MouseEventKind::Down(MouseButton::Left) {
                            let mut widget = MarkdownWidget::new(
                                &content,
                                &mut app.markdown_scroll,
                                &mut app.markdown_source,
                                &mut app.markdown_cache,
                                &app.markdown_display,
                                &mut app.markdown_collapse,
                                &mut app.markdown_expandable,
                                &mut app.markdown_git_stats,
                                &mut app.markdown_vim,
                                &mut app.markdown_selection,
                                &mut app.markdown_double_click,
                            )
                            .show_toc(true);
                            widget.set_toc_hovered(app.toc_hovered);
                            widget.set_toc_scroll_offset(app.toc_scroll_offset);

                            if widget.handle_toc_click(&mouse, inner_area) {
                                app.toc_hovered_entry = widget.get_toc_hovered_entry();
                                continue;
                            }
                        }

                        // Use widget's unified mouse event handler
                        let mut widget = MarkdownWidget::new(
                            &content,
                            &mut app.markdown_scroll,
                            &mut app.markdown_source,
                            &mut app.markdown_cache,
                            &app.markdown_display,
                            &mut app.markdown_collapse,
                            &mut app.markdown_expandable,
                            &mut app.markdown_git_stats,
                            &mut app.markdown_vim,
                            &mut app.markdown_selection,
                            &mut app.markdown_double_click,
                        )
                        .show_toc(true);
                        widget.set_rendered_lines(app.markdown_rendered_lines.clone());
                        widget.set_toc_hovered(app.toc_hovered);
                        widget.set_toc_scroll_offset(app.toc_scroll_offset);

                        let event = widget.handle_mouse_event(&mouse, inner_area);

                        // Update app's TOC scroll offset after handling
                        app.toc_scroll_offset = widget.get_toc_scroll_offset();

                        // Check for copied text (reliable method via selection state)
                        if let Some(copied_text) = widget.take_last_copied() {
                            let display = if copied_text.len() > 30 {
                                format!("{}...", &copied_text[..30])
                            } else {
                                copied_text
                            };
                            app.toast_manager.add(Toast::new(
                                &format!("Copied: {}", display),
                                ToastLevel::Success,
                                None,
                            ));
                        }

                        match event {
                            MarkdownEvent::Copied { .. } => {
                                // Already handled above via take_last_copied
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
                                app.toast_manager
                                    .add(Toast::new(&msg, ToastLevel::Info, None));
                            }
                            MarkdownEvent::SelectionStarted => {
                                // Selection mode started
                            }
                            _ => {}
                        }
                    }

                    // Handle terminal mouse events (scroll, selection, drag)
                    if app.current_tab == DemoTab::Terminal {
                        // Calculate split areas
                        let main_chunks = Layout::default()
                            .direction(Direction::Vertical)
                            .constraints([
                                Constraint::Length(3), // Menu bar
                                Constraint::Min(0),    // Content
                                Constraint::Length(1), // Hotkey footer
                            ])
                            .split(area);
                        let content_area = main_chunks[1];

                        let left_width = (content_area.width as u32
                            * app.terminal_split.split_percent as u32
                            / 100) as u16;

                        let left_terminal_area = Rect {
                            x: content_area.x,
                            y: content_area.y,
                            width: left_width,
                            height: content_area.height,
                        };

                        let _right_terminal_area = Rect {
                            x: content_area.x + left_width,
                            y: content_area.y,
                            width: content_area.width.saturating_sub(left_width),
                            height: content_area.height,
                        };

                        // Handle split divider first
                        match mouse.kind {
                            MouseEventKind::Down(MouseButton::Left) => {
                                if app.terminal_split.is_on_divider(
                                    mouse.column,
                                    mouse.row,
                                    content_area,
                                ) {
                                    app.terminal_split.start_drag();
                                }
                            }
                            MouseEventKind::Drag(MouseButton::Left) => {
                                if app.terminal_split.is_dragging {
                                    app.terminal_split.update_from_mouse(
                                        mouse.column,
                                        mouse.row,
                                        content_area,
                                    );
                                }
                            }
                            MouseEventKind::Up(MouseButton::Left) => {
                                if app.terminal_split.is_dragging {
                                    app.terminal_split.stop_drag();
                                }
                            }
                            MouseEventKind::Moved => {
                                app.terminal_split.is_hovering = app.terminal_split.is_on_divider(
                                    mouse.column,
                                    mouse.row,
                                    content_area,
                                );
                            }
                            _ => {}
                        }

                        // Only handle terminal events when NOT dragging divider
                        if !app.terminal_split.is_dragging {
                            if let Some(ref mut term) = app.terminal {
                                term.handle_mouse(mouse, left_terminal_area);
                            }
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

/// Apply the theme at the given index to the application.
///
/// Loads the theme from builtin themes and applies it live for preview.
/// Also updates the menu bar to use the new theme colors.
///
/// # Arguments
///
/// * `app` - The application state to update
/// * `index` - The index in the builtin themes list
fn apply_theme_at_index(app: &mut App, index: usize) {
    let themes = all_app_themes();
    if let Some(theme_name) = themes.get(index) {
        if let Ok(theme) = load_builtin_theme(theme_name, ThemeVariant::Dark) {
            app.current_theme = theme;
            // Update menu bar to use the new theme
            app.menu_bar.apply_theme(&app.current_theme);
            // Update code diff to use the new theme
            app.code_diff.apply_theme(&app.current_theme);
        }
    }
}
