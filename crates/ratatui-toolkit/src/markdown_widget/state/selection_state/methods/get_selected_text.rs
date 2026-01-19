//! Extract selected text from rendered lines.

use crate::markdown_widget::state::selection_state::SelectionState;
use ratatui::text::Line;

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
