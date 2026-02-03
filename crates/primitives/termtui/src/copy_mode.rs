//! Copy mode with frozen screen snapshot (mprocs pattern)
//!
//! When entering copy mode, we take a snapshot of the current screen state.
//! This allows stable text selection even while the terminal continues
//! receiving output in the background.

use crate::screen::Screen;

/// Cursor position in copy mode
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CopyPos {
    pub x: i32,
    pub y: i32,
}

impl CopyPos {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    /// Get the low and high positions for selection
    pub fn to_low_high(start: &CopyPos, end: &CopyPos) -> (CopyPos, CopyPos) {
        if start.y < end.y || (start.y == end.y && start.x <= end.x) {
            (*start, *end)
        } else {
            (*end, *start)
        }
    }
}

/// Movement direction for copy mode navigation
#[derive(Clone, Copy, Debug)]
pub enum CopyMoveDir {
    Up,
    Down,
    Left,
    Right,
    LineStart,
    LineEnd,
    PageUp,
    PageDown,
    Top,
    Bottom,
    WordLeft,
    WordRight,
}

impl CopyMoveDir {
    /// Get delta (dx, dy) for movement
    pub fn delta(&self) -> (i32, i32) {
        match self {
            CopyMoveDir::Up => (0, -1),
            CopyMoveDir::Down => (0, 1),
            CopyMoveDir::Left => (-1, 0),
            CopyMoveDir::Right => (1, 0),
            _ => (0, 0), // Special movements handled separately
        }
    }
}

/// Copy mode state
#[derive(Clone, Default)]
pub enum CopyMode {
    /// Not in copy mode
    #[default]
    None,
    /// Active copy mode with frozen screen
    Active {
        /// Frozen screen snapshot
        frozen_screen: Box<Screen>,
        /// Current cursor position
        cursor: CopyPos,
        /// Selection anchor (start of selection)
        anchor: Option<CopyPos>,
        /// Screen height for bounds
        screen_height: i32,
        /// Screen width for bounds
        screen_width: i32,
        /// Scrollback available
        scrollback_available: i32,
    },
}

impl CopyMode {
    /// Check if copy mode is active
    pub fn is_active(&self) -> bool {
        matches!(self, CopyMode::Active { .. })
    }

    /// Enter copy mode with a screen snapshot
    pub fn enter(screen: Screen, start: CopyPos) -> Self {
        let size = screen.size();
        let scrollback = screen.primary_grid().scrollback_available() as i32;

        CopyMode::Active {
            frozen_screen: Box::new(screen),
            cursor: start,
            anchor: None,
            screen_height: size.rows as i32,
            screen_width: size.cols as i32,
            scrollback_available: scrollback,
        }
    }

    /// Move cursor in copy mode
    pub fn move_cursor(&mut self, dx: i32, dy: i32) {
        if let CopyMode::Active {
            cursor,
            screen_width,
            screen_height,
            scrollback_available,
            ..
        } = self
        {
            let new_x = (cursor.x + dx).clamp(0, *screen_width - 1);
            let new_y = (cursor.y + dy).clamp(-*scrollback_available, *screen_height - 1);

            cursor.x = new_x;
            cursor.y = new_y;
        }
    }

    /// Move cursor by direction
    pub fn move_dir(&mut self, dir: CopyMoveDir) {
        if let CopyMode::Active {
            cursor,
            screen_width,
            screen_height,
            scrollback_available,
            ..
        } = self
        {
            match dir {
                CopyMoveDir::Up | CopyMoveDir::Down | CopyMoveDir::Left | CopyMoveDir::Right => {
                    let (dx, dy) = dir.delta();
                    self.move_cursor(dx, dy);
                }
                CopyMoveDir::LineStart => {
                    cursor.x = 0;
                }
                CopyMoveDir::LineEnd => {
                    cursor.x = *screen_width - 1;
                }
                CopyMoveDir::PageUp => {
                    let page = *screen_height / 2;
                    cursor.y = (cursor.y - page).max(-*scrollback_available);
                }
                CopyMoveDir::PageDown => {
                    let page = *screen_height / 2;
                    cursor.y = (cursor.y + page).min(*screen_height - 1);
                }
                CopyMoveDir::Top => {
                    cursor.y = -*scrollback_available;
                    cursor.x = 0;
                }
                CopyMoveDir::Bottom => {
                    cursor.y = *screen_height - 1;
                    cursor.x = *screen_width - 1;
                }
                CopyMoveDir::WordLeft | CopyMoveDir::WordRight => {
                    // Simplified: just move by 5 characters
                    let delta = if matches!(dir, CopyMoveDir::WordLeft) {
                        -5
                    } else {
                        5
                    };
                    cursor.x = (cursor.x + delta).clamp(0, *screen_width - 1);
                }
            }
        }
    }

