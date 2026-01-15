use ratatui::layout::Rect;
use ratatui::Frame;

/// Direction of the split
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SplitDirection {
    /// Vertical split (left/right panels) - divider is vertical, mouse drags horizontally
    Vertical,
    /// Horizontal split (top/bottom panels) - divider is horizontal, mouse drags vertically
    Horizontal,
}

/// Tracks the state of a resizable split
#[derive(Debug, Clone)]
pub struct ResizableSplit {
    /// Current split position as percentage (0-100)
    /// For Vertical: left panel percentage (e.g., 70 means left is 70%, right is 30%)
    /// For Horizontal: top panel percentage (e.g., 70 means top is 70%, bottom is 30%)
    pub split_percent: u16,
    /// Minimum percentage for first panel (left or top)
    pub min_percent: u16,
    /// Maximum percentage for first panel (left or top)
    pub max_percent: u16,
    /// Whether currently dragging the divider
    pub is_dragging: bool,
    /// Whether mouse is hovering over the divider
    pub is_hovering: bool,
    /// Direction of the split
    pub direction: SplitDirection,
    /// The column or row position of the divider (updated each frame)
    pub divider_pos: u16,
}

impl ResizableSplit {
    /// Create a new vertical split (default)
    pub fn new(initial_percent: u16) -> Self {
        Self::new_with_direction(initial_percent, SplitDirection::Vertical)
    }

    /// Create a new split with a specific direction
    pub fn new_with_direction(initial_percent: u16, direction: SplitDirection) -> Self {
        Self {
            split_percent: initial_percent.clamp(5, 95),
            min_percent: 10,
            max_percent: 90,
            is_dragging: false,
            is_hovering: false,
            direction,
            divider_pos: 0,
        }
    }

    /// Update the divider position based on the current area
    pub fn update_divider_position(&mut self, area: Rect) {
        match self.direction {
            SplitDirection::Vertical => {
                // Calculate where the divider is in absolute columns
                // The divider is at the boundary between left and right panels
                let left_width = (area.width as u32 * self.split_percent as u32 / 100) as u16;
                self.divider_pos = area.x + left_width;
            }
            SplitDirection::Horizontal => {
                // Calculate where the divider is in absolute rows
                // The divider is at the boundary between top and bottom panels
                let top_height = (area.height as u32 * self.split_percent as u32 / 100) as u16;
                self.divider_pos = area.y + top_height;
            }
        }
    }

    /// Check if a mouse position is on the divider
    /// We give it a 3-unit wide hit area for easier clicking (columns for vertical, rows for horizontal)
    pub fn is_on_divider(&self, mouse_column: u16, mouse_row: u16, area: Rect) -> bool {
        match self.direction {
            SplitDirection::Vertical => {
                let divider_start = self.divider_pos.saturating_sub(1);
                let divider_end = (self.divider_pos + 1).min(area.x + area.width.saturating_sub(1));
                mouse_column >= divider_start && mouse_column <= divider_end
            }
            SplitDirection::Horizontal => {
                let divider_start = self.divider_pos.saturating_sub(1);
                let divider_end =
                    (self.divider_pos + 1).min(area.y + area.height.saturating_sub(1));
                mouse_row >= divider_start && mouse_row <= divider_end
            }
        }
    }

    /// Start dragging the divider
    pub fn start_drag(&mut self) {
        self.is_dragging = true;
    }

    /// Stop dragging the divider
    pub fn stop_drag(&mut self) {
        self.is_dragging = false;
    }

    /// Update the split position based on mouse position
    pub fn update_from_mouse(&mut self, mouse_column: u16, mouse_row: u16, area: Rect) {
        if !self.is_dragging {
            return;
        }

        let new_percent = match self.direction {
            SplitDirection::Vertical => {
                // Calculate the relative position within the area
                let relative_column = mouse_column.saturating_sub(area.x);

                // Convert to percentage
                if area.width > 0 {
                    ((relative_column as u32 * 100) / area.width as u32) as u16
                } else {
                    self.split_percent
                }
            }
            SplitDirection::Horizontal => {
                // Calculate the relative position within the area
                let relative_row = mouse_row.saturating_sub(area.y);

                // Convert to percentage
                if area.height > 0 {
                    ((relative_row as u32 * 100) / area.height as u32) as u16
                } else {
                    self.split_percent
                }
            }
        };

        // Clamp to min/max
        self.split_percent = new_percent.clamp(self.min_percent, self.max_percent);
    }

