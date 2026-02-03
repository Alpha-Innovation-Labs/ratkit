//! Main loop handler for the showcase demo.

use crate::app::App;
use crate::demo_tab::DemoTab;
use crate::handlers::{
    ai_chat_handler::AiChatHandler, code_diff_handler::CodeDiffHandler,
    markdown_handler::MarkdownHandler, primitives_handler, primitives_handler::PrimitivesHandler,
    split_grid_handler::SplitGridHandler, terminal_handler::TerminalHandler,
    theme_picker_handler::ThemePickerHandler, tree_handler::TreeHandler, TabHandler,
};
use crate::helpers::render_dialog;
use crate::render::{
    render_ai_chat_demo, render_code_diff_demo, render_markdown_demo,
    render_split_layout_grid_demo, render_terminal_demo, render_theme_picker, render_trees_demo,
};
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
    layout::{Constraint, Direction, Layout},
    Frame, Terminal,
};
use ratatui_toolkit::{render_toasts, HotkeyFooter, HotkeyItem, Toast, ToastLevel, WidgetEvent};
use std::io;

pub fn run_main_loop() -> io::Result<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new();
    app.toast_manager
        .info("Welcome to ratatui-toolkit showcase!");

    let mut handlers: Vec<Box<dyn TabHandler>> = vec![
        Box::new(MarkdownHandler),
        Box::new(CodeDiffHandler),
        Box::new(TreeHandler),
        Box::new(TerminalHandler),
        Box::new(SplitGridHandler),
        Box::new(AiChatHandler),
        Box::new(PrimitivesHandler),
    ];

    let mut theme_picker_handler = ThemePickerHandler;

    loop {
        terminal.draw(|frame| {
            let area = frame.area();

            let main_chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Length(3),
                    Constraint::Min(0),
                    Constraint::Length(1),
                ])
                .split(area);

            app.menu_bar.render(frame, main_chunks[0]);

            let content_area = main_chunks[1];

            let theme = app.current_theme.clone();
            match app.current_tab {
                DemoTab::Markdown => render_markdown_demo(frame, content_area, &mut app, &theme),
                DemoTab::CodeDiff => render_code_diff_demo(frame, content_area, &app),
                DemoTab::Tree => render_trees_demo(frame, content_area, &mut app, &theme),
                DemoTab::Terminal => render_terminal_demo(frame, content_area, &mut app, &theme),
                DemoTab::SplitLayoutGrid => {
                    render_split_layout_grid_demo(frame, content_area, &mut app, &theme)
                }
                DemoTab::AiChat => render_ai_chat_demo(frame, content_area, &mut app, &theme),
                DemoTab::Primitives => primitives_handler::render_primitives_demo(
                    frame,
                    content_area,
                    &mut app,
                    &theme,
                ),
            }

            let footer_items = vec![
                HotkeyItem::new("Tab", "switch"),
                HotkeyItem::new("1-7", "tabs"),
                HotkeyItem::new("T", "theme"),
                HotkeyItem::new("t", "toast"),
                HotkeyItem::new("q", "quit"),
            ];
            let footer = HotkeyFooter::new(footer_items).with_theme(&app.current_theme);
            frame.render_widget(&footer, main_chunks[2]);

            render_toasts(frame, &app.toast_manager);
            render_dialog(frame, &app, area);

            if app.show_theme_picker {
                render_theme_picker(frame, &mut app, &mut app.theme_picker);
            }
        })?;

        app.toast_manager.remove_expired();
        app.markdown_widget.update_git_stats();

        let poll_timeout = get_poll_timeout(&handlers, &app);

        if event::poll(poll_timeout)? {
            match event::read()? {
                Event::Key(key) if key.kind == KeyEventKind::Press => {
                    handle_key_event(&mut app, &key, &handlers, &theme_picker_handler);
                }
                Event::Mouse(mouse) => {
                    handle_mouse_click(&mut app, &mouse);
                    app.menu_bar.update_hover(mouse.column, mouse.row);
                }
                _ => {}
            }
        }
    }

    #[allow(unreachable_code)]
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;
    Ok(())
}

