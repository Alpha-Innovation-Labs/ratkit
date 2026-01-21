//! Expanded mode rendering for TOC (full heading text).

use ratatui::{buffer::Buffer, layout::Rect};
use unicode_width::UnicodeWidthStr;

use crate::widgets::markdown_widget::extensions::toc::{Toc, TocStyle};

impl<'a> Toc<'a> {
    /// Render the TOC in expanded mode (full heading text).
    ///
    /// Shows heading text with indentation based on level.
    /// Active heading is shown in blue, hovered has background highlight.
    pub(crate) fn render_expanded(&self, area: Rect, buf: &mut Buffer) {
        let entries = &self.toc_state.entries;
        if entries.is_empty() || area.height == 0 {
            return;
        }

        let padding_left: u16 = 2;
        let padding_right: u16 = 1;
        let available_width = area.width.saturating_sub(padding_left + padding_right) as usize;

        // Use TOC scroll offset for scrolling through the list
        let visible_count = area.height as usize;
        let start_idx = self.toc_state.scroll_offset;

        // Get hover state from TocState
        let hovered_index = self.toc_state.hovered_entry;
        // Note: active_index tracking may need to be added to TocState
        let active_index: Option<usize> = None;

        for (display_idx, entry_idx) in (start_idx..entries.len()).take(visible_count).enumerate() {
            let entry = &entries[entry_idx];
            let y = area.y + display_idx as u16;

            if y >= area.y + area.height {
                break;
            }

            // Calculate indentation based on heading level
            let indent = ((entry.level - 1) as usize) * 2;
            let indent_str = " ".repeat(indent.min(available_width / 2));

            // Truncate text to fit available width
            let text_space = available_width.saturating_sub(indent);
            let display_text = truncate_text(&entry.text, text_space);

            // Determine style based on state
            let (text_style, fill_bg) = if Some(entry_idx) == hovered_index {
                (self.config.hover_style, true)
            } else if Some(entry_idx) == active_index {
                (self.config.active_style, false)
            } else {
                (self.config.text_style, false)
            };

            // Clerk-style accent line on the left
            let accent_char = '│';
            let is_active_or_hovered =
                Some(entry_idx) == hovered_index || Some(entry_idx) == active_index;
            let accent_style = if is_active_or_hovered {
                self.config.active_accent_style
            } else if self.config.style == TocStyle::Clerk {
                // Subtle accent for inactive entries in clerk mode
                self.config.accent_style
            } else {
                text_style
            };

            // Render clerk-style accent bar on left
            if self.config.style == TocStyle::Clerk {
                let accent_x = area.x;
                if let Some(cell) = buf.cell_mut((accent_x, y)) {
                    cell.set_char(accent_char).set_style(accent_style);
                }
            }

            // Fill background for hovered items
            if fill_bg {
                for x in area.x + 1..area.x + area.width {
                    if let Some(cell) = buf.cell_mut((x, y)) {
                        cell.set_style(self.config.hover_style);
                    }
                }
            }

            // Render the text
            let x = area.x + padding_left;
            let full_text = format!("{}{}", indent_str, display_text);

            let mut current_x = x;
            for ch in full_text.chars() {
                if current_x >= area.x + area.width - padding_right {
                    break;
                }
                let ch_width = unicode_width::UnicodeWidthChar::width(ch).unwrap_or(1) as u16;
                if let Some(cell) = buf.cell_mut((current_x, y)) {
                    cell.set_char(ch).set_style(text_style);
                }
                current_x += ch_width;
            }
        }
    }
}

/// Truncate text to fit within a given width, adding ellipsis if needed.
fn truncate_text(text: &str, max_width: usize) -> String {
    if text.width() <= max_width {
        return text.to_string();
    }

    if max_width <= 3 {
        return "...".chars().take(max_width).collect();
    }

    let mut result = String::new();
    let mut current_width = 0;
    let target_width = max_width - 1; // Leave room for ellipsis

    for ch in text.chars() {
        let ch_width = unicode_width::UnicodeWidthChar::width(ch).unwrap_or(1);
        if current_width + ch_width > target_width {
            break;
        }
        result.push(ch);
        current_width += ch_width;
    }

    result.push('\u{2026}');
    result
}
