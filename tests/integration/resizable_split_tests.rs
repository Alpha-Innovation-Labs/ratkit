use ratatui::layout::Rect;
use ratatui_toolkit::resizable_split::{ResizableSplit, SplitDirection};

// ============================================================================
// INITIALIZATION TESTS
// ============================================================================

#[test]
fn test_new_creates_vertical_split_by_default() {
    let split = ResizableSplit::new(70);
    assert_eq!(split.direction, SplitDirection::Vertical);
    assert_eq!(split.split_percent, 70);
    assert_eq!(split.min_percent, 10);
    assert_eq!(split.max_percent, 90);
    assert!(!split.is_dragging);
    assert!(!split.is_hovering);
    assert_eq!(split.divider_pos, 0);
}

#[test]
fn test_new_with_direction_vertical() {
    let split = ResizableSplit::new_with_direction(60, SplitDirection::Vertical);
    assert_eq!(split.direction, SplitDirection::Vertical);
    assert_eq!(split.split_percent, 60);
}

#[test]
fn test_new_with_direction_horizontal() {
    let split = ResizableSplit::new_with_direction(40, SplitDirection::Horizontal);
    assert_eq!(split.direction, SplitDirection::Horizontal);
    assert_eq!(split.split_percent, 40);
}

#[test]
fn test_default_creates_70_percent_vertical_split() {
    let split = ResizableSplit::default();
    assert_eq!(split.direction, SplitDirection::Vertical);
    assert_eq!(split.split_percent, 70);
}

// ============================================================================
// PERCENTAGE CLAMPING TESTS (INITIALIZATION)
// ============================================================================

#[test]
fn test_new_clamps_percentage_to_min_5() {
    let split = ResizableSplit::new(0);
    assert_eq!(split.split_percent, 5);

    let split = ResizableSplit::new(3);
    assert_eq!(split.split_percent, 5);
}

#[test]
fn test_new_clamps_percentage_to_max_95() {
    let split = ResizableSplit::new(100);
    assert_eq!(split.split_percent, 95);

    let split = ResizableSplit::new(150);
    assert_eq!(split.split_percent, 95);

    let split = ResizableSplit::new(u16::MAX);
    assert_eq!(split.split_percent, 95);
}

#[test]
fn test_new_accepts_valid_percentages() {
    let split = ResizableSplit::new(5);
    assert_eq!(split.split_percent, 5);

    let split = ResizableSplit::new(50);
    assert_eq!(split.split_percent, 50);

    let split = ResizableSplit::new(95);
    assert_eq!(split.split_percent, 95);
}

#[test]
fn test_new_with_direction_clamps_percentage() {
    let split = ResizableSplit::new_with_direction(0, SplitDirection::Vertical);
    assert_eq!(split.split_percent, 5);

    let split = ResizableSplit::new_with_direction(200, SplitDirection::Horizontal);
    assert_eq!(split.split_percent, 95);
}

// ============================================================================
// UPDATE_DIVIDER_POSITION TESTS - VERTICAL
// ============================================================================

#[test]
fn test_update_divider_position_vertical_50_percent() {
    let mut split = ResizableSplit::new(50);
    let area = Rect::new(0, 0, 100, 20);

    split.update_divider_position(area);
    // 50% of 100 columns = 50
    assert_eq!(split.divider_pos, 50);
}

#[test]
fn test_update_divider_position_vertical_70_percent() {
    let mut split = ResizableSplit::new(70);
    let area = Rect::new(0, 0, 100, 20);

    split.update_divider_position(area);
    // 70% of 100 columns = 70
    assert_eq!(split.divider_pos, 70);
}

#[test]
fn test_update_divider_position_vertical_with_offset() {
    let mut split = ResizableSplit::new(50);
    let area = Rect::new(10, 5, 100, 20);

    split.update_divider_position(area);
    // 50% of 100 columns = 50, plus offset 10 = 60
    assert_eq!(split.divider_pos, 60);
}

