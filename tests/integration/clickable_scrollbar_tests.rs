//! Comprehensive unit tests for ClickableScrollbar component
//!
//! Tests cover:
//! - State initialization and management
//! - Mouse event handling (clicks, drags, wheel)
//! - ScrollbarEvent generation
//! - Position calculation from mouse coordinates
//! - Edge cases and boundary conditions
//! - Both vertical and horizontal orientations

use crossterm::event::{MouseButton, MouseEvent, MouseEventKind};
use ratatui::layout::Rect;
use ratatui::widgets::ScrollbarOrientation;
use ratatui_toolkit::clickable_scrollbar::{ClickableScrollbarState, ScrollbarEvent};

// ============================================================================
// State Initialization Tests
// ============================================================================

#[test]
fn test_state_new_default_values() {
    let state = ClickableScrollbarState::new();

    assert_eq!(state.offset, 0);
    assert_eq!(state.page_len, 0);
    assert_eq!(state.max_offset, 0);
    assert_eq!(state.area, Rect::default());
    assert!(state.scroll_by.is_none());
}

#[test]
fn test_state_default_trait() {
    let state = ClickableScrollbarState::default();

    assert_eq!(state.offset, 0);
    assert_eq!(state.page_len, 0);
    assert_eq!(state.max_offset, 0);
}

#[test]
fn test_set_content_calculates_max_offset() {
    let state = ClickableScrollbarState::new().set_content(100, 20);

    assert_eq!(state.page_len, 20);
    assert_eq!(state.max_offset, 80); // 100 - 20
}

#[test]
fn test_set_content_handles_content_smaller_than_page() {
    let state = ClickableScrollbarState::new().set_content(10, 20);

    assert_eq!(state.page_len, 20);
    assert_eq!(state.max_offset, 0); // saturating_sub(20) = 0
}

#[test]
fn test_position_clamps_to_max_offset() {
    let state = ClickableScrollbarState::new()
        .set_content(100, 20)
        .position(150);

    assert_eq!(state.offset, 80); // Clamped to max_offset
}

#[test]
fn test_position_within_bounds() {
    let state = ClickableScrollbarState::new()
        .set_content(100, 20)
        .position(50);

    assert_eq!(state.offset, 50);
}

// ============================================================================
// Offset Management Tests
// ============================================================================

#[test]
fn test_get_offset() {
    let state = ClickableScrollbarState::new()
        .set_content(100, 20)
        .position(30);

    assert_eq!(state.offset(), 30);
}

#[test]
fn test_set_offset_returns_true_when_changed() {
    let mut state = ClickableScrollbarState::new().set_content(100, 20);

    let changed = state.set_offset(30);

    assert!(changed);
    assert_eq!(state.offset, 30);
}

#[test]
fn test_set_offset_returns_false_when_unchanged() {
    let mut state = ClickableScrollbarState::new()
        .set_content(100, 20)
        .position(30);

    let changed = state.set_offset(30);

    assert!(!changed);
    assert_eq!(state.offset, 30);
}

#[test]
fn test_set_offset_clamps_to_max_offset() {
    let mut state = ClickableScrollbarState::new().set_content(100, 20);

    state.set_offset(150);

    assert_eq!(state.offset, 80); // Clamped to max_offset
}

#[test]
fn test_scroll_up_decreases_offset() {
    let mut state = ClickableScrollbarState::new()
        .set_content(100, 20)
        .position(50);

    let changed = state.scroll_up(10);

    assert!(changed);
    assert_eq!(state.offset, 40);
}

#[test]
fn test_scroll_up_saturates_at_zero() {
    let mut state = ClickableScrollbarState::new()
        .set_content(100, 20)
        .position(5);

    let changed = state.scroll_up(10);

    assert!(changed);
    assert_eq!(state.offset, 0);
}

#[test]
fn test_scroll_up_returns_false_at_zero() {
    let mut state = ClickableScrollbarState::new()
        .set_content(100, 20)
        .position(0);

    let changed = state.scroll_up(10);

    assert!(!changed);
    assert_eq!(state.offset, 0);
}

