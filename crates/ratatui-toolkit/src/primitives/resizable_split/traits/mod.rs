use crate::primitives::resizable_split::ResizableSplit;

impl Default for ResizableSplit {
    fn default() -> Self {
        Self::new(70)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::SplitDirection;
    use ratatui::layout::Rect;

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
        assert_eq!(split.divider_pos, 50);
    }

    #[test]
    fn test_update_divider_position_horizontal() {
        let mut split = ResizableSplit::new_with_direction(50, SplitDirection::Horizontal);
        let area = Rect::new(0, 0, 100, 20);

        split.update_divider_position(area);
        assert_eq!(split.divider_pos, 10);
    }

    #[test]
    fn test_is_on_divider_vertical() {
        let mut split = ResizableSplit::new(50);
        let area = Rect::new(0, 0, 100, 20);
        split.update_divider_position(area);

        assert!(split.is_on_divider(49, 10, area));
        assert!(split.is_on_divider(50, 10, area));
        assert!(split.is_on_divider(51, 10, area));
        assert!(!split.is_on_divider(47, 10, area));
        assert!(!split.is_on_divider(53, 10, area));
    }

    #[test]
    fn test_is_on_divider_horizontal() {
        let mut split = ResizableSplit::new_with_direction(50, SplitDirection::Horizontal);
        let area = Rect::new(0, 0, 100, 20);
        split.update_divider_position(area);

        assert!(split.is_on_divider(50, 9, area));
        assert!(split.is_on_divider(50, 10, area));
        assert!(split.is_on_divider(50, 11, area));
        assert!(!split.is_on_divider(50, 7, area));
        assert!(!split.is_on_divider(50, 13, area));
    }

    #[test]
    fn test_update_from_mouse_vertical() {
        let mut split = ResizableSplit::new(50);
        let area = Rect::new(0, 0, 100, 20);

        split.update_from_mouse(75, 10, area);
        assert_eq!(split.split_percent, 50);

        split.start_drag();
        split.update_from_mouse(75, 10, area);
        assert_eq!(split.split_percent, 75);

        split.update_from_mouse(99, 10, area);
        assert_eq!(split.split_percent, 90);

        split.update_from_mouse(1, 10, area);
        assert_eq!(split.split_percent, 10);
    }

    #[test]
    fn test_update_from_mouse_horizontal() {
        let mut split = ResizableSplit::new_with_direction(50, SplitDirection::Horizontal);
        let area = Rect::new(0, 0, 100, 20);

        split.update_from_mouse(50, 15, area);
        assert_eq!(split.split_percent, 50);

        split.start_drag();
        split.update_from_mouse(50, 15, area);
        assert_eq!(split.split_percent, 75);

        split.update_from_mouse(50, 19, area);
        assert_eq!(split.split_percent, 90);

        split.update_from_mouse(50, 1, area);
        assert_eq!(split.split_percent, 10);
    }
}