#[test]
fn test_update_divider_position_vertical_odd_width() {
    let mut split = ResizableSplit::new(50);
    let area = Rect::new(0, 0, 99, 20);

    split.update_divider_position(area);
    // 50% of 99 = 49.5, which rounds down to 49
    assert_eq!(split.divider_pos, 49);
}

#[test]
fn test_update_divider_position_vertical_small_area() {
    let mut split = ResizableSplit::new(50);
    let area = Rect::new(0, 0, 10, 5);

    split.update_divider_position(area);
    // 50% of 10 = 5
    assert_eq!(split.divider_pos, 5);
}

#[test]
fn test_update_divider_position_vertical_extreme_percentages() {
    let mut split = ResizableSplit::new(5);
    let area = Rect::new(0, 0, 100, 20);
    split.update_divider_position(area);
    // 5% of 100 = 5
    assert_eq!(split.divider_pos, 5);

    let mut split = ResizableSplit::new(95);
    let area = Rect::new(0, 0, 100, 20);
    split.update_divider_position(area);
    // 95% of 100 = 95
    assert_eq!(split.divider_pos, 95);
}

// ============================================================================
// UPDATE_DIVIDER_POSITION TESTS - HORIZONTAL
// ============================================================================

#[test]
fn test_update_divider_position_horizontal_50_percent() {
    let mut split = ResizableSplit::new_with_direction(50, SplitDirection::Horizontal);
    let area = Rect::new(0, 0, 100, 20);

    split.update_divider_position(area);
    // 50% of 20 rows = 10
    assert_eq!(split.divider_pos, 10);
}

#[test]
fn test_update_divider_position_horizontal_70_percent() {
    let mut split = ResizableSplit::new_with_direction(70, SplitDirection::Horizontal);
    let area = Rect::new(0, 0, 100, 20);

    split.update_divider_position(area);
    // 70% of 20 rows = 14
    assert_eq!(split.divider_pos, 14);
}

#[test]
fn test_update_divider_position_horizontal_with_offset() {
    let mut split = ResizableSplit::new_with_direction(50, SplitDirection::Horizontal);
    let area = Rect::new(10, 5, 100, 20);

    split.update_divider_position(area);
    // 50% of 20 rows = 10, plus offset 5 = 15
    assert_eq!(split.divider_pos, 15);
}

#[test]
fn test_update_divider_position_horizontal_odd_height() {
    let mut split = ResizableSplit::new_with_direction(50, SplitDirection::Horizontal);
    let area = Rect::new(0, 0, 100, 21);

    split.update_divider_position(area);
    // 50% of 21 = 10.5, which rounds down to 10
    assert_eq!(split.divider_pos, 10);
}

#[test]
fn test_update_divider_position_horizontal_small_area() {
    let mut split = ResizableSplit::new_with_direction(50, SplitDirection::Horizontal);
    let area = Rect::new(0, 0, 10, 8);

    split.update_divider_position(area);
    // 50% of 8 = 4
    assert_eq!(split.divider_pos, 4);
}

#[test]
fn test_update_divider_position_horizontal_extreme_percentages() {
    let mut split = ResizableSplit::new_with_direction(5, SplitDirection::Horizontal);
    let area = Rect::new(0, 0, 100, 100);
    split.update_divider_position(area);
    // 5% of 100 = 5
    assert_eq!(split.divider_pos, 5);

    let mut split = ResizableSplit::new_with_direction(95, SplitDirection::Horizontal);
    let area = Rect::new(0, 0, 100, 100);
    split.update_divider_position(area);
    // 95% of 100 = 95
    assert_eq!(split.divider_pos, 95);
}

// ============================================================================
// IS_ON_DIVIDER TESTS - VERTICAL (3-UNIT HIT AREA)
// ============================================================================

#[test]
fn test_is_on_divider_vertical_exact_position() {
    let mut split = ResizableSplit::new(50);
    let area = Rect::new(0, 0, 100, 20);
    split.update_divider_position(area);

    // Divider at column 50, should hit exactly
    assert!(split.is_on_divider(50, 10, area));
}