#[test]
fn test_scroll_down_increases_offset() {
    let mut state = ClickableScrollbarState::new()
        .set_content(100, 20)
        .position(40);

    let changed = state.scroll_down(10);

    assert!(changed);
    assert_eq!(state.offset, 50);
}

#[test]
fn test_scroll_down_clamps_at_max_offset() {
    let mut state = ClickableScrollbarState::new()
        .set_content(100, 20)
        .position(75);

    let changed = state.scroll_down(10);

    assert!(changed);
    assert_eq!(state.offset, 80); // Clamped to max_offset
}

#[test]
fn test_scroll_down_returns_false_at_max() {
    let mut state = ClickableScrollbarState::new()
        .set_content(100, 20)
        .position(80);

    let changed = state.scroll_down(10);

    assert!(!changed);
    assert_eq!(state.offset, 80);
}

// ============================================================================
// Scroll Increment Tests
// ============================================================================

#[test]
fn test_scroll_increment_default_is_tenth_of_page() {
    let state = ClickableScrollbarState::new().set_content(100, 50);

    assert_eq!(state.scroll_increment(), 5); // 50 / 10
}

#[test]
fn test_scroll_increment_minimum_is_one() {
    let state = ClickableScrollbarState::new().set_content(100, 5);

    assert_eq!(state.scroll_increment(), 1); // max(0, 1) = 1
}

#[test]
fn test_scroll_increment_custom_value() {
    let mut state = ClickableScrollbarState::new().set_content(100, 50);
    state.scroll_by = Some(3);

    assert_eq!(state.scroll_increment(), 3);
}

// ============================================================================
// Mouse Event Handling - Outside Area
// ============================================================================

#[test]
fn test_mouse_outside_scrollbar_returns_none() {
    let mut state = ClickableScrollbarState::new().set_content(100, 20);
    state.area = Rect::new(10, 5, 2, 20);
    state.orientation = ScrollbarOrientation::VerticalRight;

    let event = MouseEvent {
        kind: MouseEventKind::Down(MouseButton::Left),
        column: 0, // Outside scrollbar area
        row: 10,
        modifiers: crossterm::event::KeyModifiers::empty(),
    };

    let result = state.handle_mouse_event(&event);

    assert_eq!(result, ScrollbarEvent::None);
}

#[test]
fn test_mouse_outside_releases_drag() {
    let mut state = ClickableScrollbarState::new().set_content(100, 20);
    state.area = Rect::new(10, 5, 2, 20);
    state.orientation = ScrollbarOrientation::VerticalRight;

    // Start drag inside
    let start_drag = MouseEvent {
        kind: MouseEventKind::Down(MouseButton::Left),
        column: 10,
        row: 10,
        modifiers: crossterm::event::KeyModifiers::empty(),
    };
    state.handle_mouse_event(&start_drag);

    // Drag outside scrollbar area
    let event = MouseEvent {
        kind: MouseEventKind::Drag(MouseButton::Left),
        column: 0, // Outside scrollbar area
        row: 10,
        modifiers: crossterm::event::KeyModifiers::empty(),
    };

    let result = state.handle_mouse_event(&event);

    // Should return None since we're outside
    assert_eq!(result, ScrollbarEvent::None);

    // Verify drag is released by trying to drag again inside - should return None
    let drag_again = MouseEvent {
        kind: MouseEventKind::Drag(MouseButton::Left),
        column: 10,
        row: 15,
        modifiers: crossterm::event::KeyModifiers::empty(),
    };
    let result2 = state.handle_mouse_event(&drag_again);
    assert_eq!(result2, ScrollbarEvent::None);
}

// ============================================================================
// Mouse Event Handling - Vertical Scrollbar
// ============================================================================

#[test]
fn test_vertical_scroll_wheel_down() {
    let mut state = ClickableScrollbarState::new().set_content(100, 20);
    state.area = Rect::new(10, 5, 2, 20);
    state.orientation = ScrollbarOrientation::VerticalRight;

    let event = MouseEvent {
        kind: MouseEventKind::ScrollDown,
        column: 10,
        row: 10,
        modifiers: crossterm::event::KeyModifiers::empty(),
    };

    let result = state.handle_mouse_event(&event);

    assert_eq!(result, ScrollbarEvent::Down(2)); // scroll_increment() = 20/10 = 2
}