    /// Set selection anchor at current cursor position
    pub fn set_anchor(&mut self) {
        if let CopyMode::Active { cursor, anchor, .. } = self {
            if anchor.is_some() {
                // Toggle anchor off
                *anchor = None;
            } else {
                // Set anchor at current position
                *anchor = Some(*cursor);
            }
        }
    }

    /// Set end position (for mouse selection)
    pub fn set_end(&mut self) {
        if let CopyMode::Active { cursor, anchor, .. } = self {
            if anchor.is_none() {
                // If no anchor, set it first
                *anchor = Some(*cursor);
            }
        }
    }

    /// Get the current selection range
    pub fn get_selection(&self) -> Option<(CopyPos, CopyPos)> {
        if let CopyMode::Active { cursor, anchor, .. } = self {
            anchor.map(|a| (a, *cursor))
        } else {
            None
        }
    }

    /// Get selected text from frozen screen
    pub fn get_selected_text(&self) -> Option<String> {
        if let CopyMode::Active {
            frozen_screen,
            cursor,
            anchor,
            ..
        } = self
        {
            let anchor = (*anchor)?;
            let (low, high) = CopyPos::to_low_high(&anchor, cursor);

            Some(frozen_screen.get_selected_text(low.x, low.y, high.x, high.y))
        } else {
            None
        }
    }

    /// Get the frozen screen (for rendering in copy mode)
    pub fn frozen_screen(&self) -> Option<&Screen> {
        if let CopyMode::Active { frozen_screen, .. } = self {
            Some(frozen_screen)
        } else {
            None
        }
    }

    /// Get cursor position in copy mode
    pub fn cursor(&self) -> Option<CopyPos> {
        if let CopyMode::Active { cursor, .. } = self {
            Some(*cursor)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_test_screen() -> Screen {
        Screen::new(24, 80, 1000)
    }

    #[test]
    fn test_copy_mode_enter() {
        let screen = make_test_screen();
        let mode = CopyMode::enter(screen, CopyPos::new(0, 0));

        assert!(mode.is_active());
    }

    #[test]
    fn test_copy_mode_move_cursor() {
        let screen = make_test_screen();
        let mut mode = CopyMode::enter(screen, CopyPos::new(10, 10));

        mode.move_cursor(5, 2);

        if let CopyMode::Active { cursor, .. } = mode {
            assert_eq!(cursor.x, 15);
            assert_eq!(cursor.y, 12);
        } else {
            panic!("Expected active mode");
        }
    }

    #[test]
    fn test_copy_mode_bounds() {
        let screen = make_test_screen();
        let mut mode = CopyMode::enter(screen, CopyPos::new(0, 0));

        // Try to move out of bounds
        mode.move_cursor(-10, -10);

        if let CopyMode::Active { cursor, .. } = mode {
            assert_eq!(cursor.x, 0);
            assert!(cursor.y <= 0);
        } else {
            panic!("Expected active mode");
        }
    }

    #[test]
    fn test_copy_mode_selection() {
        let screen = make_test_screen();
        let mut mode = CopyMode::enter(screen, CopyPos::new(5, 5));

        // No selection initially
        assert!(mode.get_selection().is_none());

        // Set anchor
        mode.set_anchor();

        // Move cursor
        mode.move_cursor(10, 0);

        // Now we have selection
        let selection = mode.get_selection();
        assert!(selection.is_some());

        let (start, end) = selection.unwrap();
        assert_eq!(start.x, 5);
        assert_eq!(end.x, 15);
    }

    #[test]
    fn test_copy_pos_to_low_high() {
        let a = CopyPos::new(10, 5);
        let b = CopyPos::new(5, 10);

        let (low, high) = CopyPos::to_low_high(&a, &b);
        assert_eq!(low.y, 5);
        assert_eq!(high.y, 10);
    }
}