#[test]
fn test_is_on_divider_vertical_one_before() {
    let mut split = ResizableSplit::new(50);
    let area = Rect::new(0, 0, 100, 20);
    split.update_divider_position(area);

    // Column 49 should be in hit area (divider_pos - 1)
    assert!(split.is_on_divider(49, 10, area));
}

#[test]
fn test_is_on_divider_vertical_one_after() {
    let mut split = ResizableSplit::new(50);
    let area = Rect::new(0, 0, 100, 20);
    split.update_divider_position(area);

    // Column 51 should be in hit area (divider_pos + 1)
    assert!(split.is_on_divider(51, 10, area));
}

#[test]
fn test_is_on_divider_vertical_two_before() {
    let mut split = ResizableSplit::new(50);
    let area = Rect::new(0, 0, 100, 20);
    split.update_divider_position(area);

    // Column 48 should be outside hit area
    assert!(!split.is_on_divider(48, 10, area));
}

#[test]
fn test_is_on_divider_vertical_two_after() {
    let mut split = ResizableSplit::new(50);
    let area = Rect::new(0, 0, 100, 20);
    split.update_divider_position(area);

    // Column 52 should be outside hit area
    assert!(!split.is_on_divider(52, 10, area));
}

#[test]
fn test_is_on_divider_vertical_at_left_edge() {
    let mut split = ResizableSplit::new(5);
    let area = Rect::new(0, 0, 100, 20);
    split.update_divider_position(area);

    // Divider at column 5
    assert!(split.is_on_divider(5, 10, area));
    assert!(split.is_on_divider(4, 10, area));
    assert!(split.is_on_divider(6, 10, area));
}

#[test]
fn test_is_on_divider_vertical_at_right_edge() {
    let mut split = ResizableSplit::new(95);
    let area = Rect::new(0, 0, 100, 20);
    split.update_divider_position(area);

    // Divider at column 95
    assert!(split.is_on_divider(95, 10, area));
    assert!(split.is_on_divider(94, 10, area));
    // Column 96 is clamped to area boundary (99)
    assert!(split.is_on_divider(96, 10, area));
}

#[test]
fn test_is_on_divider_vertical_with_area_offset() {
    let mut split = ResizableSplit::new(50);
    let area = Rect::new(10, 5, 100, 20);
    split.update_divider_position(area);

    // Divider at column 60 (50 + 10 offset)
    assert!(split.is_on_divider(60, 10, area));
    assert!(split.is_on_divider(59, 10, area));
    assert!(split.is_on_divider(61, 10, area));
    assert!(!split.is_on_divider(58, 10, area));
    assert!(!split.is_on_divider(62, 10, area));
}

#[test]
fn test_is_on_divider_vertical_row_doesnt_matter() {
    let mut split = ResizableSplit::new(50);
    let area = Rect::new(0, 0, 100, 20);
    split.update_divider_position(area);

    // For vertical split, row position doesn't matter
    assert!(split.is_on_divider(50, 0, area));
    assert!(split.is_on_divider(50, 10, area));
    assert!(split.is_on_divider(50, 19, area));
}

// ============================================================================
// IS_ON_DIVIDER TESTS - HORIZONTAL (3-UNIT HIT AREA)
// ============================================================================

#[test]
fn test_is_on_divider_horizontal_exact_position() {
    let mut split = ResizableSplit::new_with_direction(50, SplitDirection::Horizontal);
    let area = Rect::new(0, 0, 100, 20);
    split.update_divider_position(area);

    // Divider at row 10, should hit exactly
    assert!(split.is_on_divider(50, 10, area));
}

#[test]
fn test_is_on_divider_horizontal_one_before() {
    let mut split = ResizableSplit::new_with_direction(50, SplitDirection::Horizontal);
    let area = Rect::new(0, 0, 100, 20);
    split.update_divider_position(area);

    // Row 9 should be in hit area (divider_pos - 1)
    assert!(split.is_on_divider(50, 9, area));
}