#[test]
fn test_vertical_scroll_wheel_up() {
    let mut state = ClickableScrollbarState::new().set_content(100, 20);
    state.area = Rect::new(10, 5, 2, 20);
    state.orientation = ScrollbarOrientation::VerticalRight;

    let event = MouseEvent {
        kind: MouseEventKind::ScrollUp,
        column: 10,
        row: 10,
        modifiers: crossterm::event::KeyModifiers::empty(),
    };

    let result = state.handle_mouse_event(&event);

    assert_eq!(result, ScrollbarEvent::Up(2)); // scroll_increment() = 20/10 = 2
}

#[test]
fn test_vertical_left_click_activates_drag() {
    let mut state = ClickableScrollbarState::new().set_content(100, 20);
    state.area = Rect::new(10, 5, 2, 20);
    state.orientation = ScrollbarOrientation::VerticalRight;

    let event = MouseEvent {
        kind: MouseEventKind::Down(MouseButton::Left),
        column: 10,
        row: 15, // Middle of scrollbar
        modifiers: crossterm::event::KeyModifiers::empty(),
    };

    let result = state.handle_mouse_event(&event);

    // Position calculation: (row - area.y - 1) = (15 - 5 - 1) = 9
    // span = area.height - 2 = 20 - 2 = 18
    // offset = (80 * 9) / 18 = 40
    assert!(matches!(result, ScrollbarEvent::Position(_)));

    // Verify drag is active by checking that subsequent drag events work
    let drag_event = MouseEvent {
        kind: MouseEventKind::Drag(MouseButton::Left),
        column: 10,
        row: 20,
        modifiers: crossterm::event::KeyModifiers::empty(),
    };
    let drag_result = state.handle_mouse_event(&drag_event);
    assert!(matches!(drag_result, ScrollbarEvent::Position(_)));
}

#[test]
fn test_vertical_drag_updates_position() {
    let mut state = ClickableScrollbarState::new().set_content(100, 20);
    state.area = Rect::new(10, 5, 2, 20);
    state.orientation = ScrollbarOrientation::VerticalRight;

    // Activate drag first
    let click = MouseEvent {
        kind: MouseEventKind::Down(MouseButton::Left),
        column: 10,
        row: 10,
        modifiers: crossterm::event::KeyModifiers::empty(),
    };
    state.handle_mouse_event(&click);

    let event = MouseEvent {
        kind: MouseEventKind::Drag(MouseButton::Left),
        column: 10,
        row: 20, // Near bottom
        modifiers: crossterm::event::KeyModifiers::empty(),
    };

    let result = state.handle_mouse_event(&event);

    // Position calculation: (row - area.y - 1) = (20 - 5 - 1) = 14
    // span = area.height - 2 = 20 - 2 = 18
    // offset = (80 * 14) / 18 = 62
    if let ScrollbarEvent::Position(pos) = result {
        assert_eq!(pos, 62);
    } else {
        panic!("Expected Position event");
    }
}

#[test]
fn test_vertical_drag_without_active_flag_ignored() {
    let mut state = ClickableScrollbarState::new().set_content(100, 20);
    state.area = Rect::new(10, 5, 2, 20);
    state.orientation = ScrollbarOrientation::VerticalRight;
    // Don't activate drag - just try to drag directly

    let event = MouseEvent {
        kind: MouseEventKind::Drag(MouseButton::Left),
        column: 10,
        row: 15,
        modifiers: crossterm::event::KeyModifiers::empty(),
    };

    let result = state.handle_mouse_event(&event);

    assert_eq!(result, ScrollbarEvent::None);
}

