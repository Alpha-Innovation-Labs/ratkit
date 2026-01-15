//! Dialog component
//!
//! Provides modal dialog widgets with customizable buttons and styles.

use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Clear, Paragraph, Widget, Wrap},
};

/// Dialog type
///
/// Represents different types of dialogs with associated visual styles.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DialogType {
    /// Informational dialog (cyan border)
    Info,
    /// Success dialog (green border)
    Success,
    /// Warning dialog (yellow border)
    Warning,
    /// Error dialog (red border)
    Error,
    /// Confirmation dialog (blue border)
    Confirm,
}

/// A dialog/modal widget that overlays content
///
/// Dialogs are centered modals that display a message and buttons.
/// They support different visual styles (info, success, warning, error, confirm)
/// and can handle mouse clicks on buttons.
pub struct Dialog<'a> {
    /// Dialog title
    title: &'a str,
    /// Dialog message
    message: &'a str,
    /// Dialog type
    dialog_type: DialogType,
    /// Buttons to show
    buttons: Vec<&'a str>,
    /// Selected button index
    selected_button: usize,
    /// Width percentage (0.0 to 1.0)
    width_percent: f32,
    /// Height percentage (0.0 to 1.0)
    height_percent: f32,
    /// Style for the dialog
    style: Style,
    /// Style for selected button
    button_selected_style: Style,
    /// Style for unselected button
    button_style: Style,
    /// Areas for buttons (for click detection)
    button_areas: Vec<Rect>,
}

impl<'a> Dialog<'a> {
    /// Create a new dialog with default styling
    pub fn new(title: &'a str, message: &'a str) -> Self {
        Self {
            title,
            message,
            dialog_type: DialogType::Info,
            buttons: vec!["OK"],
            selected_button: 0,
            width_percent: 0.6,
            height_percent: 0.4,
            style: Style::default().fg(Color::White).bg(Color::Black),
            button_selected_style: Style::default()
                .fg(Color::Black)
                .bg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
            button_style: Style::default().fg(Color::White).bg(Color::DarkGray),
            button_areas: Vec::new(),
        }
    }

    /// Set dialog type
    pub fn dialog_type(mut self, dialog_type: DialogType) -> Self {
        self.dialog_type = dialog_type;
        self
    }

    /// Set buttons
    pub fn buttons(mut self, buttons: Vec<&'a str>) -> Self {
        self.buttons = buttons;
        self
    }

    /// Set width percentage
    pub fn width_percent(mut self, percent: f32) -> Self {
        self.width_percent = percent.clamp(0.1, 1.0);
        self
    }

    /// Set height percentage
    pub fn height_percent(mut self, percent: f32) -> Self {
        self.height_percent = percent.clamp(0.1, 1.0);
        self
    }

    /// Select next button
    pub fn select_next_button(&mut self) {
        if !self.buttons.is_empty() && self.selected_button < self.buttons.len() - 1 {
            self.selected_button += 1;
        }
    }

    /// Select previous button
    pub fn select_previous_button(&mut self) {
        if self.selected_button > 0 {
            self.selected_button -= 1;
        }
    }

    /// Get selected button index
    pub fn get_selected_button(&self) -> usize {
        self.selected_button
    }

    /// Get selected button text
    pub fn get_selected_button_text(&self) -> Option<&str> {
        self.buttons.get(self.selected_button).copied()
    }

    /// Handle click on buttons
    pub fn handle_click(&self, column: u16, row: u16) -> Option<usize> {
        for (idx, area) in self.button_areas.iter().enumerate() {
            if column >= area.x
                && column < area.x + area.width
                && row >= area.y
                && row < area.y + area.height
            {
                return Some(idx);
            }
        }
        None
    }

    /// Create a confirmation dialog
    pub fn confirm(title: &'a str, message: &'a str) -> Self {
        Self::new(title, message)
            .dialog_type(DialogType::Confirm)
            .buttons(vec!["Yes", "No"])
    }

    /// Create an info dialog
    pub fn info(title: &'a str, message: &'a str) -> Self {
        Self::new(title, message).dialog_type(DialogType::Info)
    }

    /// Create a success dialog
    pub fn success(title: &'a str, message: &'a str) -> Self {
        Self::new(title, message).dialog_type(DialogType::Success)
    }

    /// Create a warning dialog
    pub fn warning(title: &'a str, message: &'a str) -> Self {
        Self::new(title, message).dialog_type(DialogType::Warning)
    }

    /// Create an error dialog
    pub fn error(title: &'a str, message: &'a str) -> Self {
        Self::new(title, message).dialog_type(DialogType::Error)
    }

