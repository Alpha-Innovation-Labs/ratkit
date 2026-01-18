//! Calculate source line from minimap click position.

use super::super::Minimap;

impl<'a> Minimap<'a> {
    /// Calculate which source line corresponds to a minimap click.
    ///
    /// # Arguments
    ///
    /// * `minimap_y` - The y coordinate clicked within the minimap
    /// * `minimap_height` - Total height of the minimap
    ///
    /// # Returns
    ///
    /// The source line number that should be scrolled to.
    pub fn click_to_line(&self, minimap_y: usize, minimap_height: usize) -> usize {
        if minimap_height == 0 {
            return 0;
        }

        let ratio = minimap_y as f32 / minimap_height as f32;
        ((ratio * self.total_lines as f32) as usize).min(self.total_lines.saturating_sub(1))
    }
}

#[cfg(test)]
mod tests {
    use super::super::super::Minimap;

    #[test]
    fn test_click_to_line() {
        let content = "a\nb\nc\nd\ne\nf\ng\nh\ni\nj"; // 10 lines
        let minimap = Minimap::new(content);

        // Click at top should go to line 0
        assert_eq!(minimap.click_to_line(0, 10), 0);

        // Click at middle should go to middle
        assert_eq!(minimap.click_to_line(5, 10), 5);

        // Click at bottom should go to end
        assert_eq!(minimap.click_to_line(9, 10), 9);
    }
}
