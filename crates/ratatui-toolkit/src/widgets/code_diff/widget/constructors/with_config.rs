use crate::primitives::resizable_grid::ResizableGrid;
use crate::widgets::code_diff::code_diff::CodeDiff;
use crate::widgets::code_diff::diff_config::DiffConfig;

impl CodeDiff {
    /// Sets the configuration for this diff widget.
    ///
    /// This also updates the sidebar state based on the new config:
    /// - `show_sidebar` is set from `config.sidebar_enabled`
    /// - `sidebar_width_percent` is set from `config.sidebar_default_width`
    ///
    /// # Arguments
    ///
    /// * `config` - The display configuration to use
    ///
    /// # Returns
    ///
    /// Self for method chaining
    ///
    /// # Example
    ///
    /// ```rust
    /// use ratatui_toolkit::code_diff::{CodeDiff, DiffConfig};
    ///
    /// let config = DiffConfig::new()
    ///     .show_line_numbers(false)
    ///     .sidebar_enabled(true);
    /// let diff = CodeDiff::new().with_config(config);
    /// ```
    pub fn with_config(mut self, config: DiffConfig) -> Self {
        self.show_sidebar = config.sidebar_enabled;
        let mut sidebar_split = ResizableGrid::new(0);
        sidebar_split.split_pane_vertically(0);
        let split_index = 0;
        sidebar_split.resize_split(split_index, config.sidebar_default_width);
        self.sidebar_split = sidebar_split;
        self.config = config;
        self
    }
}