#[test]
fn test_is_on_divider_horizontal_one_after() {
    let mut split = ResizableSplit::new_with_direction(50, SplitDirection::Horizontal);
    let area = Rect::new(0, 0, 100, 20);
    split.update_divider_position(area);

    // Row 11 should be in hit area (divider_pos + 1)
    assert!(split.is_on_divider(50, 11, area));
}

#[test]
fn test_is_on_divider_horizontal_two_before() {
    let mut split = ResizableSplit::new_with_direction(50, SplitDirection::Horizontal);
    let area = Rect::new(0, 0, 100, 20);
    split.update_divider_position(area);

    // Row 8 should be outside hit area
    assert!(!split.is_on_divider(50, 8, area));
}

#[test]
fn test_is_on_divider_horizontal_two_after() {
    let mut split = ResizableSplit::new_with_direction(50, SplitDirection::Horizontal);
    let area = Rect::new(0, 0, 100, 20);
    split.update_divider_position(area);

    // Row 12 should be outside hit area
    assert!(!split.is_on_divider(50, 12, area));
}

#[test]
fn test_is_on_divider_horizontal_at_top_edge() {
    let mut split = ResizableSplit::new_with_direction(5, SplitDirection::Horizontal);
    let area = Rect::new(0, 0, 100, 100);
    split.update_divider_position(area);

    // Divider at row 5
    assert!(split.is_on_divider(50, 5, area));
    assert!(split.is_on_divider(50, 4, area));
    assert!(split.is_on_divider(50, 6, area));
}

#[test]
fn test_is_on_divider_horizontal_at_bottom_edge() {
    let mut split = ResizableSplit::new_with_direction(95, SplitDirection::Horizontal);
    let area = Rect::new(0, 0, 100, 100);
    split.update_divider_position(area);

    // Divider at row 95
    assert!(split.is_on_divider(50, 95, area));
    assert!(split.is_on_divider(50, 94, area));
    // Row 96 is clamped to area boundary (99)
    assert!(split.is_on_divider(50, 96, area));
}

#[test]
fn test_is_on_divider_horizontal_with_area_offset() {
    let mut split = ResizableSplit::new_with_direction(50, SplitDirection::Horizontal);
    let area = Rect::new(10, 5, 100, 20);
    split.update_divider_position(area);

    // Divider at row 15 (10 + 5 offset)
    assert!(split.is_on_divider(50, 15, area));
    assert!(split.is_on_divider(50, 14, area));
    assert!(split.is_on_divider(50, 16, area));
    assert!(!split.is_on_divider(50, 13, area));
    assert!(!split.is_on_divider(50, 17, area));
}

#[test]
fn test_is_on_divider_horizontal_column_doesnt_matter() {
    let mut split = ResizableSplit::new_with_direction(50, SplitDirection::Horizontal);
    let area = Rect::new(0, 0, 100, 20);
    split.update_divider_position(area);

    // For horizontal split, column position doesn't matter
    assert!(split.is_on_divider(0, 10, area));
    assert!(split.is_on_divider(50, 10, area));
    assert!(split.is_on_divider(99, 10, area));
}

// ============================================================================
// START_DRAG / STOP_DRAG STATE MANAGEMENT TESTS
// ============================================================================

#[test]
fn test_start_drag_sets_is_dragging() {
    let mut split = ResizableSplit::new(50);
    assert!(!split.is_dragging);

    split.start_drag();
    assert!(split.is_dragging);
}

#[test]
fn test_stop_drag_clears_is_dragging() {
    let mut split = ResizableSplit::new(50);
    split.start_drag();
    assert!(split.is_dragging);

    split.stop_drag();
    assert!(!split.is_dragging);
}

#[test]
fn test_multiple_start_drag_calls() {
    let mut split = ResizableSplit::new(50);

    split.start_drag();
    assert!(split.is_dragging);

    split.start_drag();
    assert!(split.is_dragging);
}

