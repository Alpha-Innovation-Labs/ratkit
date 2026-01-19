//! Standalone Markdown Widget Demo
//!
//! Run with: cargo run --example markdown_demo --features full
//! Or: just demo-mdz

use std::io;

use crossterm::{
    cursor::Show,
    event::{
        self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind, MouseEventKind,
    },
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, layout::Rect, Terminal};
use ratatui_toolkit::{
    markdown_widget::foundation::elements::CodeBlockTheme, render_toasts,
    services::theme::loader::load_builtin_theme, AppTheme, MarkdownState, MarkdownWidget,
    ThemeVariant, Toast, ToastLevel, ToastManager,
};

/// Application state containing the unified markdown state and app-level settings.
struct AppState {
    /// Unified markdown widget state
    markdown: MarkdownState,
    /// Theme for the widget
    theme: AppTheme,
    /// Toast notifications
    toast_manager: ToastManager,
}

fn main() -> io::Result<()> {
    // Install panic hook for cleanup
    let original_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |panic_info| {
        // Cleanup terminal
        let _ = disable_raw_mode();
        let _ = execute!(io::stdout(), LeaveAlternateScreen, DisableMouseCapture);
        let _ = execute!(io::stdout(), Show);
        // Call original hook
        original_hook(panic_info);
    }));

    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Create unified state with custom display settings
    let mut markdown = MarkdownState::default();
    markdown.display.set_show_line_numbers(true);
    markdown.display.set_show_document_line_numbers(true);
    markdown
        .display
        .set_code_block_theme(CodeBlockTheme::AyuDark);
    markdown.display.set_show_heading_collapse(false);
    markdown.git_stats.set_show(true);

    // Load source file
    markdown
        .source
        .set_source_file("crates/ratatui-toolkit/examples/markdown_demo_full.md")?;

    let mut app = AppState {
        markdown,
        theme: load_builtin_theme("ayu", ThemeVariant::Light).unwrap_or_default(),
        toast_manager: ToastManager::new(),
    };

    let result = run_demo(&mut terminal, &mut app);

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    result
}

fn run_demo(
    terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
    app: &mut AppState,
) -> io::Result<()> {
    let mut render_area = Rect::default();

    loop {
        // Update git stats periodically
        app.markdown.update_git_stats();

        // Remove expired toasts
        app.toast_manager.remove_expired();

        // Render
        terminal.draw(|frame| {
            render_area = frame.area();
            let content = app.markdown.content().to_string();

            let widget = MarkdownWidget::from_state(&content, &mut app.markdown)
                .show_toc(true)
                .show_statusline(true)
                .show_scrollbar(true)
                .with_theme(&app.theme);

            frame.render_widget(widget, render_area);
            render_toasts(frame, &app.toast_manager);
        })?;

        // Drain and batch scroll events
        let mut scroll_delta: i32 = 0;
        let mut other_events: Vec<Event> = Vec::new();
        let mut had_events = false;

        while event::poll(std::time::Duration::from_millis(0))? {
            had_events = true;
            let evt = event::read()?;
            match &evt {
                Event::Mouse(mouse) => match mouse.kind {
                    MouseEventKind::ScrollUp => scroll_delta -= 1,
                    MouseEventKind::ScrollDown => scroll_delta += 1,
                    _ => other_events.push(evt),
                },
                _ => other_events.push(evt),
            }
        }

        // Apply batched scroll
        if scroll_delta != 0 {
            let amount =
                scroll_delta.unsigned_abs() as usize * app.markdown.display.scroll_multiplier();
            if scroll_delta < 0 {
                app.markdown.scroll.scroll_up(amount);
            } else {
                app.markdown.scroll.scroll_down(amount);
            }
        }

        // Process other events
        for evt in other_events {
            match evt {
                Event::Key(key) if key.kind == KeyEventKind::Press => {
                    if key.code == KeyCode::Char('q') {
                        return Ok(());
                    }
                    if key.code == KeyCode::Char('c') {
                        // Show info about current highlighted line
                        let content = app.markdown.content().to_string();
                        let widget = MarkdownWidget::from_state(&content, &mut app.markdown);
                        if let Some((line_number, line_kind, line_content)) = widget
                            .get_current_line_info(render_area.width.saturating_sub(2) as usize)
                        {
                            let display_content = if line_content.len() > 40 {
                                format!("{}...", &line_content[..40])
                            } else {
                                line_content
                            };
                            app.toast_manager.add(Toast::new(
                                &format!(
                                    "Line {}: {} - \"{}\"",
                                    line_number, line_kind, display_content
                                ),
                                ToastLevel::Info,
                                None,
                            ));
                        }
                    }
                    // Handle key event
                    let content = app.markdown.content().to_string();
                    let event = {
                        let mut widget = MarkdownWidget::from_state(&content, &mut app.markdown);
                        widget.handle_key_event(key)
                    };

                    // Handle filter mode events
                    use ratatui_toolkit::markdown_widget::foundation::events::MarkdownEvent;
                    match event {
                        MarkdownEvent::FilterModeChanged { active, filter } => {
                            app.markdown.filter_mode = active;
                            app.markdown.filter = Some(filter);
                        }
                        MarkdownEvent::FilterModeExited { line } => {
                            app.markdown.filter_mode = false;
                            app.markdown.filter = None;
                            app.markdown.scroll.current_line = line;
                            app.markdown.scroll.filter_mode = false;
                            app.markdown.scroll.filter = None;
                            // Clear render cache so all content is shown again
                            app.markdown.cache.clear_render_cache();
                        }
                        _ => {}
                    }
                }
                Event::Mouse(mouse) => {
                    // Handle toast click-to-dismiss
                    if matches!(
                        mouse.kind,
                        MouseEventKind::Down(crossterm::event::MouseButton::Left)
                    ) {
                        if app
                            .toast_manager
                            .handle_click(mouse.column, mouse.row, render_area)
                        {
                            continue; // Toast was clicked and dismissed, skip other handling
                        }
                    }

                    let content = app.markdown.content().to_string();
                    let mut sync_state = {
                        let mut widget =
                            MarkdownWidget::from_state(&content, &mut app.markdown).show_toc(true);

                        // Handle all mouse interactions with single widget instance
                        if mouse.kind == MouseEventKind::Moved {
                            widget.handle_toc_hover(&mouse, render_area);
                        }
                        // Handle TOC clicks for navigation
                        widget.handle_toc_click(&mouse, render_area);
                        widget.handle_mouse_event(&mouse, render_area);

                        // Get the state to sync back
                        widget.get_state_sync()
                    };

                    // Check for double-click and show toast
                    if let Some((line_number, line_kind, line_content)) =
                        sync_state.take_double_click()
                    {
                        let display_content = if line_content.len() > 40 {
                            format!("{}...", &line_content[..40])
                        } else {
                            line_content
                        };
                        app.toast_manager.add(Toast::new(
                            &format!(
                                "Line {}: {} - \"{}\"",
                                line_number, line_kind, display_content
                            ),
                            ToastLevel::Info,
                            None,
                        ));
                    }

                    // Sync state back to MarkdownState (widget is now dropped)
                    sync_state.apply_to(&mut app.markdown);
                }
                _ => {}
            }
        }

        if !had_events {
            std::thread::sleep(std::time::Duration::from_millis(50));
        }
    }
}
