use crate::code_diff::code_diff::CodeDiff;
use crate::code_diff::diff_config::DiffConfig;
use crate::primitives::resizable_split::ResizableSplit;

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
        // Update sidebar state from config
        self.show_sidebar = config.sidebar_enabled;
        // Create new ResizableSplit with config values
        let mut sidebar_split = ResizableSplit::new(config.sidebar_default_width);
        sidebar_split.min_percent = config.sidebar_min_width;
        sidebar_split.max_percent = config.sidebar_max_width;
        self.sidebar_split = sidebar_split;
        self.config = config;
        self
    }
}
