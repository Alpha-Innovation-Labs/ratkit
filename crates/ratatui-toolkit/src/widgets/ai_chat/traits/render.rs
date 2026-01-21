use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, List, ListItem, Paragraph},
    Frame,
};

use crate::widgets::ai_chat::state::MessageRole;
use crate::widgets::ai_chat::AIChat;

impl<'a> AIChat<'a> {
    pub fn render(&self, frame: &mut Frame, area: Rect) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Min(0), Constraint::Length(3)])
            .split(area);

        let messages_area = chunks[0];
        let input_area = chunks[1];

        self.render_messages(frame, messages_area);
        self.render_input(frame, input_area);

        if self.input.is_file_mode() {
            self.render_file_popup(frame, input_area);
        } else if self.input.is_command_mode() {
            self.render_command_popup(frame, input_area);
        }
    }

    fn render_messages(&self, frame: &mut Frame, area: Rect) {
        let block = Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .title(" Chat ");

        let inner = block.inner(area);
        frame.render_widget(block, area);

        let mut items = Vec::new();

        for msg in self.messages.messages() {
            let prefix = match msg.role {
                MessageRole::User => "You: ",
                MessageRole::Assistant => "AI:  ",
            };

            let style = match msg.role {
                MessageRole::User => self.user_message_style,
                MessageRole::Assistant => self.ai_message_style,
            };

            let mut content = vec![Span::styled(prefix, style)];

            if !msg.attachments.is_empty() {
                let files_str = msg
                    .attachments
                    .iter()
                    .map(|f| format!("@{}", f))
                    .collect::<Vec<_>>()
                    .join(", ");
                content.push(Span::styled(
                    format!("[{}] ", files_str),
                    Style::default().fg(Color::Yellow),
                ));
            }

            content.push(Span::raw(&msg.content));

            let line = Line::from(content);
            items.push(ListItem::new(line));
        }

        if self.is_loading {
            items.push(ListItem::new(Line::from(vec![
                Span::styled("AI:  ", self.ai_message_style),
                Span::styled("⠋ Thinking...", Style::default().fg(Color::Gray)),
            ])));
        }

        let list = List::new(items)
            .block(Block::default())
            .style(Style::default());

        frame.render_widget(list, inner);
    }

    fn render_input(&self, frame: &mut Frame, area: Rect) {
        let mut input_text = self.input.text().to_string();

        if self.input.is_file_mode() {
            let filtered = self.input.filtered_files();
            if let Some(file) = filtered.get(self.input.selected_file_index()) {
                input_text = format!("@{}{}", self.input.file_query(), file);
            } else {
                input_text = format!("@{}", self.input.file_query());
            }
        } else if self.input.is_command_mode() {
            let filtered = self.filtered_commands();
            if let Some(cmd) = filtered.get(self.selected_command_index()) {
                input_text = cmd.clone();
            } else {
                input_text = format!("/{}", self.input.command());
            }
        }

        let prompt = &self.input_prompt;
        let cursor_pos = prompt.len() + self.input.cursor();

        let paragraph = Paragraph::new(format!("{}{}", prompt, input_text))
            .style(self.input_style)
            .block(Block::default());

        frame.render_widget(paragraph, area);

        if cursor_pos < input_text.len() + prompt.len() {
            let cursor_x = area.x + cursor_pos as u16;
            let cursor_y = area.y;
            frame.set_cursor_position((cursor_x, cursor_y));
        }
    }

    fn render_file_popup(&self, frame: &mut Frame, input_area: Rect) {
        let filtered = self.input.filtered_files();

        if filtered.is_empty() {
            return;
        }

        let max_height = 10.min(filtered.len() as u16);
        let popup_height = max_height + 2;

        let popup_y = if input_area.y.saturating_sub(popup_height) > 0 {
            input_area.y.saturating_sub(popup_height)
        } else {
            input_area.y.saturating_add(1)
        };

        let popup_width = 40.min(input_area.width);
        let popup_x = input_area.x;

        let popup_area = Rect {
            x: popup_x,
            y: popup_y,
            width: popup_width,
            height: popup_height,
        };

        let items: Vec<ListItem> = filtered
            .iter()
            .enumerate()
            .map(|(i, file)| {
                let style = if i == self.input.selected_file_index() {
                    Style::default()
                        .bg(Color::Blue)
                        .fg(Color::White)
                        .add_modifier(Modifier::BOLD)
                } else {
                    Style::default().fg(Color::White).bg(Color::Black)
                };
                ListItem::new(Span::styled(file.clone(), style))
            })
            .collect();

        let list = List::new(items).block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .style(Style::default().bg(Color::Black)),
        );

        frame.render_widget(list, popup_area);
    }

    fn render_command_popup(&self, frame: &mut Frame, input_area: Rect) {
        let filtered = self.filtered_commands();

        if filtered.is_empty() {
            return;
        }

        let max_height = 10.min(filtered.len() as u16);
        let popup_height = max_height + 2;

        let popup_y = if input_area.y.saturating_sub(popup_height) > 0 {
            input_area.y.saturating_sub(popup_height)
        } else {
            input_area.y.saturating_add(1)
        };

        let popup_width = 40.min(input_area.width);
        let popup_x = input_area.x;

        let popup_area = Rect {
            x: popup_x,
            y: popup_y,
            width: popup_width,
            height: popup_height,
        };

        let items: Vec<ListItem> = filtered
            .iter()
            .enumerate()
            .map(|(i, cmd)| {
                let style = if i == self.selected_command_index() {
                    Style::default()
                        .bg(Color::Blue)
                        .fg(Color::White)
                        .add_modifier(Modifier::BOLD)
                } else {
                    Style::default().fg(Color::White).bg(Color::Black)
                };
                ListItem::new(Span::styled(cmd.clone(), style))
            })
            .collect();

        let list = List::new(items).block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .style(Style::default().bg(Color::Black)),
        );

        frame.render_widget(list, popup_area);
    }
}
