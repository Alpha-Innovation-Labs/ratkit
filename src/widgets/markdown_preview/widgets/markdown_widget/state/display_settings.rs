//! Display settings for markdown widget.
//!
//! Manages display-related configuration like line numbers and themes.

use crate::widgets::markdown_preview::widgets::markdown_widget::foundation::elements::CodeBlockTheme;

/// Display settings for markdown rendering.
///
/// Controls visual options like line numbers, themes, and collapse indicators.
#[derive(Debug, Clone)]
pub struct DisplaySettings {
    /// Whether to show line numbers in code blocks.
    pub show_line_numbers: bool,
    /// Whether to show line numbers for the entire document.
    pub show_document_line_numbers: bool,
    /// Color theme for code blocks.
    pub code_block_theme: CodeBlockTheme,
    /// Whether to show collapse indicators on headings.
    pub show_heading_collapse: bool,
    /// Scroll multiplier (lines per scroll tick).
    pub scroll_multiplier: usize,
}

impl DisplaySettings {
    /// Create new display settings with defaults.
    pub fn new() -> Self {
        Self {
            show_line_numbers: false,
            show_document_line_numbers: false,
            code_block_theme: CodeBlockTheme::default(),
            show_heading_collapse: false,
            scroll_multiplier: 3,
        }
    }

    /// Set the code block color theme.
    ///
    /// # Arguments
    ///
    /// * `theme` - The theme to use for code blocks.
    ///
    /// # Returns
    ///
    /// `true` if the value changed (caller should invalidate cache).
    pub fn set_code_block_theme(&mut self, theme: CodeBlockTheme) -> bool {
        if self.code_block_theme != theme {
            self.code_block_theme = theme;
            true
        } else {
            false
        }
    }

    /// Set the scroll multiplier (lines per scroll tick).
    ///
    /// # Arguments
    ///
    /// * `multiplier` - Number of lines to scroll per tick.
    ///
    /// # Returns
    ///
    /// `true` if the value changed (caller should invalidate cache).
    pub fn set_scroll_multiplier(&mut self, multiplier: usize) -> bool {
        if self.scroll_multiplier != multiplier {
            self.scroll_multiplier = multiplier;
            true
        } else {
            false
        }
    }

    /// Get the current scroll multiplier.
    pub fn scroll_multiplier(&self) -> usize {
        self.scroll_multiplier
    }

    /// Enable or disable document-wide line numbers.
    ///
    /// # Arguments
    ///
    /// * `show` - Whether to show document line numbers.
    ///
    /// # Returns
    ///
    /// `true` if the value changed (caller should invalidate cache).
    pub fn set_show_document_line_numbers(&mut self, show: bool) -> bool {
        if self.show_document_line_numbers != show {
            self.show_document_line_numbers = show;
            true
        } else {
            false
        }
    }

    /// Enable or disable collapse indicators on headings.
    ///
    /// # Arguments
    ///
    /// * `show` - Whether to show collapse indicators.
    ///
    /// # Returns
    ///
    /// `true` if the value changed (caller should invalidate cache).
    pub fn set_show_heading_collapse(&mut self, show: bool) -> bool {
        if self.show_heading_collapse != show {
            self.show_heading_collapse = show;
            true
        } else {
            false
        }
    }

    /// Enable or disable line numbers in code blocks.
    ///
    /// # Arguments
    ///
    /// * `show` - Whether to show line numbers.
    ///
    /// # Returns
    ///
    /// `true` if the value changed (caller should invalidate cache).
    pub fn set_show_line_numbers(&mut self, show: bool) -> bool {
        if self.show_line_numbers != show {
            self.show_line_numbers = show;
            true
        } else {
            false
        }
    }
}

impl Default for DisplaySettings {
    fn default() -> Self {
        Self::new()
    }
}
