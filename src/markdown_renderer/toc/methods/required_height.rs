//! Static method to calculate required TOC height.

use super::super::Toc;

impl<'a> Toc<'a> {
    /// Calculate the required height for expanded mode.
    ///
    /// Accounts for border (2 rows) and one row per entry.
    ///
    /// # Arguments
    ///
    /// * `content` - The markdown content to scan.
    /// * `show_border` - Whether the border is shown.
    ///
    /// # Returns
    ///
    /// The required height in rows.
    pub fn required_height(content: &str, show_border: bool) -> u16 {
        let heading_count = Self::count_headings(content) as u16;
        let border_height = if show_border { 2 } else { 0 };
        heading_count + border_height
    }
}