#[test]
fn test_vertical_mouse_up_releases_drag() {
    let mut state = ClickableScrollbarState::new().set_content(100, 20);
    state.area = Rect::new(10, 5, 2, 20);
    state.orientation = ScrollbarOrientation::VerticalRight;

    // Start drag first
    let click = MouseEvent {
        kind: MouseEventKind::Down(MouseButton::Left),
        column: 10,
        row: 10,
        modifiers: crossterm::event::KeyModifiers::empty(),
    };
    state.handle_mouse_event(&click);

    let event = MouseEvent {
        kind: MouseEventKind::Up(MouseButton::Left),
        column: 10,
        row: 15,
        modifiers: crossterm::event::KeyModifiers::empty(),
    };

    let result = state.handle_mouse_event(&event);

    assert_eq!(result, ScrollbarEvent::None);

    // Verify drag is released by trying to drag - should return None
    let drag_after = MouseEvent {
        kind: MouseEventKind::Drag(MouseButton::Left),
        column: 10,
        row: 20,
        modifiers: crossterm::event::KeyModifiers::empty(),
    };
    let drag_result = state.handle_mouse_event(&drag_after);
    assert_eq!(drag_result, ScrollbarEvent::None);
}

#[test]
fn test_vertical_right_click_ignored() {
    let mut state = ClickableScrollbarState::new().set_content(100, 20);
    state.area = Rect::new(10, 5, 2, 20);
    state.orientation = ScrollbarOrientation::VerticalRight;

    let event = MouseEvent {
        kind: MouseEventKind::Down(MouseButton::Right),
        column: 10,
        row: 15,
        modifiers: crossterm::event::KeyModifiers::empty(),
    };

    let result = state.handle_mouse_event(&event);

    assert_eq!(result, ScrollbarEvent::None);

    // Verify drag was not activated by trying to drag - should return None
    let drag = MouseEvent {
        kind: MouseEventKind::Drag(MouseButton::Left),
        column: 10,
        row: 20,
        modifiers: crossterm::event::KeyModifiers::empty(),
    };
    let drag_result = state.handle_mouse_event(&drag);
    assert_eq!(drag_result, ScrollbarEvent::None);
}

// ============================================================================
// Mouse Event Handling - Horizontal Scrollbar
// ============================================================================

#[test]
fn test_horizontal_scroll_wheel_ignored() {
    let mut state = ClickableScrollbarState::new().set_content(100, 20);
    state.area = Rect::new(5, 20, 40, 2);
    state.orientation = ScrollbarOrientation::HorizontalBottom;

    let event_down = MouseEvent {
        kind: MouseEventKind::ScrollDown,
        column: 20,
        row: 20,
        modifiers: crossterm::event::KeyModifiers::empty(),
    };

    let result_down = state.handle_mouse_event(&event_down);
    assert_eq!(result_down, ScrollbarEvent::None);

    let event_up = MouseEvent {
        kind: MouseEventKind::ScrollUp,
        column: 20,
        row: 20,
        modifiers: crossterm::event::KeyModifiers::empty(),
    };

    let result_up = state.handle_mouse_event(&event_up);
    assert_eq!(result_up, ScrollbarEvent::None);
}

#[test]
fn test_horizontal_left_click_jumps_to_position() {
    let mut state = ClickableScrollbarState::new().set_content(200, 40);
    state.area = Rect::new(5, 20, 40, 2);
    state.orientation = ScrollbarOrientation::HorizontalBottom;

    let event = MouseEvent {
        kind: MouseEventKind::Down(MouseButton::Left),
        column: 25, // Middle of scrollbar
        row: 20,
        modifiers: crossterm::event::KeyModifiers::empty(),
    };

    let result = state.handle_mouse_event(&event);

    // Position calculation: (col - area.x - 1) = (25 - 5 - 1) = 19
    // span = area.width - 2 = 40 - 2 = 38
    // offset = (160 * 19) / 38 = 80
    if let ScrollbarEvent::Position(pos) = result {
        assert_eq!(pos, 80);
    } else {
        panic!("Expected Position event");
    }

    // Verify drag is active by checking that subsequent drag events work
    let drag_event = MouseEvent {
        kind: MouseEventKind::Drag(MouseButton::Left),
        column: 30,
        row: 20,
        modifiers: crossterm::event::KeyModifiers::empty(),
    };
    let drag_result = state.handle_mouse_event(&drag_event);
    assert!(matches!(drag_result, ScrollbarEvent::Position(_)));
}

