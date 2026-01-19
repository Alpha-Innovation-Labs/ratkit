//! Calculate required width for expanded TOC mode.

use unicode_width::UnicodeWidthStr;

use super::super::Toc;

impl<'a> Toc<'a> {
    /// Calculate the required width to display all headings without truncation.
    ///
    /// Takes into account:
    /// - Indentation based on heading level (2 chars per level)
    /// - Actual text width using Unicode width
    /// - Padding (left and right)
    /// - Border if enabled
    ///
    /// # Arguments
    ///
    /// * `content` - The markdown content to scan.
    /// * `show_border` - Whether the border is shown.
    ///
    /// # Returns
    ///
    /// The required width in columns.
    pub fn required_expanded_width(content: &str, show_border: bool) -> u16 {
        let padding_left: u16 = 2;
        let padding_right: u16 = 1;
        let border_width: u16 = if show_border { 2 } else { 0 };

        let mut max_width: u16 = 0;

        for line in content.lines() {
            let trimmed = line.trim_start();
            if !trimmed.starts_with('#') {
                continue;
            }

            // Count heading level
            let hash_count = trimmed.chars().take_while(|&c| c == '#').count();
            if !(1..=6).contains(&hash_count) {
                continue;
            }

            // Extract heading text
            let after_hashes = &trimmed[hash_count..];
            if !after_hashes.starts_with(' ') && !after_hashes.is_empty() {
                continue;
            }

            let text = after_hashes.trim();
            if text.is_empty() {
                continue;
            }

            // Calculate width: indent + text width
            let indent = ((hash_count - 1) * 2) as u16;
            let text_width = text.width() as u16;
            let entry_width = indent + text_width;

            if entry_width > max_width {
                max_width = entry_width;
            }
        }

        // Minimum width if no headings found
        if max_width == 0 {
            return if show_border { 10 } else { 8 };
        }

        max_width + padding_left + padding_right + border_width
    }
}
