//! Check if a minimap line is in the viewport.

use crate::markdown_widget::extensions::minimap::Minimap;

impl<'a> Minimap<'a> {
    /// Check if a minimap line index falls within the viewport.
    ///
    /// # Arguments
    ///
    /// * `minimap_line` - The line index in the minimap
    /// * `minimap_height` - Total height of the minimap in lines
    ///
    /// # Returns
    ///
    /// True if the minimap line represents content within the viewport.
    pub fn is_in_viewport(&self, minimap_line: usize, minimap_height: usize) -> bool {
        if self.total_lines == 0 || minimap_height == 0 {
            return false;
        }

        // Map minimap line to source line range
        let lines_per_minimap_line = (self.total_lines as f32 / minimap_height as f32).ceil();
        let source_start = (minimap_line as f32 * lines_per_minimap_line) as usize;
        let source_end = ((minimap_line + 1) as f32 * lines_per_minimap_line) as usize;

        // Check if this range overlaps with viewport
        source_start < self.viewport_end && source_end > self.viewport_start
    }
}

#[cfg(test)]
mod tests {
    use crate::markdown_widget::extensions::minimap::Minimap;

    #[test]
    fn test_is_in_viewport() {
        let content = "line1\nline2\nline3\nline4\nline5";
        // viewport(1, 3, 5) means viewing lines 1-2 (viewport_end is exclusive)
        let minimap = Minimap::new(content).viewport(1, 3, 5);

        // With 5 lines and minimap height 5, each minimap line = 1 source line
        // Minimap line 0 = source line 0, not in viewport 1-3
        assert!(!minimap.is_in_viewport(0, 5));
        // Minimap line 1 = source line 1, in viewport 1-3
        assert!(minimap.is_in_viewport(1, 5));
        // Minimap line 2 = source line 2, in viewport 1-3
        assert!(minimap.is_in_viewport(2, 5));
        // Minimap line 3 = source line 3, not in viewport 1-3
        assert!(!minimap.is_in_viewport(3, 5));
        // Minimap line 4 = source line 4, not in viewport 1-3
        assert!(!minimap.is_in_viewport(4, 5));
    }
}