#[test]
fn test_horizontal_drag_updates_position() {
    let mut state = ClickableScrollbarState::new().set_content(200, 40);
    state.area = Rect::new(5, 20, 40, 2);
    state.orientation = ScrollbarOrientation::HorizontalBottom;

    // Activate drag first
    let click = MouseEvent {
        kind: MouseEventKind::Down(MouseButton::Left),
        column: 20,
        row: 20,
        modifiers: crossterm::event::KeyModifiers::empty(),
    };
    state.handle_mouse_event(&click);

    let event = MouseEvent {
        kind: MouseEventKind::Drag(MouseButton::Left),
        column: 40, // Near end
        row: 20,
        modifiers: crossterm::event::KeyModifiers::empty(),
    };

    let result = state.handle_mouse_event(&event);

    // Position calculation: (col - area.x - 1) = (40 - 5 - 1) = 34
    // span = area.width - 2 = 40 - 2 = 38
    // offset = (160 * 34) / 38 = 143
    if let ScrollbarEvent::Position(pos) = result {
        assert_eq!(pos, 143);
    } else {
        panic!("Expected Position event");
    }
}

// ============================================================================
// Position Mapping Edge Cases - Vertical
// ============================================================================

#[test]
fn test_vertical_position_at_top_arrow() {
    let mut state = ClickableScrollbarState::new().set_content(100, 20);
    state.area = Rect::new(10, 5, 2, 20);
    state.orientation = ScrollbarOrientation::VerticalRight;

    let event = MouseEvent {
        kind: MouseEventKind::Down(MouseButton::Left),
        column: 10,
        row: 5, // At the top (first row)
        modifiers: crossterm::event::KeyModifiers::empty(),
    };

    let result = state.handle_mouse_event(&event);

    // Position calculation: (row - area.y - 1) = (5 - 5 - 1) = -1 -> saturates to 0
    // span = area.height - 2 = 20 - 2 = 18
    // offset = (80 * 0) / 18 = 0
    if let ScrollbarEvent::Position(pos) = result {
        assert_eq!(pos, 0);
    } else {
        panic!("Expected Position event");
    }
}

#[test]
fn test_vertical_position_at_second_row() {
    let mut state = ClickableScrollbarState::new().set_content(100, 20);
    state.area = Rect::new(10, 5, 2, 20);
    state.orientation = ScrollbarOrientation::VerticalRight;

    let event = MouseEvent {
        kind: MouseEventKind::Down(MouseButton::Left),
        column: 10,
        row: 6, // Second row (just after top arrow)
        modifiers: crossterm::event::KeyModifiers::empty(),
    };

    let result = state.handle_mouse_event(&event);

    // Position calculation: (row - area.y - 1) = (6 - 5 - 1) = 0
    // span = area.height - 2 = 20 - 2 = 18
    // offset = (80 * 0) / 18 = 0
    if let ScrollbarEvent::Position(pos) = result {
        assert_eq!(pos, 0);
    } else {
        panic!("Expected Position event");
    }
}

#[test]
fn test_vertical_position_at_bottom_arrow() {
    let mut state = ClickableScrollbarState::new().set_content(100, 20);
    state.area = Rect::new(10, 5, 2, 20);
    state.orientation = ScrollbarOrientation::VerticalRight;

    let event = MouseEvent {
        kind: MouseEventKind::Down(MouseButton::Left),
        column: 10,
        row: 24, // At the bottom (last row)
        modifiers: crossterm::event::KeyModifiers::empty(),
    };

    let result = state.handle_mouse_event(&event);

    // Position calculation: (row - area.y - 1) = (24 - 5 - 1) = 18
    // span = area.height - 2 = 20 - 2 = 18
    // offset = (80 * 18) / 18 = 80 (max_offset)
    if let ScrollbarEvent::Position(pos) = result {
        assert_eq!(pos, 80);
    } else {
        panic!("Expected Position event");
    }
}

#[test]
fn test_vertical_position_empty_span() {
    let mut state = ClickableScrollbarState::new().set_content(100, 20);
    state.area = Rect::new(10, 5, 2, 2); // Height of 2 (arrows only, no track)
    state.orientation = ScrollbarOrientation::VerticalRight;

    let event = MouseEvent {
        kind: MouseEventKind::Down(MouseButton::Left),
        column: 10,
        row: 6,
        modifiers: crossterm::event::KeyModifiers::empty(),
    };

    let result = state.handle_mouse_event(&event);

    // span = area.height - 2 = 2 - 2 = 0
    // When span is 0, should return 0
    if let ScrollbarEvent::Position(pos) = result {
        assert_eq!(pos, 0);
    } else {
        panic!("Expected Position event");
    }
}