#[test]
fn test_multiple_stop_drag_calls() {
    let mut split = ResizableSplit::new(50);

    split.stop_drag();
    assert!(!split.is_dragging);

    split.stop_drag();
    assert!(!split.is_dragging);
}

#[test]
fn test_drag_lifecycle() {
    let mut split = ResizableSplit::new(50);

    assert!(!split.is_dragging);
    split.start_drag();
    assert!(split.is_dragging);
    split.stop_drag();
    assert!(!split.is_dragging);
    split.start_drag();
    assert!(split.is_dragging);
    split.stop_drag();
    assert!(!split.is_dragging);
}

// ============================================================================
// UPDATE_FROM_MOUSE TESTS - VERTICAL
// ============================================================================

#[test]
fn test_update_from_mouse_vertical_does_not_update_when_not_dragging() {
    let mut split = ResizableSplit::new(50);
    let area = Rect::new(0, 0, 100, 20);

    split.update_from_mouse(75, 10, area);
    assert_eq!(split.split_percent, 50); // Should not change
}

#[test]
fn test_update_from_mouse_vertical_updates_when_dragging() {
    let mut split = ResizableSplit::new(50);
    let area = Rect::new(0, 0, 100, 20);

    split.start_drag();
    split.update_from_mouse(75, 10, area);
    assert_eq!(split.split_percent, 75);
}

#[test]
fn test_update_from_mouse_vertical_calculates_percentage_correctly() {
    let mut split = ResizableSplit::new(50);
    let area = Rect::new(0, 0, 100, 20);
    split.start_drag();

    split.update_from_mouse(25, 10, area);
    assert_eq!(split.split_percent, 25);

    split.update_from_mouse(75, 10, area);
    assert_eq!(split.split_percent, 75);

    split.update_from_mouse(50, 10, area);
    assert_eq!(split.split_percent, 50);
}

#[test]
fn test_update_from_mouse_vertical_with_area_offset() {
    let mut split = ResizableSplit::new(50);
    let area = Rect::new(10, 5, 100, 20);
    split.start_drag();

    // Mouse at column 60, area starts at 10, so relative is 50
    // 50 / 100 = 50%
    split.update_from_mouse(60, 10, area);
    assert_eq!(split.split_percent, 50);

    // Mouse at column 85, area starts at 10, so relative is 75
    // 75 / 100 = 75%
    split.update_from_mouse(85, 10, area);
    assert_eq!(split.split_percent, 75);
}

#[test]
fn test_update_from_mouse_vertical_clamps_to_min_percent() {
    let mut split = ResizableSplit::new(50);
    let area = Rect::new(0, 0, 100, 20);
    split.start_drag();

    // Try to set to 5% (below min of 10%)
    split.update_from_mouse(5, 10, area);
    assert_eq!(split.split_percent, 10); // Clamped to min_percent

    // Try to set to 0%
    split.update_from_mouse(0, 10, area);
    assert_eq!(split.split_percent, 10);
}

#[test]
fn test_update_from_mouse_vertical_clamps_to_max_percent() {
    let mut split = ResizableSplit::new(50);
    let area = Rect::new(0, 0, 100, 20);
    split.start_drag();

    // Try to set to 95% (above max of 90%)
    split.update_from_mouse(95, 10, area);
    assert_eq!(split.split_percent, 90); // Clamped to max_percent

    // Try to set to 100%
    split.update_from_mouse(99, 10, area);
    assert_eq!(split.split_percent, 90);
}

#[test]
fn test_update_from_mouse_vertical_mouse_before_area() {
    let mut split = ResizableSplit::new(50);
    let area = Rect::new(10, 5, 100, 20);
    split.start_drag();

    // Mouse at column 5, before area starts (area.x = 10)
    // saturating_sub will give 0
    split.update_from_mouse(5, 10, area);
    assert_eq!(split.split_percent, 10); // Clamped to min_percent
}

