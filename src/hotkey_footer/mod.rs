use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Paragraph, Widget};
use ratatui::Frame;

/// A single hotkey with its description
#[derive(Clone, Debug)]
pub struct HotkeyItem {
    /// The key or key combination (e.g., "j/k", "Enter", "?")
    pub key: String,
    /// The description of what the key does (e.g., "scroll", "navigate", "help")
    pub description: String,
}

impl HotkeyItem {
    /// Create a new hotkey item
    pub fn new(key: impl Into<String>, description: impl Into<String>) -> Self {
        Self {
            key: key.into(),
            description: description.into(),
        }
    }
}

/// A styled hotkey footer bar component (aerospace-tui style)
/// Renders a single line with alternating hotkey/description pairs
#[derive(Clone, Debug)]
pub struct HotkeyFooter {
    /// List of hotkey items to display
    pub items: Vec<HotkeyItem>,
    /// Color for the hotkeys
    pub key_color: Color,
    /// Color for the descriptions
    pub description_color: Color,
    /// Background color for the footer
    pub background_color: Color,
}

impl HotkeyFooter {
    /// Create a new hotkey footer with default styling (aerospace-tui style)
    pub fn new(items: Vec<HotkeyItem>) -> Self {
        Self {
            items,
            key_color: Color::Cyan,
            description_color: Color::DarkGray,
            background_color: Color::Black,
        }
    }

    /// Set the color for hotkeys
    pub fn key_color(mut self, color: Color) -> Self {
        self.key_color = color;
        self
    }

    /// Set the color for descriptions
    pub fn description_color(mut self, color: Color) -> Self {
        self.description_color = color;
        self
    }

    /// Set the background color
    pub fn background_color(mut self, color: Color) -> Self {
        self.background_color = color;
        self
    }

    /// Build the hotkey line with alternating styled spans
    fn build_line(&self) -> Line<'static> {
        let mut spans = Vec::new();

        for (i, item) in self.items.iter().enumerate() {
            // Add leading space for first item
            if i == 0 {
                spans.push(Span::raw(" "));
            }

            // Add hotkey (bold + colored)
            spans.push(Span::styled(
                item.key.clone(),
                Style::default()
                    .fg(self.key_color)
                    .add_modifier(Modifier::BOLD),
            ));

            // Add description (dark gray)
            spans.push(Span::styled(
                format!(" {}  ", item.description),
                Style::default().fg(self.description_color),
            ));
        }

        Line::from(spans)
    }

    /// Render the hotkey footer in the given area
    pub fn render(&self, frame: &mut Frame, area: Rect) {
        let line = self.build_line();
        let widget = Paragraph::new(line).style(Style::default().bg(self.background_color));
        frame.render_widget(widget, area);
    }
}

impl Widget for HotkeyFooter {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let line = self.build_line();
        let widget = Paragraph::new(line).style(Style::default().bg(self.background_color));
        widget.render(area, buf);
    }
}

impl Widget for &HotkeyFooter {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let line = self.build_line();
        let widget = Paragraph::new(line).style(Style::default().bg(self.background_color));
        widget.render(area, buf);
    }
}

/// Builder pattern for creating hotkey footers
pub struct HotkeyFooterBuilder {
    items: Vec<HotkeyItem>,
}

impl HotkeyFooterBuilder {
    /// Create a new builder
    pub fn new() -> Self {
        Self { items: Vec::new() }
    }

    /// Add a hotkey item
    pub fn add(mut self, key: impl Into<String>, description: impl Into<String>) -> Self {
        self.items.push(HotkeyItem::new(key, description));
        self
    }

    /// Add multiple hotkey items from a vec of (key, description) tuples
    pub fn add_items(mut self, items: Vec<(String, &str)>) -> Self {
        for (key, desc) in items {
            self.items.push(HotkeyItem::new(key, desc));
        }
        self
    }

    /// Build the hotkey footer
    pub fn build(self) -> HotkeyFooter {
        HotkeyFooter::new(self.items)
    }
}

impl Default for HotkeyFooterBuilder {
    fn default() -> Self {
        Self::new()
    }
}