fn get_poll_timeout(handlers: &Vec<Box<dyn TabHandler>>, app: &App) -> std::time::Duration {
    let tab_idx = app.current_tab as usize;
    let needs_fast = handlers
        .get(tab_idx)
        .map(|h| h.needs_fast_refresh(app))
        .unwrap_or(false);
    if needs_fast {
        std::time::Duration::from_millis(8)
    } else {
        std::time::Duration::from_millis(50)
    }
}

fn handle_key_event(
    app: &mut App,
    key: &crossterm::event::KeyEvent,
    handlers: &mut Vec<Box<dyn TabHandler>>,
    theme_picker_handler: &mut ThemePickerHandler,
) {
    if app.show_dialog {
        app.show_dialog = false;
        return;
    }

    if app.show_theme_picker {
        theme_picker_handler.handle_key(app, *key);
        return;
    }

    match key.code {
        KeyCode::Char('q') | KeyCode::Char('Q') => std::process::exit(0),
        KeyCode::Tab => switch_tab(app, 1),
        KeyCode::BackTab => switch_tab(app, -1),
        KeyCode::Char('1') => app.select_tab(DemoTab::Markdown),
        KeyCode::Char('2') => app.select_tab(DemoTab::CodeDiff),
        KeyCode::Char('3') => app.select_tab(DemoTab::Tree),
        KeyCode::Char('4') => app.select_tab(DemoTab::Terminal),
        KeyCode::Char('5') => app.select_tab(DemoTab::SplitLayoutGrid),
        KeyCode::Char('6') => app.select_tab(DemoTab::AiChat),
        KeyCode::Char('7') => app.select_tab(DemoTab::Primitives),
        KeyCode::Char('t') => show_toast(app),
        KeyCode::Char('T') => open_theme_picker(app),
        _ => {
            let tab_idx = app.current_tab as usize;
            if let Some(handler) = handlers.get_mut(tab_idx) {
                handler.handle_key(app, *key);
            }
        }
    }
}

fn switch_tab(app: &mut App, delta: i32) {
    let tabs = DemoTab::all();
    let idx = tabs.iter().position(|t| *t == app.current_tab).unwrap_or(0);
    let next = ((idx as i32 + delta + tabs.len() as i32) % tabs.len() as i32) as usize;
    app.select_tab(tabs[next]);
}

fn show_toast(app: &mut App) {
    let messages = [
        ("Info toast", ToastLevel::Info),
        ("Success!", ToastLevel::Success),
        ("Warning message", ToastLevel::Warning),
        ("Error occurred", ToastLevel::Error),
    ];
    let idx = (app.start_time.elapsed().as_millis() as usize) % messages.len();
    app.toast_manager
        .add(Toast::new(messages[idx].0, messages[idx].1, None));
}

fn open_theme_picker(app: &mut App) {
    app.original_theme = Some(app.current_theme.clone());
    app.show_theme_picker = true;
    app.theme_filter.clear();
    app.theme_picker_index = 0;
}

fn handle_mouse_click(app: &mut App, mouse: &crossterm::event::MouseEvent) {
    match app.menu_bar.handle_mouse(mouse.column, mouse.row) {
        WidgetEvent::MenuSelected { index, action } => {
            if let Some(action) = action {
                action();
            } else {
                handle_menu_selection(app, index);
            }
        }
        _ => {}
    }
}

fn handle_menu_selection(app: &mut App, index: usize) {
    if index == 6 {
        app.select_tab(DemoTab::AiChat);
    } else if index == 7 {
        app.select_tab(DemoTab::Primitives);
    } else if index == 8 {
        open_theme_picker(app);
    } else if index < DemoTab::all().len() {
        app.select_tab(DemoTab::all()[index]);
    }
}
