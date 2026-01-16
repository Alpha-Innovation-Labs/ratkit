use ratatui::style::Style;

use crate::button::Button;

impl Button {
    /// Sets the normal (non-hovered) style for the button.
    ///
    /// # Arguments
    ///
    /// * `style` - The style to apply when the button is not hovered
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
    ///     .normal_style(Style::default().fg(Color::White));
    /// ```
    pub fn normal_style(mut self, style: Style) -> Self {
        self.normal_style = style;
        self
    }
}