    /// Get border color based on dialog type
    fn get_border_color(&self) -> Color {
        match self.dialog_type {
            DialogType::Info => Color::Cyan,
            DialogType::Success => Color::Green,
            DialogType::Warning => Color::Yellow,
            DialogType::Error => Color::Red,
            DialogType::Confirm => Color::Blue,
        }
    }
}

impl Widget for Dialog<'_> {
    fn render(mut self, area: Rect, buf: &mut Buffer) {
        let dialog_width = (area.width as f32 * self.width_percent) as u16;
        let dialog_height = (area.height as f32 * self.height_percent) as u16;
        let dialog_x = (area.width.saturating_sub(dialog_width)) / 2;
        let dialog_y = (area.height.saturating_sub(dialog_height)) / 2;

        let dialog_area = Rect {
            x: area.x + dialog_x,
            y: area.y + dialog_y,
            width: dialog_width,
            height: dialog_height,
        };

        Clear.render(dialog_area, buf);

        let block = Block::default()
            .title(self.title)
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Style::default().fg(self.get_border_color()))
            .style(self.style);

        let inner = block.inner(dialog_area);
        block.render(dialog_area, buf);

        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Min(3), Constraint::Length(3)])
            .split(inner);

        let message = Paragraph::new(self.message)
            .style(self.style)
            .alignment(Alignment::Center)
            .wrap(Wrap { trim: true });
        message.render(chunks[0], buf);

        self.button_areas.clear();

        if !self.buttons.is_empty() {
            let total_button_width: usize = self.buttons.iter().map(|b| b.len() + 4).sum();
            let button_area_width = chunks[1].width as usize;
            let start_x = if total_button_width < button_area_width {
                chunks[1].x + ((button_area_width - total_button_width) / 2) as u16
            } else {
                chunks[1].x
            };

            let mut x = start_x;
            let y = chunks[1].y + 1;

            for (idx, button_text) in self.buttons.iter().enumerate() {
                let button_width = button_text.len() as u16 + 2;
                let style = if idx == self.selected_button {
                    self.button_selected_style
                } else {
                    self.button_style
                };

                let button_area = Rect {
                    x,
                    y,
                    width: button_width,
                    height: 1,
                };

                self.button_areas.push(button_area);

                for bx in x..x + button_width {
                    if let Some(cell) = buf.cell_mut((bx, y)) {
                        cell.set_style(style);
                    }
                }

                let button_line =
                    Line::from(vec![Span::styled(format!(" {} ", button_text), style)]);

                buf.set_line(x, y, &button_line, button_width);
                x += button_width + 2;
            }
        }
    }
}

/// Mutable widget variant
pub struct DialogWidget<'a> {
    dialog: &'a mut Dialog<'a>,
}

impl<'a> DialogWidget<'a> {
    pub fn new(dialog: &'a mut Dialog<'a>) -> Self {
        Self { dialog }
    }
}

impl Widget for DialogWidget<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
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

        Clear.render(dialog_area, buf);

        let block = Block::default()
            .title(self.dialog.title)
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Style::default().fg(self.dialog.get_border_color()))
            .style(self.dialog.style);

        let inner = block.inner(dialog_area);
        block.render(dialog_area, buf);

        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Min(3), Constraint::Length(3)])
            .split(inner);

        let message = Paragraph::new(self.dialog.message)
            .style(self.dialog.style)
            .alignment(Alignment::Center)
            .wrap(Wrap { trim: true });
        message.render(chunks[0], buf);

        self.dialog.button_areas.clear();

        if !self.dialog.buttons.is_empty() {
            let total_button_width: usize = self.dialog.buttons.iter().map(|b| b.len() + 4).sum();
            let button_area_width = chunks[1].width as usize;
            let start_x = if total_button_width < button_area_width {
                chunks[1].x + ((button_area_width - total_button_width) / 2) as u16
            } else {
                chunks[1].x
            };

            let mut x = start_x;
            let y = chunks[1].y + 1;

            for (idx, button_text) in self.dialog.buttons.iter().enumerate() {
                let button_width = button_text.len() as u16 + 2;
                let style = if idx == self.dialog.selected_button {
                    self.dialog.button_selected_style
                } else {
                    self.dialog.button_style
                };

                let button_area = Rect {
                    x,
                    y,
                    width: button_width,
                    height: 1,
                };

                self.dialog.button_areas.push(button_area);

                for bx in x..x + button_width {
                    if let Some(cell) = buf.cell_mut((bx, y)) {
                        cell.set_style(style);
                    }
                }

                let button_line =
                    Line::from(vec![Span::styled(format!(" {} ", button_text), style)]);

                buf.set_line(x, y, &button_line, button_width);
                x += button_width + 2;
            }
        }
    }
}
