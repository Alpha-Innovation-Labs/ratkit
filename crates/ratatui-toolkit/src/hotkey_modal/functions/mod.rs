use ratatui::layout::Rect;
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, BorderType, Borders, Clear, Paragraph, Wrap};
use ratatui::Frame;

use crate::hotkey_modal::{HotkeyModalConfig, HotkeySection};

pub fn render_hotkey_modal(
    frame: &mut Frame,
    sections: &[HotkeySection],
    config: &HotkeyModalConfig,
) {
    let mut lines = Vec::new();

    if config.title_inside {
        lines.push(Line::from(vec![Span::styled(
            &config.title,
            Style::default()
                .add_modifier(Modifier::BOLD)
                .fg(config.border_color),
        )]));
        lines.push(Line::from(""));
    }

    for (i, section) in sections.iter().enumerate() {
        lines.push(Line::from(vec![Span::styled(
            &section.title,
            Style::default().add_modifier(Modifier::BOLD),
        )]));
        lines.push(Line::from(""));

        for hotkey in &section.hotkeys {
            let line = format!("  {:<12}{}", hotkey.key, hotkey.description);
            lines.push(Line::from(line));
        }

        if i < sections.len() - 1 {
            lines.push(Line::from(""));
        }
    }

    if let Some(ref footer) = config.footer {
        lines.push(Line::from(""));
        lines.push(Line::from(vec![Span::styled(
            footer,
            Style::default().fg(Color::DarkGray),
        )]));
    }

    let area = frame.area();

    let overlay_paragraph = Paragraph::new("").style(
        Style::default()
            .bg(Color::Rgb(0, 0, 0))
            .fg(Color::Rgb(40, 40, 40)),
    );
    frame.render_widget(overlay_paragraph, area);

    let popup_width = (area.width as f32 * config.width_percent) as u16;
    let popup_height = (area.height as f32 * config.height_percent) as u16;
    let popup_x = (area.width - popup_width) / 2;
    let popup_y = (area.height - popup_height) / 2;

    let popup_area = Rect {
        x: popup_x,
        y: popup_y,
        width: popup_width,
        height: popup_height,
    };

    frame.render_widget(Clear, popup_area);

    let block = if config.title_inside {
        Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Style::default().fg(config.border_color))
    } else {
        Block::default()
            .title(format!(" {} ", config.title))
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Style::default().fg(config.border_color))
    };

    let paragraph = Paragraph::new(lines)
        .style(Style::default())
        .wrap(Wrap { trim: false });

    frame.render_widget(block, popup_area);

    let inner_area = popup_area.inner(ratatui::layout::Margin {
        horizontal: 2,
        vertical: 2,
    });
    frame.render_widget(paragraph, inner_area);
}
