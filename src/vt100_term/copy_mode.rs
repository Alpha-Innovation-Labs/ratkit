//! Copy mode implementation with frozen screen snapshots
//!
//! This implements mprocs-style copy mode where entering copy mode
//! freezes the terminal screen, allowing stable selection even as
//! new output arrives.

use super::Screen;

/// Position in terminal (can be negative for scrollback)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Pos {
    pub x: i32,
    pub y: i32,
}

impl Pos {
    /// Create a new position
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    /// Check if position is within selection range
    pub fn within(start: &Pos, end: &Pos, pos: &Pos) -> bool {
        let (low, high) = Self::to_low_high(start, end);

        if pos.y < low.y || pos.y > high.y {
            return false;
        }

        if pos.y == low.y && pos.y == high.y {
            // Single line
            pos.x >= low.x && pos.x <= high.x
        } else if pos.y == low.y {
            // First line
            pos.x >= low.x
        } else if pos.y == high.y {
            // Last line
            pos.x <= high.x
        } else {
            // Middle lines
            true
        }
    }

    /// Convert two positions to normalized (low, high) order
    pub fn to_low_high(a: &Pos, b: &Pos) -> (Pos, Pos) {
        if a.y < b.y || (a.y == b.y && a.x <= b.x) {
            (*a, *b)
        } else {
            (*b, *a)
        }
    }
}

/// Copy mode state
///
/// This is the key feature from mprocs: when entering copy mode,
/// we clone the screen state so selection coordinates remain stable
/// even as new terminal output arrives.
#[derive(Debug, Clone)]
pub enum CopyMode {
    /// Not in copy mode, optionally storing last mouse position
    None,

    /// Active copy mode with frozen screen snapshot
    Active {
        /// Frozen screen state at time of entering copy mode
        screen: Screen,

        /// Selection start position
        start: Pos,

        /// Selection end position (None until user sets it)
        end: Option<Pos>,
    },
}

impl CopyMode {
    /// Check if currently in copy mode
    pub fn is_active(&self) -> bool {
        matches!(self, CopyMode::Active { .. })
    }

    /// Get selection range if active
    pub fn get_selection(&self) -> Option<(Pos, Pos)> {
        match self {
            CopyMode::Active { start, end, .. } => Some((*start, end.unwrap_or(*start))),
            CopyMode::None => None,
        }
    }

    /// Get frozen screen if in copy mode
    pub fn screen(&self) -> Option<&Screen> {
        match self {
            CopyMode::Active { screen, .. } => Some(screen),
            CopyMode::None => None,
        }
    }

    /// Enter copy mode with frozen screen
    pub fn enter(screen: Screen, start: Pos) -> Self {
        CopyMode::Active {
            screen,
            start,
            end: None,
        }
    }

    /// Exit copy mode
    pub fn exit() -> Self {
        CopyMode::None
    }

    /// Move cursor in copy mode
    pub fn move_cursor(&mut self, dx: i32, dy: i32) {
        if let CopyMode::Active { screen, start, end } = self {
            let pos = end.as_mut().unwrap_or(start);

            // Apply movement
            pos.x = (pos.x + dx).max(0).min(screen.size().cols as i32 - 1);
            pos.y = (pos.y + dy)
                .max(-(screen.scrollback_len() as i32))
                .min(screen.size().rows as i32 - 1);
        }
    }

    /// Set end position (start range selection)
    pub fn set_end(&mut self) {
        if let CopyMode::Active { start, end, .. } = self {
            if end.is_none() {
                *end = Some(*start);
            }
        }
    }

    /// Get selected text from frozen screen
    pub fn get_selected_text(&self) -> Option<String> {
        match self {
            CopyMode::Active { screen, start, end } => {
                let end_pos = end.unwrap_or(*start);
                let (low, high) = Pos::to_low_high(start, &end_pos);
                Some(screen.get_selected_text(low.x, low.y, high.x, high.y))
            }
            CopyMode::None => None,
        }
    }
}

/// Direction for cursor movement in copy mode
#[derive(Debug, Clone, Copy)]
pub enum CopyMoveDir {
    Up,
    Down,
    Left,
    Right,
}

impl CopyMoveDir {
    /// Get delta for this direction
    pub fn delta(&self) -> (i32, i32) {
        match self {
            CopyMoveDir::Up => (0, -1),
            CopyMoveDir::Down => (0, 1),
            CopyMoveDir::Left => (-1, 0),
            CopyMoveDir::Right => (1, 0),
        }
    }
}
