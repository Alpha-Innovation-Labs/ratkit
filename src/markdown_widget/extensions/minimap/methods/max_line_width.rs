//! Calculate maximum line width for minimap rendering.

use crate::markdown_widget::extensions::minimap::Minimap;

impl<'a> Minimap<'a> {
    /// Calculate the maximum line width in the content.
    ///
    /// Used for normalizing density calculations.
    pub fn max_line_width(&self) -> usize {
        self.content
            .lines()
            .map(|line| line.chars().filter(|c| !c.is_whitespace()).count())
            .max()
            .unwrap_or(0)
    }
}

#[cfg(test)]
mod tests {
    use crate::markdown_widget::extensions::minimap::Minimap;

    #[test]
    fn test_max_line_width() {
        let content = "short\na much longer line here\nmed";
        let minimap = Minimap::new(content);
        // "amuchlongerlinehere" = 19 characters (without spaces)
        assert_eq!(minimap.max_line_width(), 19);
    }
}
