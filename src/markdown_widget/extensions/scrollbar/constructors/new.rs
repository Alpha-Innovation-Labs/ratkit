//! Constructor methods for CustomScrollbar.

use crate::markdown_widget::extensions::scrollbar::{CustomScrollbar, ScrollbarConfig};
use crate::markdown_widget::state::scroll_state::ScrollState;

impl<'a> CustomScrollbar<'a> {
    /// Create a new CustomScrollbar with the given scroll state.
    ///
    /// # Arguments
    ///
    /// * `scroll_state` - Reference to the scroll state to track.
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// let scrollbar = CustomScrollbar::new(&scroll_state);
    /// ```
    pub fn new(scroll_state: &'a ScrollState) -> Self {
        Self {
            scroll_state,
            config: ScrollbarConfig::default(),
            show_percentage: false,
        }
    }

    /// Set the scrollbar configuration.
    ///
    /// # Arguments
    ///
    /// * `config` - The configuration to use.
    ///
    /// # Returns
    ///
    /// Self for method chaining.
    pub fn config(mut self, config: ScrollbarConfig) -> Self {
        self.config = config;
        self
    }

    /// Enable or disable the percentage indicator.
    ///
    /// # Arguments
    ///
    /// * `show` - Whether to show the percentage.
    ///
    /// # Returns
    ///
    /// Self for method chaining.
    pub fn show_percentage(mut self, show: bool) -> Self {
        self.show_percentage = show;
        self
    }
}
