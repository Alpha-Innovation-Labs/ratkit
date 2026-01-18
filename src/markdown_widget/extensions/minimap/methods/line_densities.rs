//! Calculate line densities for minimap rendering.

use crate::markdown_widget::extensions::minimap::Minimap;

impl<'a> Minimap<'a> {
    /// Get density values for all lines in the content.
    ///
    /// # Returns
    ///
    /// A vector of density values (0.0 to 1.0) for each line.
    pub fn line_densities(&self) -> Vec<f32> {
        let max_width = self.max_line_width().max(1);

        self.content
            .lines()
            .map(|line| {
                let char_count = line.chars().filter(|c| !c.is_whitespace()).count();
                (char_count as f32 / max_width as f32).min(1.0)
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use crate::markdown_widget::extensions::minimap::Minimap;

    #[test]
    fn test_line_densities() {
        let content = "hello\n\nworld";
        let minimap = Minimap::new(content);
        let densities = minimap.line_densities();

        assert_eq!(densities.len(), 3);
        assert_eq!(densities[0], 1.0); // "hello" is max
        assert_eq!(densities[1], 0.0); // empty line
        assert_eq!(densities[2], 1.0); // "world" same length
    }
}