// ============================================================================
// Position Mapping Edge Cases - Horizontal
// ============================================================================

#[test]
fn test_horizontal_position_at_left_arrow() {
    let mut state = ClickableScrollbarState::new().set_content(200, 40);
    state.area = Rect::new(5, 20, 40, 2);
    state.orientation = ScrollbarOrientation::HorizontalBottom;

    let event = MouseEvent {
        kind: MouseEventKind::Down(MouseButton::Left),
        column: 5, // At the left (first column)
        row: 20,
        modifiers: crossterm::event::KeyModifiers::empty(),
    };

    let result = state.handle_mouse_event(&event);

    // Position calculation: (col - area.x - 1) = (5 - 5 - 1) = -1 -> saturates to 0
    // span = area.width - 2 = 40 - 2 = 38
    // offset = (160 * 0) / 38 = 0
    if let ScrollbarEvent::Position(pos) = result {
        assert_eq!(pos, 0);
    } else {
        panic!("Expected Position event");
    }
}

#[test]
fn test_horizontal_position_at_second_column() {
    let mut state = ClickableScrollbarState::new().set_content(200, 40);
    state.area = Rect::new(5, 20, 40, 2);
    state.orientation = ScrollbarOrientation::HorizontalBottom;

    let event = MouseEvent {
        kind: MouseEventKind::Down(MouseButton::Left),
        column: 6, // Second column (just after left arrow)
        row: 20,
        modifiers: crossterm::event::KeyModifiers::empty(),
    };

    let result = state.handle_mouse_event(&event);

    // Position calculation: (col - area.x - 1) = (6 - 5 - 1) = 0
    // span = area.width - 2 = 40 - 2 = 38
    // offset = (160 * 0) / 38 = 0
    if let ScrollbarEvent::Position(pos) = result {
        assert_eq!(pos, 0);
    } else {
        panic!("Expected Position event");
    }
}

#[test]
fn test_horizontal_position_at_right_arrow() {
    let mut state = ClickableScrollbarState::new().set_content(200, 40);
    state.area = Rect::new(5, 20, 40, 2);
    state.orientation = ScrollbarOrientation::HorizontalBottom;

    let event = MouseEvent {
        kind: MouseEventKind::Down(MouseButton::Left),
        column: 44, // At the right (last column)
        row: 20,
        modifiers: crossterm::event::KeyModifiers::empty(),
    };

    let result = state.handle_mouse_event(&event);

    // Position calculation: (col - area.x - 1) = (44 - 5 - 1) = 38
    // span = area.width - 2 = 40 - 2 = 38
    // offset = (160 * 38) / 38 = 160 (max_offset)
    if let ScrollbarEvent::Position(pos) = result {
        assert_eq!(pos, 160);
    } else {
        panic!("Expected Position event");
    }
}

#[test]
fn test_horizontal_position_empty_span() {
    let mut state = ClickableScrollbarState::new().set_content(200, 40);
    state.area = Rect::new(5, 20, 2, 2); // Width of 2 (arrows only, no track)
    state.orientation = ScrollbarOrientation::HorizontalBottom;

    let event = MouseEvent {
        kind: MouseEventKind::Down(MouseButton::Left),
        column: 6,
        row: 20,
        modifiers: crossterm::event::KeyModifiers::empty(),
    };

    let result = state.handle_mouse_event(&event);

    // span = area.width - 2 = 2 - 2 = 0
    // When span is 0, should return 0
    if let ScrollbarEvent::Position(pos) = result {
        assert_eq!(pos, 0);
    } else {
        panic!("Expected Position event");
    }
}

// ============================================================================
// Complex Interaction Scenarios
// ============================================================================

