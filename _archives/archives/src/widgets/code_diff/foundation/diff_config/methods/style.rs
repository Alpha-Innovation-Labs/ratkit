use crate::widgets::code_diff::diff_config::DiffConfig;
use crate::widgets::code_diff::enums::DiffStyle;

impl DiffConfig {
    /// Sets the diff display style.
    ///
    /// # Arguments
    ///
    /// * `style` - The display style to use
    ///
    /// # Returns
    ///
    /// Self for method chaining
    ///
    /// # Example
    ///
    /// ```rust
    /// use ratatui_toolkit::code_diff::{DiffConfig, DiffStyle};
    ///
    /// let config = DiffConfig::new().style(DiffStyle::Unified);
    /// ```
    pub fn style(mut self, style: DiffStyle) -> Self {
        self.style = style;
        self
    }
}
