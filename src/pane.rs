use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, BorderType, Borders, Paragraph, Widget};
use ratatui::Frame;

/// A styled panel component with title, icon, padding, and optional footer
#[derive(Clone, Debug)]
pub struct Pane<'a> {
    /// Title text for the pane
    pub title: String,

    /// Icon to display before the title (optional)
    pub icon: Option<String>,

    /// Padding around the content (top, right, bottom, left)
    pub padding: (u16, u16, u16, u16),

    /// Simple text footer (optional) - displayed in border
    pub text_footer: Option<Line<'a>>,

    /// Height of the footer area when using widget footers
    pub footer_height: u16,

    // Styling
    pub border_style: Style,
    pub border_type: BorderType,
    pub title_style: Style,
    pub footer_style: Style,
}

impl<'a> Pane<'a> {
    /// Create a new pane with the given title
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            icon: None,
            padding: (0, 0, 0, 0),
            text_footer: None,
            footer_height: 0,
            border_style: Style::default().fg(Color::White),
            border_type: BorderType::Rounded,
            title_style: Style::default().add_modifier(Modifier::BOLD),
            footer_style: Style::default().fg(Color::DarkGray),
        }
    }

    /// Set the icon for the title
    pub fn with_icon(mut self, icon: impl Into<String>) -> Self {
        self.icon = Some(icon.into());
        self
    }

    /// Set the padding (top, right, bottom, left)
    pub fn with_padding(mut self, top: u16, right: u16, bottom: u16, left: u16) -> Self {
        self.padding = (top, right, bottom, left);
        self
    }

    /// Set uniform padding on all sides
    pub fn with_uniform_padding(mut self, padding: u16) -> Self {
        self.padding = (padding, padding, padding, padding);
        self
    }

    /// Set a simple text footer (displayed in the border)
    pub fn with_text_footer(mut self, footer: Line<'a>) -> Self {
        self.text_footer = Some(footer);
        self
    }

    /// Set the height for the footer area (when using widget footers)
    pub fn with_footer_height(mut self, height: u16) -> Self {
        self.footer_height = height;
        self
    }

    /// Set the border style
    pub fn border_style(mut self, style: Style) -> Self {
        self.border_style = style;
        self
    }

    /// Set the border type
    pub fn border_type(mut self, border_type: BorderType) -> Self {
        self.border_type = border_type;
        self
    }

    /// Set the title style
    pub fn title_style(mut self, style: Style) -> Self {
        self.title_style = style;
        self
    }

    /// Set the footer style
    pub fn footer_style(mut self, style: Style) -> Self {
        self.footer_style = style;
        self
    }

    /// Build the title line with optional icon
    fn build_title_line(&self) -> Line<'a> {
        let mut spans = vec![Span::raw(" ")];

        if let Some(ref icon) = self.icon {
            spans.push(Span::styled(icon.clone(), self.title_style));
            spans.push(Span::raw(" "));
        }

        spans.push(Span::styled(self.title.clone(), self.title_style));
        spans.push(Span::raw(" "));

        Line::from(spans)
    }

    /// Get the inner area after applying padding
    fn get_padded_area(&self, area: Rect) -> Rect {
        Rect {
            x: area.x + self.padding.3,
            y: area.y + self.padding.0,
            width: area.width.saturating_sub(self.padding.1 + self.padding.3),
            height: area.height.saturating_sub(self.padding.0 + self.padding.2),
        }
    }

    /// Build the block with title and optional text footer
    fn build_block(&self) -> Block<'a> {
        let mut block = Block::default()
            .borders(Borders::ALL)
            .border_type(self.border_type)
            .border_style(self.border_style)
            .title(self.build_title_line());

        // Add text footer if present
        if let Some(ref footer) = self.text_footer {
            block = block.title_bottom(footer.clone().style(self.footer_style));
        }

        block
    }

    /// Render the pane with the given content widget (no footer widget)
    pub fn render<W>(&self, frame: &mut Frame, area: Rect, content: W)
    where
        W: Widget,
    {
        let padded_area = self.get_padded_area(area);
        let block = self.build_block();
        let inner = block.inner(padded_area);

        frame.render_widget(block, padded_area);
        frame.render_widget(content, inner);
    }

    /// Render the pane with content and a footer widget
    /// The footer area will be `footer_height` lines tall at the bottom
    pub fn render_with_footer<C, F>(&self, frame: &mut Frame, area: Rect, content: C, footer: F)
    where
        C: Widget,
        F: Widget,
    {
        let padded_area = self.get_padded_area(area);
        let block = self.build_block();
        let inner = block.inner(padded_area);

        frame.render_widget(block, padded_area);

        // If footer height is 0, just render content
        if self.footer_height == 0 {
            frame.render_widget(content, inner);
            return;
        }

        // Split the inner area into content and footer
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Min(0),                     // Content area
                Constraint::Length(self.footer_height), // Footer area
            ])
            .split(inner);

        frame.render_widget(content, chunks[0]);
        frame.render_widget(footer, chunks[1]);
    }

    /// Render the pane with a paragraph content
    pub fn render_paragraph(&self, frame: &mut Frame, area: Rect, content: Vec<Line<'a>>) {
        let paragraph = Paragraph::new(content);
        self.render(frame, area, paragraph);
    }

    /// Render the pane with paragraph content and a footer widget
    pub fn render_paragraph_with_footer<F>(
        &self,
        frame: &mut Frame,
        area: Rect,
        content: Vec<Line<'a>>,
        footer: F,
    ) where
        F: Widget,
    {
        let paragraph = Paragraph::new(content);
        self.render_with_footer(frame, area, paragraph, footer);
    }

    /// Render just the pane block and return the inner area (useful for custom rendering)
    /// Returns (content_area, optional_footer_area)
    pub fn render_block(&self, frame: &mut Frame, area: Rect) -> (Rect, Option<Rect>) {
        let padded_area = self.get_padded_area(area);
        let block = self.build_block();
        let inner = block.inner(padded_area);

        frame.render_widget(block, padded_area);

        // If no footer height, return just the content area
        if self.footer_height == 0 {
            return (inner, None);
        }

        // Split into content and footer areas
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Min(0), Constraint::Length(self.footer_height)])
            .split(inner);

        (chunks[0], Some(chunks[1]))
    }
}
