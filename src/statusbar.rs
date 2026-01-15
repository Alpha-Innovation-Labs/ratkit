use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::Widget,
};

/// A status bar widget typically displayed at the bottom of the screen
pub struct StatusBar<'a> {
    /// Left-aligned items
    left_items: Vec<StatusItem<'a>>,
    /// Center-aligned items
    center_items: Vec<StatusItem<'a>>,
    /// Right-aligned items
    right_items: Vec<StatusItem<'a>>,
    /// Background style for the status bar
    style: Style,
}

/// An item in the status bar
#[derive(Clone)]
pub struct StatusItem<'a> {
    /// The text content
    text: String,
    /// The style for this item
    style: Style,
    /// Optional separator after this item
    separator: Option<&'a str>,
}

impl<'a> StatusItem<'a> {
    /// Create a new status item
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            style: Style::default().fg(Color::White),
            separator: Some(" │ "),
        }
    }

    /// Set the style for this item
    pub fn style(mut self, style: Style) -> Self {
        self.style = style;
        self
    }

    /// Set the separator (default is " │ ")
    pub fn separator(mut self, separator: Option<&'a str>) -> Self {
        self.separator = separator;
        self
    }

    /// Create a styled item with a specific color
    pub fn colored(text: impl Into<String>, color: Color) -> Self {
        Self::new(text).style(Style::default().fg(color))
    }

    /// Create a bold item
    pub fn bold(text: impl Into<String>) -> Self {
        Self::new(text).style(Style::default().add_modifier(Modifier::BOLD))
    }

    /// Create a dimmed item
    pub fn dimmed(text: impl Into<String>) -> Self {
        Self::new(text).style(Style::default().fg(Color::DarkGray))
    }
}

impl<'a> StatusBar<'a> {
    /// Create a new status bar
    pub fn new() -> Self {
        Self {
            left_items: Vec::new(),
            center_items: Vec::new(),
            right_items: Vec::new(),
            style: Style::default().bg(Color::DarkGray).fg(Color::White),
        }
    }

    /// Set the background style
    pub fn style(mut self, style: Style) -> Self {
        self.style = style;
        self
    }

    /// Add items to the left section
    pub fn left(mut self, items: Vec<StatusItem<'a>>) -> Self {
        self.left_items = items;
        self
    }

    /// Add items to the center section
    pub fn center(mut self, items: Vec<StatusItem<'a>>) -> Self {
        self.center_items = items;
        self
    }

    /// Add items to the right section
    pub fn right(mut self, items: Vec<StatusItem<'a>>) -> Self {
        self.right_items = items;
        self
    }

    /// Add a single item to the left
    pub fn add_left(mut self, item: StatusItem<'a>) -> Self {
        self.left_items.push(item);
        self
    }

    /// Add a single item to the center
    pub fn add_center(mut self, item: StatusItem<'a>) -> Self {
        self.center_items.push(item);
        self
    }

    /// Add a single item to the right
    pub fn add_right(mut self, item: StatusItem<'a>) -> Self {
        self.right_items.push(item);
        self
    }

    /// Build spans from items
    fn build_spans(&self, items: &[StatusItem<'a>]) -> Vec<Span<'_>> {
        let mut spans = Vec::new();
        for (i, item) in items.iter().enumerate() {
            spans.push(Span::styled(item.text.clone(), item.style));
            // Add separator if not the last item and separator is set
            if i < items.len() - 1 {
                if let Some(sep) = item.separator {
                    spans.push(Span::styled(sep, Style::default().fg(Color::DarkGray)));
                }
            }
        }
        spans
    }
}

impl Default for StatusBar<'_> {
    fn default() -> Self {
        Self::new()
    }
}

impl Widget for StatusBar<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        if area.height == 0 || area.width == 0 {
            return;
        }

        // Fill the background
        for x in area.x..area.x + area.width {
            buf[(x, area.y)].set_style(self.style);
        }

        // Build left section
        let left_spans = self.build_spans(&self.left_items);
        if !left_spans.is_empty() {
            let left_line = Line::from(left_spans);
            buf.set_line(area.x + 1, area.y, &left_line, area.width);
        }

        // Build center section
        let center_spans = self.build_spans(&self.center_items);
        if !center_spans.is_empty() {
            let center_line = Line::from(center_spans);
            let center_width = center_line.width() as u16;
            let center_x = area.x + (area.width.saturating_sub(center_width)) / 2;
            buf.set_line(center_x, area.y, &center_line, area.width);
        }

        // Build right section
        let right_spans = self.build_spans(&self.right_items);
        if !right_spans.is_empty() {
            let right_line = Line::from(right_spans);
            let right_width = right_line.width() as u16;
            let right_x = area.x + area.width.saturating_sub(right_width + 1);
            buf.set_line(right_x, area.y, &right_line, area.width);
        }
    }
}

/// Builder methods for common status bar patterns
impl<'a> StatusBar<'a> {
    /// Create a status bar with a simple message
    pub fn with_message(message: impl Into<String>) -> Self {
        Self::new().add_left(StatusItem::new(message))
    }

    /// Create a status bar showing mode and position (vim-like)
    pub fn vim_style(mode: impl Into<String>, line: usize, col: usize, total: usize) -> Self {
        Self::new()
            .add_left(StatusItem::bold(mode).separator(None))
            .add_right(StatusItem::dimmed(format!("{}:{}", line, col)))
            .add_right(
                StatusItem::dimmed(format!("{:.0}%", (line as f64 / total as f64) * 100.0))
                    .separator(None),
            )
    }

    /// Create a status bar with file information
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
