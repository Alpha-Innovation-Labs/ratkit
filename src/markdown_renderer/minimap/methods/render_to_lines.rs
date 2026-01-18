//! Render minimap content to styled lines.

use ratatui::text::{Line, Span};

use super::super::Minimap;
use super::helpers::density_pair_to_braille;

impl<'a> Minimap<'a> {
    /// Render the minimap to a vector of styled Lines.
    ///
    /// # Arguments
    ///
    /// * `height` - The available height in terminal rows
    ///
    /// # Returns
    ///
    /// A vector of `Line` instances ready for rendering.
    pub fn render_to_lines(&self, height: usize) -> Vec<Line<'static>> {
        if height == 0 || self.total_lines == 0 {
            return vec![];
        }

        let densities = self.line_densities();
        let lines_per_row = (self.total_lines as f32 / height as f32).ceil() as usize;
        let chars_per_row = (self.width as usize / 2).max(1); // Each braille char = 2 columns

        let mut result = Vec::with_capacity(height);

        for row in 0..height {
            let source_start = row * lines_per_row;
            if source_start >= densities.len() {
                // Past end of content, add empty line
                result.push(Line::from(Span::styled(
                    " ".repeat(self.width as usize),
                    self.config.background_style,
                )));
                continue;
            }

            let source_end = ((row + 1) * lines_per_row).min(densities.len());
            let is_viewport = self.is_in_viewport(row, height);

            // Calculate average density for this row
            let avg_density = if source_start < source_end {
                densities[source_start..source_end].iter().sum::<f32>()
                    / (source_end - source_start) as f32
            } else {
                0.0
            };

            // Build the braille string for this row
            let mut braille_chars = String::with_capacity(chars_per_row);
            for _ in 0..chars_per_row {
                // Use density_pair_to_braille with same density for both columns
                // This creates a more uniform look
                let ch = density_pair_to_braille(avg_density, avg_density);
                braille_chars.push(ch);
            }

            // Pad to full width if needed
            let current_width = braille_chars.chars().count();
            if current_width < self.width as usize {
                braille_chars.push_str(&" ".repeat(self.width as usize - current_width));
            }

            let style = if is_viewport {
                self.config.viewport_style
            } else {
                self.config.text_style
            };

            result.push(Line::from(Span::styled(braille_chars, style)));
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::super::super::Minimap;

    #[test]
    fn test_render_to_lines_empty() {
        let minimap = Minimap::new("");
        let lines = minimap.render_to_lines(5);
        assert!(lines.is_empty());
    }

    #[test]
    fn test_render_to_lines_basic() {
        let content = "hello\nworld\ntest";
        let minimap = Minimap::new(content).width(4);
        let lines = minimap.render_to_lines(3);

        assert_eq!(lines.len(), 3);
    }
}
