use ratatui::style::Style;

use crate::primitives::button::Button;

impl Button {
    /// Sets the hover style for the button.
    ///
    /// # Arguments
    ///
    /// * `style` - The style to apply when the button is hovered
    ///
    /// # Returns
    ///
    /// `self` for method chaining
    ///
    /// # Example
    ///
    /// ```rust
    /// use ratatui::style::{Color, Style};
    /// use ratatui_toolkit::Button;
    ///
    /// let button = Button::new("Click Me")
    ///     .hover_style(Style::default().fg(Color::Yellow));
    /// ```
    pub fn hover_style(mut self, style: Style) -> Self {
        self.hover_style = style;
        self
    }
}