#[test]
fn test_click_drag_release_sequence() {
    let mut state = ClickableScrollbarState::new().set_content(100, 20);
    state.area = Rect::new(10, 5, 2, 20);
    state.orientation = ScrollbarOrientation::VerticalRight;

    // 1. Click to start drag
    let click = MouseEvent {
        kind: MouseEventKind::Down(MouseButton::Left),
        column: 10,
        row: 10,
        modifiers: crossterm::event::KeyModifiers::empty(),
    };
    let result1 = state.handle_mouse_event(&click);
    assert!(matches!(result1, ScrollbarEvent::Position(_)));

    // 2. Drag to new position - should work
    let drag = MouseEvent {
        kind: MouseEventKind::Drag(MouseButton::Left),
        column: 10,
        row: 15,
        modifiers: crossterm::event::KeyModifiers::empty(),
    };
    let result2 = state.handle_mouse_event(&drag);
    assert!(matches!(result2, ScrollbarEvent::Position(_)));

    // 3. Release mouse button
    let release = MouseEvent {
        kind: MouseEventKind::Up(MouseButton::Left),
        column: 10,
        row: 15,
        modifiers: crossterm::event::KeyModifiers::empty(),
    };
    let result3 = state.handle_mouse_event(&release);
    assert_eq!(result3, ScrollbarEvent::None);

    // 4. Further drags should be ignored
    let drag_after_release = MouseEvent {
        kind: MouseEventKind::Drag(MouseButton::Left),
        column: 10,
        row: 20,
        modifiers: crossterm::event::KeyModifiers::empty(),
    };
    let result4 = state.handle_mouse_event(&drag_after_release);
    assert_eq!(result4, ScrollbarEvent::None);
}

#[test]
fn test_drag_outside_area_cancels_drag() {
    let mut state = ClickableScrollbarState::new().set_content(100, 20);
    state.area = Rect::new(10, 5, 2, 20);
    state.orientation = ScrollbarOrientation::VerticalRight;

    // Start drag
    let click = MouseEvent {
        kind: MouseEventKind::Down(MouseButton::Left),
        column: 10,
        row: 10,
        modifiers: crossterm::event::KeyModifiers::empty(),
    };
    let result1 = state.handle_mouse_event(&click);
    assert!(matches!(result1, ScrollbarEvent::Position(_)));

    // Drag outside area
    let drag_outside = MouseEvent {
        kind: MouseEventKind::Drag(MouseButton::Left),
        column: 0, // Outside scrollbar area
        row: 10,
        modifiers: crossterm::event::KeyModifiers::empty(),
    };
    let result2 = state.handle_mouse_event(&drag_outside);
    assert_eq!(result2, ScrollbarEvent::None);

    // Verify drag is cancelled by trying to drag inside again - should return None
    let drag_inside = MouseEvent {
        kind: MouseEventKind::Drag(MouseButton::Left),
        column: 10,
        row: 15,
        modifiers: crossterm::event::KeyModifiers::empty(),
    };
    let result3 = state.handle_mouse_event(&drag_inside);
    assert_eq!(result3, ScrollbarEvent::None);
}

#[test]
fn test_scrollbar_with_zero_max_offset() {
    let mut state = ClickableScrollbarState::new().set_content(20, 20);
    state.area = Rect::new(10, 5, 2, 20);
    state.orientation = ScrollbarOrientation::VerticalRight;

    // Content fits entirely, max_offset is 0
    assert_eq!(state.max_offset, 0);

    // Click should still work but always return 0
    let click = MouseEvent {
        kind: MouseEventKind::Down(MouseButton::Left),
        column: 10,
        row: 15,
        modifiers: crossterm::event::KeyModifiers::empty(),
    };
    let result = state.handle_mouse_event(&click);

    if let ScrollbarEvent::Position(pos) = result {
        assert_eq!(pos, 0);
    } else {
        panic!("Expected Position event");
    }

    // Scroll down should return Down event
    let scroll = MouseEvent {
        kind: MouseEventKind::ScrollDown,
        column: 10,
        row: 10,
        modifiers: crossterm::event::KeyModifiers::empty(),
    };
    let result = state.handle_mouse_event(&scroll);
    assert_eq!(result, ScrollbarEvent::Down(2)); // Still generates event
}

// ============================================================================
// Orientation Tests
// ============================================================================

