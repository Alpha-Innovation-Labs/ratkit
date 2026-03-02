use crate::primitives::dialog::types::Dialog;
use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Clear, Paragraph, Widget, Wrap},
};
use unicode_width::UnicodeWidthStr;

pub struct DialogWidget<'a, 'b> {
    pub dialog: &'a mut Dialog<'b>,
}

impl<'a, 'b> DialogWidget<'a, 'b> {
    pub fn new(dialog: &'a mut Dialog<'b>) -> Self {
        Self { dialog }
    }
}

impl Widget for DialogWidget<'_, '_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        if let Some(backdrop_style) = self.dialog.backdrop_style {
            let overlay = Paragraph::new("").style(backdrop_style);
            overlay.render(area, buf);
        }

        let dialog_width = (area.width as f32 * self.dialog.width_percent) as u16;
        let dialog_height = (area.height as f32 * self.dialog.height_percent) as u16;
        let dialog_x = (area.width.saturating_sub(dialog_width)) / 2;
        let dialog_y = (area.height.saturating_sub(dialog_height)) / 2;

        let dialog_area = Rect {
            x: area.x + dialog_x,
            y: area.y + dialog_y,
            width: dialog_width,
            height: dialog_height,
        };

        if let Some((offset_x, offset_y, shadow_style)) = shadow_spec(self.dialog.shadow) {
            let shadow_area = Rect {
                x: dialog_area.x.saturating_add(offset_x),
                y: dialog_area.y.saturating_add(offset_y),
                width: dialog_area.width,
                height: dialog_area.height,
            };
            if shadow_area.width > 0 && shadow_area.height > 0 {
                Paragraph::new("")
                    .style(shadow_style)
                    .render(shadow_area, buf);
            }
        }

        Clear.render(dialog_area, buf);

        let block = if self.dialog.title_inside {
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(self.dialog.get_border_color()))
                .style(self.dialog.style)
        } else {
            Block::default()
                .title(self.dialog.title)
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(self.dialog.get_border_color()))
                .style(self.dialog.style)
        };

        let inner = block.inner(dialog_area);
        block.render(dialog_area, buf);

        let content_inner = inset_rect(
            inner,
            self.dialog.content_padding.horizontal,
            self.dialog.content_padding.vertical,
        );

        let has_actions = !self.dialog.buttons.is_empty();
        let actions_height = if has_actions {
            match self.dialog.actions_layout {
                crate::primitives::dialog::types::DialogActionsLayout::Horizontal => 1,
                crate::primitives::dialog::types::DialogActionsLayout::Vertical => {
                    (self.dialog.buttons.len() as u16).min(content_inner.height)
                }
            }
        } else {
            0
        };

        let footer_height = match self.dialog.footer {
            crate::primitives::dialog::types::DialogFooter::Hidden => 0,
            crate::primitives::dialog::types::DialogFooter::Text(_) => 1,
        };

        let body_height = content_inner
            .height
            .saturating_sub(actions_height)
            .saturating_sub(footer_height);

        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(body_height),
                Constraint::Length(actions_height),
                Constraint::Length(footer_height),
            ])
            .split(content_inner);

        let body_area = chunks[0];

        if let Some(renderer) = self.dialog.body_renderer.as_mut() {
            renderer.render_body(body_area, buf);
        } else {
            let mut lines: Vec<Line> = Vec::new();

            if self.dialog.title_inside {
                lines.push(Line::from(vec![Span::styled(
                    self.dialog.title,
                    Style::default()
                        .add_modifier(Modifier::BOLD)
                        .fg(self.dialog.get_border_color()),
                )]));
                lines.push(Line::from(""));
            }

            for line in self.dialog.message.lines() {
                lines.push(Line::from(line));
            }

            let mut message = Paragraph::new(lines)
                .style(self.dialog.style)
                .alignment(self.dialog.message_alignment);

            match self.dialog.wrap {
                crate::primitives::dialog::types::DialogWrap::WordTrim => {
                    message = message.wrap(Wrap { trim: true });
                }
                crate::primitives::dialog::types::DialogWrap::WordNoTrim => {
                    message = message.wrap(Wrap { trim: false });
                }
                crate::primitives::dialog::types::DialogWrap::NoWrap => {}
            }

            message.render(body_area, buf);
        }

        self.dialog.button_areas.clear();

        if has_actions {
            let actions_area = chunks[1];
            match self.dialog.actions_layout {
                crate::primitives::dialog::types::DialogActionsLayout::Horizontal => {
                    render_horizontal_actions(self.dialog, actions_area, buf);
                }
                crate::primitives::dialog::types::DialogActionsLayout::Vertical => {
                    render_vertical_actions(self.dialog, actions_area, buf);
                }
            }
        }

        if let crate::primitives::dialog::types::DialogFooter::Text(footer_text) =
            self.dialog.footer
        {
            let footer_area = chunks[2];
            let footer = Paragraph::new(Line::from(vec![Span::styled(
                footer_text,
                self.dialog.footer_style,
            )]))
            .alignment(self.dialog.footer_alignment);
            footer.render(footer_area, buf);
        }
    }
}

