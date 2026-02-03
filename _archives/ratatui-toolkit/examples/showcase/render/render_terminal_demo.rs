//! Render the terminal demo tab.

use ratatui::{
    layout::Rect,
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Paragraph},
};
use ratatui_toolkit::AppTheme;

use crate::app::App;

/// Render the terminal demo.
///
/// # Arguments
///
/// * `frame` - The frame to render into.
/// * `area` - The area to render in.
/// * `app` - The application state.
/// * `theme` - The application theme.
pub fn render_terminal_demo(
    frame: &mut ratatui::Frame,
    area: Rect,
    app: &mut App,
    theme: &AppTheme,
) {
    app.terminal_content_area = Some(area);

    let widget = ratatui_toolkit::primitives::resizable_grid::ResizableGridWidget::new(
        &mut app.terminal_split,
    )
    .with_divider_style(Style::default().fg(theme.secondary))
    .with_hover_style(
        Style::default()
            .fg(theme.secondary)
            .add_modifier(Modifier::BOLD),
    )
    .with_drag_style(
        Style::default()
            .fg(theme.warning)
            .add_modifier(Modifier::BOLD),
    );

    let pane_layouts = widget.pane_layouts(area);

    let left = pane_layouts.get(0).map(|l| l.area()).unwrap_or(area);
    let right = pane_layouts.get(1).map(|l| l.area()).unwrap_or(area);

    if let Some(ref mut term) = app.terminal {
        let inner = Rect {
            x: left.x + 1,
            y: left.y + 1,
            width: left.width.saturating_sub(2),
            height: left.height.saturating_sub(2),
        };
        term.resize(inner.height, inner.width);
    }

    if let Some(ref mut term) = app.terminal2 {
        let inner = Rect {
            x: right.x + 1,
            y: right.y + 1,
            width: right.width.saturating_sub(2),
            height: right.height.saturating_sub(2),
        };
        term.resize(inner.height, inner.width);
    }

    if let Some(ref mut term) = app.terminal {
        term.render(frame, left);
    } else {
        let fallback = Paragraph::new("Terminal failed to spawn").block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .title(" Terminal 1 "),
        );
        frame.render_widget(fallback, left);
    }

    if let Some(ref mut term) = app.terminal2 {
        term.render(frame, right);
    } else {
        let fallback = Paragraph::new("Terminal failed to spawn").block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .title(" Terminal 2 "),
        );
        frame.render_widget(fallback, right);
    }

    let copy_mode_info = if let Some(ref term) = app.terminal {
        if term.copy_mode.is_active() {
            "COPY MODE (hjkl/arrows to move, v to select, y to copy, Esc to exit)"
        } else {
            "Press Ctrl+X to enter copy mode"
        }
    } else {
        ""
    };

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
            Style::default().fg(theme.warning),
        )),
    ])
    .block(
        Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .title(" Info "),
    );
    frame.render_widget(info, right);

    frame.render_widget(widget, area);
}
