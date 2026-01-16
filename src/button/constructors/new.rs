use ratatui::style::{Color, Modifier, Style};

use crate::button::Button;

impl Button {
    /// Creates a new button with the specified text.
    ///
    /// # Arguments
    ///
    /// * `text` - The button text to display
    ///
    /// # Returns
    ///
    /// A new `Button` instance with default styling
    ///
    /// # Example
    ///
    /// ```rust
    /// use ratatui_toolkit::Button;
    ///
    /// let button = Button::new("Click Me");
    /// ```
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            area: None,
            hovered: false,
            normal_style: Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
            hover_style: Style::default()
                .fg(Color::Black)
                .bg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        }
    }
}
