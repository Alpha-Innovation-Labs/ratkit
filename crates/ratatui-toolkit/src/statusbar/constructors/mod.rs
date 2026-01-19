use ratatui::style::{Color, Modifier, Style};

use crate::statusbar::{StatusBar, StatusItem};

impl<'a> StatusItem<'a> {
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            style: Style::default().fg(Color::White),
            separator: Some(" │ "),
        }
    }

    pub fn style(mut self, style: Style) -> Self {
        self.style = style;
        self
    }

    pub fn separator(mut self, separator: Option<&'a str>) -> Self {
        self.separator = separator;
        self
    }

    pub fn colored(text: impl Into<String>, color: Color) -> Self {
        Self::new(text).style(Style::default().fg(color))
    }

    pub fn bold(text: impl Into<String>) -> Self {
        Self::new(text).style(Style::default().add_modifier(Modifier::BOLD))
    }

    pub fn dimmed(text: impl Into<String>) -> Self {
        Self::new(text).style(Style::default().fg(Color::DarkGray))
    }
}

impl<'a> StatusBar<'a> {
    pub fn new() -> Self {
        Self {
            left_items: Vec::new(),
            center_items: Vec::new(),
            right_items: Vec::new(),
            style: Style::default().bg(Color::DarkGray).fg(Color::White),
        }
    }

    pub fn style(mut self, style: Style) -> Self {
        self.style = style;
        self
    }

    pub fn left(mut self, items: Vec<StatusItem<'a>>) -> Self {
        self.left_items = items;
        self
    }

    pub fn center(mut self, items: Vec<StatusItem<'a>>) -> Self {
        self.center_items = items;
        self
    }

    pub fn right(mut self, items: Vec<StatusItem<'a>>) -> Self {
        self.right_items = items;
        self
    }

    pub fn add_left(mut self, item: StatusItem<'a>) -> Self {
        self.left_items.push(item);
        self
    }

    pub fn add_center(mut self, item: StatusItem<'a>) -> Self {
        self.center_items.push(item);
        self
    }

    pub fn add_right(mut self, item: StatusItem<'a>) -> Self {
        self.right_items.push(item);
        self
    }

    pub fn with_message(message: impl Into<String>) -> Self {
        Self::new().add_left(StatusItem::new(message))
    }

    pub fn vim_style(mode: impl Into<String>, line: usize, col: usize, total: usize) -> Self {
        Self::new()
            .add_left(StatusItem::bold(mode).separator(None))
            .add_right(StatusItem::dimmed(format!("{}:{}", line, col)))
            .add_right(
                StatusItem::dimmed(format!("{:.0}%", (line as f64 / total as f64) * 100.0))
                    .separator(None),
            )
    }

    pub fn file_info(filename: impl Into<String>, modified: bool, readonly: bool) -> Self {
        let mut bar = Self::new().add_left(StatusItem::new(filename));

        if modified {
            bar = bar.add_left(StatusItem::colored("[+]", Color::Yellow));
        }

        if readonly {
            bar = bar.add_left(StatusItem::colored("[RO]", Color::Red));
        }

        bar
    }
}
