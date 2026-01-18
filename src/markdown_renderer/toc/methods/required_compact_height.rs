//! Calculate required height for compact TOC mode.

use super::super::Toc;

impl<'a> Toc<'a> {
    /// Calculate the required height for compact mode.
    ///
    /// With Braille markers, we have 4 vertical dots per cell.
    /// Height = ceil(entries * spacing / 4) + border_height.
    ///
    /// # Arguments
    ///
    /// * `content` - The markdown content to scan.
    /// * `line_spacing` - Spacing between lines in dot units.
    /// * `show_border` - Whether the border is shown.
    ///
    /// # Returns
    ///
    /// The required height in rows.
    pub fn required_compact_height(content: &str, line_spacing: u8, show_border: bool) -> u16 {
        let entry_count = Self::count_headings(content);
        if entry_count == 0 {
            return if show_border { 3 } else { 1 };
        }

        let spacing = line_spacing.max(1) as f64;
        // Total dots needed = entries * spacing
        // Cells needed = ceil(dots / 4)
        let dots_needed = entry_count as f64 * spacing;
        let cells_needed = (dots_needed / 4.0).ceil() as u16;

        let border_height = if show_border { 2 } else { 0 };
        cells_needed + border_height
    }
}
