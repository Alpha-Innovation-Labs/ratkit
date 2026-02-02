//! Primitives tab handler.

use super::TabHandler;
use crate::app::App;
use crossterm::event::{KeyEvent, MouseEvent};

pub struct PrimitivesHandler;

impl TabHandler for PrimitivesHandler {
    fn handle_key(&mut self, _app: &mut App, _key: KeyEvent) {}

    fn handle_mouse(&mut self, _app: &mut App, _mouse: MouseEvent) {}

    fn needs_fast_refresh(&self, _app: &App) -> bool {
        false
    }
}

pub fn render_primitives_demo(
    frame: &mut ratatui::Frame,
    area: ratatui::layout::Rect,
    app: &mut App,
    theme: &ratatui_toolkit::AppTheme,
) {
    use ratatui::layout::Alignment;
    use ratatui::style::{Color, Modifier, Style};
    use ratatui::widgets::{Block, BorderType, Borders, Paragraph};

    let button_area = ratatui::layout::Rect {
        x: area.x + area.width / 2 - 20,
        y: area.y + area.height / 2 - 3,
        width: 40,
        height: 6,
    };

    let title_block = Block::default()
        .title("Button Component Demo")
        .title_alignment(Alignment::Center)
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(theme.border))
        .style(Style::default().fg(theme.text));

    frame.render_widget(&title_block, button_area);

    let inner_area = ratatui::layout::Rect {
        x: button_area.x + 1,
        y: button_area.y + 1,
        width: button_area.width - 2,
        height: button_area.height - 2,
    };

    let button1 = ratatui_toolkit::Button::new("Click Me!");
    let button2 =
        ratatui_toolkit::Button::new("Disabled").normal_style(Style::default().fg(Color::DarkGray));

    let button1_text = format!("[ {} ]", button1.text());
    let button2_text = format!("[ {} ]", button2.text());

    let help_text = Paragraph::new("Buttons are clickable UI components.\nThe first button is interactive, the second is disabled.")
        .alignment(Alignment::Center)
        .style(Style::default().fg(theme.text_muted));

    frame.render_widget(&help_text, inner_area);

    let button1_widget = Paragraph::new(button1_text)
        .alignment(Alignment::Center)
        .style(
            Style::default()
                .fg(theme.primary)
                .add_modifier(Modifier::BOLD),
        );

    frame.render_widget(
        button1_widget,
        ratatui::layout::Rect {
            x: inner_area.x + 2,
            y: inner_area.y + 2,
            width: inner_area.width - 4,
            height: 1,
        },
    );

    let button2_widget = Paragraph::new(button2_text)
        .alignment(Alignment::Center)
        .style(Style::default().fg(Color::DarkGray));

    frame.render_widget(
        button2_widget,
        ratatui::layout::Rect {
            x: inner_area.x + 2,
            y: inner_area.y + 3,
            width: inner_area.width - 4,
            height: 1,
        },
    );
}