#[test]
fn test_update_from_mouse_vertical_mouse_after_area() {
    let mut split = ResizableSplit::new(50);
    let area = Rect::new(0, 0, 100, 20);
    split.start_drag();

    // Mouse at column 150, beyond area width (100)
    // relative = 150, (150 * 100) / 100 = 150%
    split.update_from_mouse(150, 10, area);
    assert_eq!(split.split_percent, 90); // Clamped to max_percent
}

#[test]
fn test_update_from_mouse_vertical_zero_width_area() {
    let mut split = ResizableSplit::new(50);
    let area = Rect::new(0, 0, 0, 20);
    split.start_drag();

    // When area width is 0, should not update
    split.update_from_mouse(10, 10, area);
    assert_eq!(split.split_percent, 50); // Should remain unchanged
}

// ============================================================================
// UPDATE_FROM_MOUSE TESTS - HORIZONTAL
// ============================================================================

#[test]
fn test_update_from_mouse_horizontal_does_not_update_when_not_dragging() {
    let mut split = ResizableSplit::new_with_direction(50, SplitDirection::Horizontal);
    let area = Rect::new(0, 0, 100, 20);

    split.update_from_mouse(50, 15, area);
    assert_eq!(split.split_percent, 50); // Should not change
}

#[test]
fn test_update_from_mouse_horizontal_updates_when_dragging() {
    let mut split = ResizableSplit::new_with_direction(50, SplitDirection::Horizontal);
    let area = Rect::new(0, 0, 100, 20);

    split.start_drag();
    split.update_from_mouse(50, 15, area);
    // 15 / 20 = 75%
    assert_eq!(split.split_percent, 75);
}

#[test]
fn test_update_from_mouse_horizontal_calculates_percentage_correctly() {
    let mut split = ResizableSplit::new_with_direction(50, SplitDirection::Horizontal);
    let area = Rect::new(0, 0, 100, 20);
    split.start_drag();

    split.update_from_mouse(50, 5, area);
    assert_eq!(split.split_percent, 25); // 5 / 20 = 25%

    split.update_from_mouse(50, 10, area);
    assert_eq!(split.split_percent, 50); // 10 / 20 = 50%

    split.update_from_mouse(50, 15, area);
    assert_eq!(split.split_percent, 75); // 15 / 20 = 75%
}

#[test]
fn test_update_from_mouse_horizontal_with_area_offset() {
    let mut split = ResizableSplit::new_with_direction(50, SplitDirection::Horizontal);
    let area = Rect::new(10, 5, 100, 20);
    split.start_drag();

    // Mouse at row 15, area starts at 5, so relative is 10
    // 10 / 20 = 50%
    split.update_from_mouse(50, 15, area);
    assert_eq!(split.split_percent, 50);

    // Mouse at row 20, area starts at 5, so relative is 15
    // 15 / 20 = 75%
    split.update_from_mouse(50, 20, area);
    assert_eq!(split.split_percent, 75);
}

#[test]
fn test_update_from_mouse_horizontal_clamps_to_min_percent() {
    let mut split = ResizableSplit::new_with_direction(50, SplitDirection::Horizontal);
    let area = Rect::new(0, 0, 100, 20);
    split.start_drag();

    // Try to set to 5% (below min of 10%)
    split.update_from_mouse(50, 1, area);
    assert_eq!(split.split_percent, 10); // Clamped to min_percent

    // Try to set to 0%
    split.update_from_mouse(50, 0, area);
    assert_eq!(split.split_percent, 10);
}

#[test]
fn test_update_from_mouse_horizontal_clamps_to_max_percent() {
    let mut split = ResizableSplit::new_with_direction(50, SplitDirection::Horizontal);
    let area = Rect::new(0, 0, 100, 20);
    split.start_drag();

    // Try to set to 95% (above max of 90%)
    split.update_from_mouse(50, 19, area);
    assert_eq!(split.split_percent, 90); // Clamped to max_percent

    // Try to set to 100%
    split.update_from_mouse(50, 20, area);
    assert_eq!(split.split_percent, 90);
}