fn shadow_spec(
    shadow: crate::primitives::dialog::types::DialogShadow,
) -> Option<(u16, u16, Style)> {
    use crate::primitives::dialog::types::DialogShadow;
    match shadow {
        DialogShadow::None => None,
        DialogShadow::Soft => Some((
            1,
            1,
            Style::default().bg(ratatui::style::Color::Rgb(16, 16, 16)),
        )),
        DialogShadow::Medium => Some((
            1,
            1,
            Style::default().bg(ratatui::style::Color::Rgb(10, 10, 10)),
        )),
        DialogShadow::Strong => Some((
            2,
            1,
            Style::default().bg(ratatui::style::Color::Rgb(0, 0, 0)),
        )),
        DialogShadow::Custom {
            offset_x,
            offset_y,
            style,
        } => Some((offset_x, offset_y, style)),
    }
}

fn inset_rect(area: Rect, horizontal: u16, vertical: u16) -> Rect {
    let inset_w = horizontal.saturating_mul(2);
    let inset_h = vertical.saturating_mul(2);
    Rect {
        x: area.x.saturating_add(horizontal),
        y: area.y.saturating_add(vertical),
        width: area.width.saturating_sub(inset_w),
        height: area.height.saturating_sub(inset_h),
    }
}

fn render_horizontal_actions(dialog: &mut Dialog<'_>, area: Rect, buf: &mut Buffer) {
    if area.height == 0 {
        return;
    }

    let total_button_width: usize = dialog
        .buttons
        .iter()
        .map(|b| UnicodeWidthStr::width(*b) + 2)
        .sum::<usize>()
        + dialog.buttons.len().saturating_sub(1) * 2;
    let row_y = area.y;

    let start_x = match dialog.actions_alignment {
        Alignment::Left => area.x,
        Alignment::Right => area
            .x
            .saturating_add(area.width.saturating_sub(total_button_width as u16)),
        Alignment::Center => area
            .x
            .saturating_add(area.width.saturating_sub(total_button_width as u16) / 2),
    };

    let mut x = start_x;
    for (idx, button_text) in dialog.buttons.iter().enumerate() {
        let button_width = (UnicodeWidthStr::width(*button_text) + 2) as u16;
        let style = if idx == dialog.selected_button {
            dialog.button_selected_style
        } else {
            dialog.button_style
        };

        let button_area = Rect {
            x,
            y: row_y,
            width: button_width,
            height: 1,
        };
        dialog.button_areas.push(button_area);

        let button_line = Line::from(vec![Span::styled(format!(" {} ", button_text), style)]);
        buf.set_line(x, row_y, &button_line, button_width);

        x = x.saturating_add(button_width).saturating_add(2);
    }
}

fn render_vertical_actions(dialog: &mut Dialog<'_>, area: Rect, buf: &mut Buffer) {
    if area.width == 0 || area.height == 0 {
        return;
    }

    for (row, (idx, button_text)) in dialog
        .buttons
        .iter()
        .enumerate()
        .take(area.height as usize)
        .enumerate()
    {
        let y = area.y + row as u16;
        let button_width = (UnicodeWidthStr::width(*button_text) + 2) as u16;
        let style = if idx == dialog.selected_button {
            dialog.button_selected_style
        } else {
            dialog.button_style
        };

        let x = match dialog.actions_alignment {
            Alignment::Left => area.x,
            Alignment::Right => area
                .x
                .saturating_add(area.width.saturating_sub(button_width)),
            Alignment::Center => area
                .x
                .saturating_add(area.width.saturating_sub(button_width) / 2),
        };

        let button_area = Rect {
            x,
            y,
            width: button_width,
            height: 1,
        };
        dialog.button_areas.push(button_area);

        let button_line = Line::from(vec![Span::styled(format!(" {} ", button_text), style)]);
        buf.set_line(x, y, &button_line, button_width);
    }
}
