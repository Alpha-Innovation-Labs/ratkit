use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, BorderType, Borders, Clear, Paragraph, Wrap};
use ratatui::Frame;

/// A single hotkey binding with its description
#[derive(Debug, Clone)]
pub struct Hotkey {
    pub key: String,
    pub description: String,
}

impl Hotkey {
    pub fn new(key: impl Into<String>, description: impl Into<String>) -> Self {
        Self {
            key: key.into(),
            description: description.into(),
        }
    }
}

/// A category/section of hotkeys
#[derive(Debug, Clone)]
pub struct HotkeySection {
    pub title: String,
    pub hotkeys: Vec<Hotkey>,
}

impl HotkeySection {
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            hotkeys: Vec::new(),
        }
    }

    pub fn with_hotkeys(mut self, hotkeys: Vec<Hotkey>) -> Self {
        self.hotkeys = hotkeys;
        self
    }

    pub fn add_hotkey(mut self, key: impl Into<String>, description: impl Into<String>) -> Self {
        self.hotkeys.push(Hotkey::new(key, description));
        self
    }
}

/// Configuration for the hotkey modal appearance
#[derive(Debug, Clone)]
pub struct HotkeyModalConfig {
    /// Title of the modal
    pub title: String,
    /// Border color
    pub border_color: Color,
    /// Width as a percentage of screen width (0.0 - 1.0)
    pub width_percent: f32,
    /// Height as a percentage of screen height (0.0 - 1.0)
    pub height_percent: f32,
    /// Footer text (e.g., "Press any key to close")
    pub footer: Option<String>,
    /// Whether to show title inside content instead of in border
    pub title_inside: bool,
}

impl Default for HotkeyModalConfig {
    fn default() -> Self {
        Self {
            title: "Help".to_string(),
            border_color: Color::Cyan,
            width_percent: 0.6,
            height_percent: 0.6,
            footer: Some("Press any key to close".to_string()),
            title_inside: false,
        }
    }
}

impl HotkeyModalConfig {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_title(mut self, title: impl Into<String>) -> Self {
        self.title = title.into();
        self
    }

    pub fn with_border_color(mut self, color: Color) -> Self {
        self.border_color = color;
        self
    }

    pub fn with_size(mut self, width_percent: f32, height_percent: f32) -> Self {
        self.width_percent = width_percent.clamp(0.1, 1.0);
        self.height_percent = height_percent.clamp(0.1, 1.0);
        self
    }

    pub fn with_footer(mut self, footer: Option<String>) -> Self {
        self.footer = footer;
        self
    }

    pub fn with_title_inside(mut self, inside: bool) -> Self {
        self.title_inside = inside;
        self
    }
}

/// Render a hotkey modal with darkened background
pub fn render_hotkey_modal(
    frame: &mut Frame,
    sections: &[HotkeySection],
    config: &HotkeyModalConfig,
) {
    let mut lines = Vec::new();

    // Add title inside content if requested
    if config.title_inside {
        lines.push(Line::from(vec![Span::styled(
            &config.title,
            Style::default()
                .add_modifier(Modifier::BOLD)
                .fg(config.border_color),
        )]));
        lines.push(Line::from(""));
    }

    // Build the content from sections
    for (i, section) in sections.iter().enumerate() {
        // Add section title
        lines.push(Line::from(vec![Span::styled(
            &section.title,
            Style::default().add_modifier(Modifier::BOLD),
        )]));
        lines.push(Line::from(""));

        // Add hotkeys in this section
        for hotkey in &section.hotkeys {
            // Format: "  Key         Description"
            let line = format!("  {:<12}{}", hotkey.key, hotkey.description);
            lines.push(Line::from(line));
        }

        // Add spacing between sections (but not after the last one)
        if i < sections.len() - 1 {
            lines.push(Line::from(""));
        }
    }

    // Add footer if provided
    if let Some(ref footer) = config.footer {
        lines.push(Line::from(""));
        lines.push(Line::from(vec![Span::styled(
            footer,
            Style::default().fg(Color::DarkGray),
        )]));
    }

    let area = frame.area();

    // Render a darkened overlay over the entire screen (no block, just background)
    let overlay_paragraph = Paragraph::new("").style(
        Style::default()
            .bg(Color::Rgb(0, 0, 0)) // Pure black for maximum dimming
            .fg(Color::Rgb(40, 40, 40)),
    );
    frame.render_widget(overlay_paragraph, area);

    // Calculate popup size
    let popup_width = (area.width as f32 * config.width_percent) as u16;
    let popup_height = (area.height as f32 * config.height_percent) as u16;
    let popup_x = (area.width - popup_width) / 2;
    let popup_y = (area.height - popup_height) / 2;

    let popup_area = ratatui::layout::Rect {
        x: popup_x,
        y: popup_y,
        width: popup_width,
        height: popup_height,
    };

    // Clear the popup area to make it opaque
    frame.render_widget(Clear, popup_area);

    // Render the modal
    let block = if config.title_inside {
        // No title in border if showing inside
        Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Style::default().fg(config.border_color))
    } else {
        // Show title in border
        Block::default()
            .title(format!(" {} ", config.title))
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Style::default().fg(config.border_color))
    };

    let paragraph = Paragraph::new(lines)
        .style(Style::default())
        .wrap(Wrap { trim: false });

    // Render the block
    frame.render_widget(block, popup_area);

    // Render content with padding inside the block
    let inner_area = popup_area.inner(ratatui::layout::Margin {
        horizontal: 2,
        vertical: 2,
    });
    frame.render_widget(paragraph, inner_area);
}