#[test]
fn test_update_from_mouse_horizontal_mouse_before_area() {
    let mut split = ResizableSplit::new_with_direction(50, SplitDirection::Horizontal);
    let area = Rect::new(10, 5, 100, 20);
    split.start_drag();

    // Mouse at row 2, before area starts (area.y = 5)
    // saturating_sub will give 0
    split.update_from_mouse(50, 2, area);
    assert_eq!(split.split_percent, 10); // Clamped to min_percent
}

#[test]
fn test_update_from_mouse_horizontal_mouse_after_area() {
    let mut split = ResizableSplit::new_with_direction(50, SplitDirection::Horizontal);
    let area = Rect::new(0, 0, 100, 20);
    split.start_drag();

    // Mouse at row 50, beyond area height (20)
    // relative = 50, (50 * 100) / 20 = 250%
    split.update_from_mouse(50, 50, area);
    assert_eq!(split.split_percent, 90); // Clamped to max_percent
}

#[test]
fn test_update_from_mouse_horizontal_zero_height_area() {
    let mut split = ResizableSplit::new_with_direction(50, SplitDirection::Horizontal);
    let area = Rect::new(0, 0, 100, 0);
    split.start_drag();

    // When area height is 0, should not update
    split.update_from_mouse(50, 10, area);
    assert_eq!(split.split_percent, 50); // Should remain unchanged
}

// ============================================================================
// BOUNDARY CONDITION TESTS
// ============================================================================

#[test]
fn test_boundary_minimum_area_size_vertical() {
    let mut split = ResizableSplit::new(50);
    let area = Rect::new(0, 0, 1, 1);

    split.update_divider_position(area);
    // 50% of 1 = 0.5, rounds down to 0
    assert_eq!(split.divider_pos, 0);
}

#[test]
fn test_boundary_minimum_area_size_horizontal() {
    let mut split = ResizableSplit::new_with_direction(50, SplitDirection::Horizontal);
    let area = Rect::new(0, 0, 1, 1);

    split.update_divider_position(area);
    // 50% of 1 = 0.5, rounds down to 0
    assert_eq!(split.divider_pos, 0);
}

#[test]
fn test_boundary_max_area_offset_vertical() {
    let mut split = ResizableSplit::new(50);
    let area = Rect::new(u16::MAX - 100, 0, 100, 20);

    split.update_divider_position(area);
    let expected = (u16::MAX - 100) + 50;
    assert_eq!(split.divider_pos, expected);
}

#[test]
fn test_boundary_max_area_offset_horizontal() {
    let mut split = ResizableSplit::new_with_direction(50, SplitDirection::Horizontal);
    let area = Rect::new(0, u16::MAX - 100, 100, 100);

    split.update_divider_position(area);
    let expected = (u16::MAX - 100) + 50;
    assert_eq!(split.divider_pos, expected);
}

#[test]
fn test_boundary_divider_at_position_zero() {
    let mut split = ResizableSplit::new(5);
    let area = Rect::new(0, 0, 100, 20);
    split.update_divider_position(area);

    // Divider at 5, hit area includes columns 4, 5, 6
    // But column 4 - 1 = 3 due to saturating_sub
    assert!(split.is_on_divider(4, 10, area));
    assert!(split.is_on_divider(5, 10, area));
    assert!(split.is_on_divider(6, 10, area));
}

#[test]
fn test_boundary_extreme_split_positions() {
    // Test with minimum allowed percentage
    let mut split = ResizableSplit::new(5);
    let area = Rect::new(0, 0, 100, 20);
    split.start_drag();
    split.update_from_mouse(10, 10, area);
    assert_eq!(split.split_percent, 10); // min_percent

    // Test with maximum allowed percentage
    let mut split = ResizableSplit::new(95);
    let area = Rect::new(0, 0, 100, 20);
    split.start_drag();
    split.update_from_mouse(90, 10, area);
    assert_eq!(split.split_percent, 90); // max_percent
}

