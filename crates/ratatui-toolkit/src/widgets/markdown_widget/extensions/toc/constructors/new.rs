//! Constructor for Toc widget.

use crate::widgets::markdown_widget::extensions::toc::enums::{TocConfig, TocStyle};
use crate::widgets::markdown_widget::extensions::toc::Toc;
use crate::widgets::markdown_widget::state::toc_state::TocState;

impl<'a> Toc<'a> {
    /// Create a new TOC widget from TocState.
    ///
    /// The Toc widget is a UI-only component that receives state via reference.
    /// State mutations happen through TocState methods, not the Toc widget.
    ///
    /// # Arguments
    ///
    /// * `toc_state` - Reference to the TocState containing entries, scroll, and hover state.
    ///
    /// # Returns
    ///
    /// A new `Toc` instance ready for rendering.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use ratatui_toolkit::markdown_widget::extensions::toc::Toc;
    /// use ratatui_toolkit::markdown_widget::state::toc_state::TocState;
    ///
    /// let toc_state = TocState::new();
    /// let toc = Toc::new(&toc_state);
    /// ```
    pub fn new(toc_state: &'a TocState) -> Self {
        Self {
            toc_state,
            config: TocConfig::default(),
            expanded: false,
        }
    }

    /// Set whether the TOC is expanded.
    ///
    /// # Arguments
    ///
    /// * `expanded` - True for expanded mode (full text), false for compact mode (lines).
    ///
    /// # Returns
    ///
    /// Self for method chaining.
    pub fn expanded(mut self, expanded: bool) -> Self {
        self.expanded = expanded;
        self
    }

    /// Set the TOC visual style mode.
    ///
    /// # Arguments
    ///
    /// * `style` - The visual style mode (Normal or Clerk).
    ///
    /// # Returns
    ///
    /// Self for method chaining.
    pub fn style(mut self, style: TocStyle) -> Self {
        self.config.style = style;
        self
    }

    /// Set the TOC configuration.
    ///
    /// # Arguments
    ///
    /// * `config` - The TOC configuration.
    ///
    /// # Returns
    ///
    /// Self for method chaining.
    pub fn config(mut self, config: TocConfig) -> Self {
        self.config = config;
        self
    }
}