    /// Get the percentage for the second panel (right for vertical, bottom for horizontal)
    pub fn right_percent(&self) -> u16 {
        100 - self.split_percent
    }

    /// Alias for right_percent - gets the percentage for the bottom panel
    pub fn bottom_percent(&self) -> u16 {
        self.right_percent()
    }

    /// Render a visual indicator on the divider when hovering or dragging
    /// Note: Does nothing - border colors are changed on the panes themselves
    pub fn render_divider_indicator(&self, _frame: &mut Frame, _area: Rect) {
        // No separator rendering - the pane borders themselves change color
    }
}

impl Default for ResizableSplit {
    fn default() -> Self {
        Self::new(70)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_clamps_percentage() {
        let split = ResizableSplit::new(150);
        assert_eq!(split.split_percent, 95);

        let split = ResizableSplit::new(0);
        assert_eq!(split.split_percent, 5);
    }

    #[test]
    fn test_update_divider_position_vertical() {
        let mut split = ResizableSplit::new(50);
        let area = Rect::new(0, 0, 100, 20);

        split.update_divider_position(area);
        // 50% of 100 columns = 50 (the boundary between left and right panels)
        assert_eq!(split.divider_pos, 50);
    }

    #[test]
    fn test_update_divider_position_horizontal() {
        let mut split = ResizableSplit::new_with_direction(50, SplitDirection::Horizontal);
        let area = Rect::new(0, 0, 100, 20);

        split.update_divider_position(area);
        // 50% of 20 rows = 10 (the boundary between top and bottom panels)
        assert_eq!(split.divider_pos, 10);
    }

    #[test]
    fn test_is_on_divider_vertical() {
        let mut split = ResizableSplit::new(50);
        let area = Rect::new(0, 0, 100, 20);
        split.update_divider_position(area);

        // Should hit within 3-column range (divider at column 50)
        assert!(split.is_on_divider(49, 10, area)); // 1 before
        assert!(split.is_on_divider(50, 10, area)); // exact
        assert!(split.is_on_divider(51, 10, area)); // 1 after
        assert!(!split.is_on_divider(47, 10, area)); // too far before
        assert!(!split.is_on_divider(53, 10, area)); // too far after
    }

    #[test]
    fn test_is_on_divider_horizontal() {
        let mut split = ResizableSplit::new_with_direction(50, SplitDirection::Horizontal);
        let area = Rect::new(0, 0, 100, 20);
        split.update_divider_position(area);

        // Should hit within 3-row range (divider at row 10)
        assert!(split.is_on_divider(50, 9, area)); // 1 before
        assert!(split.is_on_divider(50, 10, area)); // exact
        assert!(split.is_on_divider(50, 11, area)); // 1 after
        assert!(!split.is_on_divider(50, 7, area)); // too far before
        assert!(!split.is_on_divider(50, 13, area)); // too far after
    }

    #[test]
    fn test_update_from_mouse_vertical() {
        let mut split = ResizableSplit::new(50);
        let area = Rect::new(0, 0, 100, 20);

        // Not dragging - should not update
        split.update_from_mouse(75, 10, area);
        assert_eq!(split.split_percent, 50);

        // Start dragging and update
        split.start_drag();
        split.update_from_mouse(75, 10, area);
        assert_eq!(split.split_percent, 75);

        // Test clamping to max
        split.update_from_mouse(99, 10, area);
        assert_eq!(split.split_percent, 90); // max_percent

        // Test clamping to min
        split.update_from_mouse(1, 10, area);
        assert_eq!(split.split_percent, 10); // min_percent
    }

    #[test]
    fn test_update_from_mouse_horizontal() {
        let mut split = ResizableSplit::new_with_direction(50, SplitDirection::Horizontal);
        let area = Rect::new(0, 0, 100, 20);

        // Not dragging - should not update
        split.update_from_mouse(50, 15, area);
        assert_eq!(split.split_percent, 50);

        // Start dragging and update
        split.start_drag();
        split.update_from_mouse(50, 15, area);
        assert_eq!(split.split_percent, 75); // 15/20 = 75%

        // Test clamping to max
        split.update_from_mouse(50, 19, area);
        assert_eq!(split.split_percent, 90); // max_percent

        // Test clamping to min
        split.update_from_mouse(50, 1, area);
        assert_eq!(split.split_percent, 10); // min_percent
    }
}