// ============================================================================
// RIGHT_PERCENT / BOTTOM_PERCENT TESTS
// ============================================================================

#[test]
fn test_right_percent_calculates_correctly() {
    let split = ResizableSplit::new(70);
    assert_eq!(split.right_percent(), 30);

    let split = ResizableSplit::new(50);
    assert_eq!(split.right_percent(), 50);

    let split = ResizableSplit::new(25);
    assert_eq!(split.right_percent(), 75);
}

#[test]
fn test_bottom_percent_is_alias_for_right_percent() {
    let split = ResizableSplit::new_with_direction(70, SplitDirection::Horizontal);
    assert_eq!(split.bottom_percent(), 30);
    assert_eq!(split.bottom_percent(), split.right_percent());
}

// ============================================================================
// INTEGRATION TESTS - REALISTIC SCENARIOS
// ============================================================================

#[test]
fn test_integration_complete_drag_cycle_vertical() {
    let mut split = ResizableSplit::new(50);
    let area = Rect::new(0, 0, 100, 20);

    // Initial state
    assert_eq!(split.split_percent, 50);
    assert!(!split.is_dragging);

    // Update divider position
    split.update_divider_position(area);
    assert_eq!(split.divider_pos, 50);

    // Check if mouse is on divider
    assert!(split.is_on_divider(50, 10, area));

    // Start dragging
    split.start_drag();
    assert!(split.is_dragging);

    // Drag to new position
    split.update_from_mouse(70, 10, area);
    assert_eq!(split.split_percent, 70);

    // Update divider position again
    split.update_divider_position(area);
    assert_eq!(split.divider_pos, 70);

    // Stop dragging
    split.stop_drag();
    assert!(!split.is_dragging);

    // Try to update mouse position (should not work)
    split.update_from_mouse(80, 10, area);
    assert_eq!(split.split_percent, 70); // Should not change
}

#[test]
fn test_integration_complete_drag_cycle_horizontal() {
    let mut split = ResizableSplit::new_with_direction(50, SplitDirection::Horizontal);
    let area = Rect::new(0, 0, 100, 40);

    // Initial state
    assert_eq!(split.split_percent, 50);
    assert!(!split.is_dragging);

    // Update divider position
    split.update_divider_position(area);
    assert_eq!(split.divider_pos, 20); // 50% of 40 rows

    // Check if mouse is on divider
    assert!(split.is_on_divider(50, 20, area));

    // Start dragging
    split.start_drag();
    assert!(split.is_dragging);

    // Drag to new position (row 30 = 75%)
    split.update_from_mouse(50, 30, area);
    assert_eq!(split.split_percent, 75);

    // Update divider position again
    split.update_divider_position(area);
    assert_eq!(split.divider_pos, 30); // 75% of 40 rows

    // Stop dragging
    split.stop_drag();
    assert!(!split.is_dragging);
}

#[test]
fn test_integration_dragging_respects_bounds() {
    let mut split = ResizableSplit::new(50);
    let area = Rect::new(0, 0, 100, 20);

    split.start_drag();

    // Try to drag beyond max
    split.update_from_mouse(95, 10, area);
    assert_eq!(split.split_percent, 90);

    // Try to drag below min
    split.update_from_mouse(5, 10, area);
    assert_eq!(split.split_percent, 10);

    // Drag to valid position
    split.update_from_mouse(60, 10, area);
    assert_eq!(split.split_percent, 60);
}

#[test]
fn test_integration_divider_hit_area_at_boundaries() {
    let mut split = ResizableSplit::new(5);
    let area = Rect::new(0, 0, 20, 10);
    split.update_divider_position(area);

    // Divider at column 1 (5% of 20)
    // Hit area should be columns 0, 1, 2
    assert!(split.is_on_divider(0, 5, area));
    assert!(split.is_on_divider(1, 5, area));
    assert!(split.is_on_divider(2, 5, area));
    assert!(!split.is_on_divider(3, 5, area));
}
