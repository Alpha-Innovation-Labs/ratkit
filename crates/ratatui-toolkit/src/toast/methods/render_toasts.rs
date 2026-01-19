use crate::toast::ToastManager;
use ratatui::layout::{Alignment, Rect};
use ratatui::style::{Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, BorderType, Borders, Clear, Paragraph, Wrap};
use ratatui::Frame;

pub fn render_toasts(frame: &mut Frame, toasts: &ToastManager) {
    let active_toasts = toasts.get_active();
    if active_toasts.is_empty() {
        return;
    }

    let area = frame.area();

    const TOAST_WIDTH: u16 = 40;
    const TOAST_HEIGHT: u16 = 3;
    const TOAST_MARGIN: u16 = 2;
    const TOAST_SPACING: u16 = 1;

    let mut y_offset = area.height.saturating_sub(TOAST_MARGIN);

    for toast in active_toasts.iter().rev() {
        let toast_y = y_offset.saturating_sub(TOAST_HEIGHT);
        let toast_x = area.width.saturating_sub(TOAST_WIDTH + TOAST_MARGIN);

        let toast_area = Rect {
            x: toast_x,
            y: toast_y,
            width: TOAST_WIDTH,
            height: TOAST_HEIGHT,
        };

        if toast_y == 0 || toast_x == 0 {
            break;
        }

        frame.render_widget(Clear, toast_area);

        let color = toast.level.color();
        let icon = toast.level.icon();

        let text = Line::from(vec![
            Span::raw("  "),
            Span::styled(
                icon,
                Style::default().fg(color).add_modifier(Modifier::BOLD),
            ),
            Span::raw("  "),
            Span::raw(&toast.message),
            Span::raw(" "),
        ]);

        let block = Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Style::default().fg(color));

        let paragraph = Paragraph::new(text)
            .block(block)
            .alignment(Alignment::Left)
            .wrap(Wrap { trim: true });

        frame.render_widget(paragraph, toast_area);

        y_offset = toast_y.saturating_sub(TOAST_SPACING);
    }
}
