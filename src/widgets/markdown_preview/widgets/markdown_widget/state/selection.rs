//! Selection state for markdown widget text selection and copy.

use crate::widgets::markdown_preview::widgets::markdown_widget::foundation::types::SelectionPos;

/// Selection state for markdown widget.
///
/// Tracks whether selection mode is active and the selection bounds.
#[derive(Debug, Clone, Default)]
pub struct SelectionState {
    /// Whether selection mode is active.
    pub active: bool,
    /// Selection anchor (start point).
    pub anchor: Option<SelectionPos>,
    /// Current cursor/end position.
    pub cursor: Option<SelectionPos>,
    /// Cached rendered lines for stable selection.
    pub frozen_lines: Option<Vec<ratatui::text::Line<'static>>>,
    /// Width when lines were frozen.
    pub frozen_width: usize,
    /// Last copied text (for showing toast notification).
    pub last_copied_text: Option<String>,
}

/// Constructor for SelectionState.

impl SelectionState {
    /// Create a new inactive selection state.
    pub fn new() -> Self {
        Self::default()
    }
}

/// Enter selection mode.
use ratatui::text::Line;

impl SelectionState {
    /// Enter selection mode at the given position.
    ///
    /// # Arguments
    ///
    /// * `x` - X coordinate (column)
    /// * `y` - Y coordinate (document row)
    /// * `lines` - Current rendered lines to freeze
    /// * `width` - Current render width
    pub fn enter(&mut self, x: i32, y: i32, lines: Vec<Line<'static>>, width: usize) {
        self.active = true;
        self.anchor = Some(SelectionPos::new(x, y));
        self.cursor = Some(SelectionPos::new(x, y));
        self.frozen_lines = Some(lines);
        self.frozen_width = width;
    }

    /// Check if selection mode is active.
    pub fn is_active(&self) -> bool {
        self.active
    }
}

/// Exit selection mode.

impl SelectionState {
    /// Exit selection mode and clear state.
    pub fn exit(&mut self) {
        self.active = false;
        self.anchor = None;
        self.cursor = None;
        self.frozen_lines = None;
        self.frozen_width = 0;
    }
}

/// Extract selected text from rendered lines.

impl SelectionState {
    /// Get the selected text from the frozen lines.
    ///
    /// # Returns
    ///
    /// The selected text as a string, or `None` if no selection.
    pub fn get_selected_text(&self) -> Option<String> {
        let (start, end) = self.get_selection()?;
        let lines = self.frozen_lines.as_ref()?;

        Some(extract_text_from_lines(
            lines,
            start.x as usize,
            start.y as usize,
            end.x as usize,
            end.y as usize,
        ))
    }
}

/// Extract text from rendered lines within the selection bounds.
fn extract_text_from_lines(
    lines: &[Line<'static>],
    start_x: usize,
    start_y: usize,
    end_x: usize,
    end_y: usize,
) -> String {
    let mut result = String::new();

    for (row_idx, line) in lines.iter().enumerate() {
        if row_idx < start_y || row_idx > end_y {
            continue;
        }

        // Get the full text content of this line
        let line_text: String = line.spans.iter().map(|s| s.content.as_ref()).collect();

        // Determine column range for this row
        let col_start = if row_idx == start_y { start_x } else { 0 };
        let col_end = if row_idx == end_y {
            end_x + 1
        } else {
            line_text.chars().count()
        };

        // Extract the relevant portion
        let chars: Vec<char> = line_text.chars().collect();
        let actual_start = col_start.min(chars.len());
        let actual_end = col_end.min(chars.len());

        if actual_start < actual_end {
            let selected: String = chars[actual_start..actual_end].iter().collect();
            result.push_str(selected.trim_end());
        }

        // Add newline between lines (not after last line)
        if row_idx < end_y {
            result.push('\n');
        }
    }

    result
}

/// Get the current selection bounds.

impl SelectionState {
    /// Get the normalized selection bounds (start, end) where start <= end.
    ///
    /// # Returns
    ///
    /// `Some((start, end))` if there's an active selection, `None` otherwise.
    pub fn get_selection(&self) -> Option<(SelectionPos, SelectionPos)> {
        if !self.active {
            return None;
        }

        let anchor = self.anchor?;
        let cursor = self.cursor?;

        // Normalize so start is before end
        Some(normalize_selection(anchor, cursor))
    }

    /// Check if there's an active selection (anchor and cursor set).
    pub fn has_selection(&self) -> bool {
        self.active && self.anchor.is_some() && self.cursor.is_some()
    }
}

/// Normalize selection bounds so start <= end.
fn normalize_selection(a: SelectionPos, b: SelectionPos) -> (SelectionPos, SelectionPos) {
    if a.y < b.y || (a.y == b.y && a.x <= b.x) {
        (a, b)
    } else {
        (b, a)
    }
}

/// Check if a position is within the selection.

impl SelectionState {
    /// Check if a cell at (x, y) is within the current selection.
    ///
    /// # Arguments
    ///
    /// * `x` - X coordinate (column)
    /// * `y` - Y coordinate (row)
    ///
    /// # Returns
    ///
    /// `true` if the position is within the selection.
    pub fn is_in_selection(&self, x: i32, y: i32) -> bool {
        let Some((start, end)) = self.get_selection() else {
            return false;
        };

        is_pos_in_selection(x, y, &start, &end)
    }
}

/// Check if position (x, y) is within the selection bounds.
pub fn is_pos_in_selection(x: i32, y: i32, start: &SelectionPos, end: &SelectionPos) -> bool {
    // Outside y range
    if y < start.y || y > end.y {
        return false;
    }

    // Single line selection
    if start.y == end.y {
        return x >= start.x && x <= end.x;
    }

    // Multi-line selection
    if y == start.y {
        // First line: from start.x to end of line
        x >= start.x
    } else if y == end.y {
        // Last line: from start of line to end.x
        x <= end.x
    } else {
        // Middle lines: entire line is selected
        true
    }
}

/// Update cursor position during selection.

impl SelectionState {
    /// Update the cursor position during selection.
    ///
    /// # Arguments
    ///
    /// * `x` - New X coordinate
    /// * `y` - New Y coordinate
    pub fn update_cursor(&mut self, x: i32, y: i32) {
        if self.active {
            self.cursor = Some(SelectionPos::new(x, y));
        }
    }

    /// Set anchor at current cursor position (for keyboard selection toggle).
    pub fn set_anchor(&mut self) {
        if self.active {
            if let Some(cursor) = self.cursor {
                self.anchor = Some(cursor);
            }
        }
    }

    /// Clear anchor (deselect).
    pub fn clear_anchor(&mut self) {
        self.anchor = None;
    }

    /// Toggle anchor at current position.
    pub fn toggle_anchor(&mut self) {
        if self.anchor.is_some() {
            self.clear_anchor();
        } else {
            self.set_anchor();
        }
    }
}
