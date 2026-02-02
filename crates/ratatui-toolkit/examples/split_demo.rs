//! Standalone Split Layout Widget Demo
//!
//! Run with: cargo run --example split_demo --features full

use std::io;

use crossterm::{
    cursor::Show,
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::Rect,
    style::{Modifier, Style},
    text::Line,
    widgets::{Block, BorderType, Borders, Paragraph},
    Terminal,
};
use ratatui_toolkit::{
    primitives::resizable_grid::{ResizableGrid, ResizableGridWidget, ResizableGridWidgetState},
    services::theme::loader::load_builtin_theme,
    AppTheme, ThemeVariant,
};

fn main() -> io::Result<()> {
    let original_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |panic_info| {
        let _ = disable_raw_mode();
        let _ = execute!(io::stdout(), LeaveAlternateScreen, DisableMouseCapture);
        let _ = execute!(io::stdout(), Show);
        original_hook(panic_info);
    }));

    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let theme = load_builtin_theme("ayu", ThemeVariant::Dark).unwrap_or_default();

    let mut split_layout = ResizableGrid::new(0);
    let pane_2 = split_layout.split_pane_horizontally(0).unwrap();
    let _pane_3 = split_layout.split_pane_vertically(pane_2).unwrap();
    let _pane_4 = split_layout.split_pane_vertically(1).unwrap();

    let _ = split_layout.resize_divider(0, 60);
    let _ = split_layout.resize_divider(0, 33);
    let _ = split_layout.resize_divider(pane_2, 50);
    let _ = split_layout.resize_divider(1, 50);

    let mut widget = ResizableGridWidget::new(&mut split_layout)
        .with_divider_style(Style::default().fg(theme.primary));
    let mut widget_state = ResizableGridWidgetState::default();

    let result = run_demo(&mut terminal, &mut widget, &mut widget_state, &theme);

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
    widget: &mut ResizableGridWidget,
    widget_state: &mut ResizableGridWidgetState,
    theme: &AppTheme,
) -> io::Result<()> {
    let mut render_area = Rect::default();
    let pane_labels = [
        ("Pane 0\nTop Left", "Drag borders\nto resize"),
        ("Pane 1\nBottom Left", "Use mouse\nor keys"),
        ("Pane 2\nTop Middle", "Press 'q'\nto quit"),
        ("Pane 3\nTop Right", "Resize\nsupports"),
        ("Pane 4\nBottom Right", "mouse drag"),
    ];

    loop {
        terminal.draw(|frame| {
            render_area = frame.area();

            let render_widget = ResizableGridWidget::new(widget.layout_mut())
                .with_state(*widget_state)
                .with_block(
                    Block::default()
                        .borders(Borders::ALL)
                        .border_type(BorderType::Rounded)
                        .title(" Split Layout Grid (5 panes) "),
                )
                .with_hover_style(
                    Style::default()
                        .fg(theme.accent)
                        .add_modifier(Modifier::BOLD),
                )
                .with_divider_style(Style::default().fg(theme.primary));

            *widget_state = render_widget.state();
            frame.render_widget(render_widget, render_area);

            let pane_layouts = widget.layout().layout_panes(render_area);

            for pane_layout in &pane_layouts {
                let pane_id = pane_layout.pane_id();
                let (title, body) = pane_labels
                    .get(pane_id as usize)
                    .unwrap_or(&("Unknown", ""));

                let content = Paragraph::new(vec![
                    Line::from(vec![ratatui::text::Span::styled(
                        *title,
                        Style::default().add_modifier(Modifier::BOLD),
                    )]),
                    Line::from(""),
                    Line::from(*body),
                ])
                .style(Style::default().fg(theme.text));

                frame.render_widget(content, pane_layout.area());
            }
        })?;

        let poll_timeout = if widget_state.dragging_divider.is_some() {
            std::time::Duration::from_millis(8)
        } else {
            std::time::Duration::from_millis(50)
        };

        if event::poll(poll_timeout)? {
            match event::read()? {
                Event::Key(key) if key.kind == KeyEventKind::Press => {
                    if key.code == KeyCode::Char('q') {
                        return Ok(());
                    }
                }
                Event::Mouse(mouse) => {
                    widget.handle_mouse(mouse, render_area);
                    *widget_state = widget.state();
                }
                _ => {}
            }
        }
    }
}