#[test]
fn test_vertical_left_orientation() {
    let mut state = ClickableScrollbarState::new().set_content(100, 20);
    state.area = Rect::new(0, 5, 2, 20);
    state.orientation = ScrollbarOrientation::VerticalLeft;

    // Should handle scroll wheel
    let scroll = MouseEvent {
        kind: MouseEventKind::ScrollDown,
        column: 0,
        row: 10,
        modifiers: crossterm::event::KeyModifiers::empty(),
    };
    let result = state.handle_mouse_event(&scroll);
    assert_eq!(result, ScrollbarEvent::Down(2));
}

#[test]
fn test_horizontal_top_orientation() {
    let mut state = ClickableScrollbarState::new().set_content(200, 40);
    state.area = Rect::new(5, 0, 40, 2);
    state.orientation = ScrollbarOrientation::HorizontalTop;

    // Should ignore scroll wheel
    let scroll = MouseEvent {
        kind: MouseEventKind::ScrollDown,
        column: 20,
        row: 0,
        modifiers: crossterm::event::KeyModifiers::empty(),
    };
    let result = state.handle_mouse_event(&scroll);
    assert_eq!(result, ScrollbarEvent::None);

    // But should handle clicks
    let click = MouseEvent {
        kind: MouseEventKind::Down(MouseButton::Left),
        column: 20,
        row: 0,
        modifiers: crossterm::event::KeyModifiers::empty(),
    };
    let result = state.handle_mouse_event(&click);
    assert!(matches!(result, ScrollbarEvent::Position(_)));
}

// ============================================================================
// ScrollbarEvent Equality Tests
// ============================================================================

#[test]
fn test_scrollbar_event_equality() {
    assert_eq!(ScrollbarEvent::None, ScrollbarEvent::None);
    assert_eq!(ScrollbarEvent::Up(5), ScrollbarEvent::Up(5));
    assert_eq!(ScrollbarEvent::Down(3), ScrollbarEvent::Down(3));
    assert_eq!(ScrollbarEvent::Position(42), ScrollbarEvent::Position(42));

    assert_ne!(ScrollbarEvent::Up(5), ScrollbarEvent::Up(3));
    assert_ne!(ScrollbarEvent::Down(5), ScrollbarEvent::Down(3));
    assert_ne!(ScrollbarEvent::Position(42), ScrollbarEvent::Position(24));
    assert_ne!(ScrollbarEvent::None, ScrollbarEvent::Up(1));
}

// ============================================================================
// Precision Tests
// ============================================================================

#[test]
fn test_position_mapping_precision_vertical() {
    let mut state = ClickableScrollbarState::new().set_content(1000, 100);
    state.area = Rect::new(10, 0, 2, 100);
    state.orientation = ScrollbarOrientation::VerticalRight;

    // Click at 25% down the scrollbar
    let click = MouseEvent {
        kind: MouseEventKind::Down(MouseButton::Left),
        column: 10,
        row: 25,
        modifiers: crossterm::event::KeyModifiers::empty(),
    };
    let result = state.handle_mouse_event(&click);

    // Position calculation: (row - area.y - 1) = (25 - 0 - 1) = 24
    // span = area.height - 2 = 100 - 2 = 98
    // offset = (900 * 24) / 98 = 220
    if let ScrollbarEvent::Position(pos) = result {
        assert_eq!(pos, 220);
    } else {
        panic!("Expected Position event");
    }
}

#[test]
fn test_position_mapping_precision_horizontal() {
    let mut state = ClickableScrollbarState::new().set_content(1000, 100);
    state.area = Rect::new(0, 10, 100, 2);
    state.orientation = ScrollbarOrientation::HorizontalBottom;

    // Click at 75% across the scrollbar
    let click = MouseEvent {
        kind: MouseEventKind::Down(MouseButton::Left),
        column: 75,
        row: 10,
        modifiers: crossterm::event::KeyModifiers::empty(),
    };
    let result = state.handle_mouse_event(&click);

    // Position calculation: (col - area.x - 1) = (75 - 0 - 1) = 74
    // span = area.width - 2 = 100 - 2 = 98
    // offset = (900 * 74) / 98 = 66600 / 98 = 679 (integer division)
    if let ScrollbarEvent::Position(pos) = result {
        assert_eq!(pos, 679);
    } else {
        panic!("Expected Position event");
    }
}
