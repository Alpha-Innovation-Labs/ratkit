//! Expanded mode rendering for TOC (full heading text).

use ratatui::{buffer::Buffer, layout::Rect};
use unicode_width::UnicodeWidthStr;

use crate::widgets::markdown_widget::extensions::toc::{Toc, TocStyle};

const DEPTH_OFFSET_DEPTH_1_2: u16 = 2;
const DEPTH_OFFSET_DEPTH_3: u16 = 4;
const DEPTH_OFFSET_DEPTH_4_PLUS: u16 = 6;

const LINE_OFFSET_DEPTH_LT_3: u16 = 0;
const LINE_OFFSET_DEPTH_GTE_3: u16 = 2;

fn get_item_offset(depth: u8) -> u16 {
    match depth {
        1 | 2 => DEPTH_OFFSET_DEPTH_1_2,
        3 => DEPTH_OFFSET_DEPTH_3,
        _ => DEPTH_OFFSET_DEPTH_4_PLUS,
    }
}

fn get_line_offset(depth: u8) -> u16 {
    if depth >= 3 {
        LINE_OFFSET_DEPTH_GTE_3
    } else {
        LINE_OFFSET_DEPTH_LT_3
    }
}

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

        let visible_count = area.height as usize;
        let start_idx = self.toc_state.scroll_offset;

        let hovered_index = self.toc_state.hovered_entry;
        let active_index: Option<usize> = None;

        for (display_idx, entry_idx) in (start_idx..entries.len()).take(visible_count).enumerate() {
            let entry = &entries[entry_idx];
            let y = area.y + display_idx as u16;

            if y >= area.y + area.height {
                break;
            }

            let indent = get_item_offset(entry.level);
            let line_offset = get_line_offset(entry.level);

            let available_for_text = available_width.saturating_sub(indent as usize);
            let display_text = truncate_text(&entry.text, available_for_text);

            let (text_style, fill_bg) = if Some(entry_idx) == hovered_index {
                (self.config.hover_style, true)
            } else if Some(entry_idx) == active_index {
                (self.config.active_style, false)
            } else {
                (self.config.text_style, false)
            };

            let is_active_or_hovered =
                Some(entry_idx) == hovered_index || Some(entry_idx) == active_index;
            let accent_style = if is_active_or_hovered {
                self.config.active_accent_style
            } else if self.config.style == TocStyle::Clerk {
                self.config.accent_style
            } else {
                text_style
            };

            if self.config.style == TocStyle::Clerk {
                self.render_clerk_lines(
                    area,
                    buf,
                    y,
                    entry.level,
                    line_offset,
                    entry_idx,
                    entries.len(),
                    start_idx,
                    hovered_index,
                    accent_style,
                );
            }

            if fill_bg {
                for x in area.x + indent..area.x + area.width {
                    if let Some(cell) = buf.cell_mut((x, y)) {
                        cell.set_style(self.config.hover_style);
                    }
                }
            }

            let x = area.x + indent;
            let full_text = display_text.to_string();

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

    fn render_clerk_lines(
        &self,
        area: Rect,
        buf: &mut Buffer,
        y: u16,
        depth: u8,
        line_offset: u16,
        entry_idx: usize,
        total_entries: usize,
        start_idx: usize,
        _hovered_index: Option<usize>,
        accent_style: ratatui::style::Style,
    ) {
        let line_x = area.x + line_offset;
        let border_x = area.x + line_offset;
        let next_entry_depth = if entry_idx + 1 < total_entries {
            Some(self.toc_state.entries[entry_idx + 1].level)
        } else {
            None
        };
        let prev_entry_idx = if entry_idx > start_idx {
            Some(entry_idx - 1)
        } else {
            None
        };
        let prev_entry_depth = prev_entry_idx
            .map(|idx| self.toc_state.entries[idx].level)
            .unwrap_or(depth);

        let upper_offset = get_line_offset(prev_entry_depth);
        let lower_offset = next_entry_depth.map(get_line_offset).unwrap_or(line_offset);

        if line_offset != upper_offset {
            let zigzag_char = match (upper_offset, line_offset) {
                (0, 2) => '└',
                (2, 0) => '┌',
                _ => '│',
            };
            if let Some(cell) = buf.cell_mut((line_x, y)) {
                cell.set_char(zigzag_char).set_style(accent_style);
            }
        }

        let vertical_x = border_x;
        let show_top = line_offset != upper_offset;
        let show_bottom = lower_offset != line_offset;

        let start_y = if show_top { y + 1 } else { y };
        let end_y = if show_bottom { y.saturating_sub(1) } else { y };

        if start_y <= end_y {
            for vy in start_y..=end_y {
                if let Some(cell) = buf.cell_mut((vertical_x, vy)) {
                    cell.set_char('│').set_style(accent_style);
                }
            }
        }

        if let Some(prev_y) = y.checked_sub(1) {
            if line_offset != upper_offset {
                let corner_char = '├';
                if let Some(cell) = buf.cell_mut((line_x, prev_y)) {
                    cell.set_char(corner_char).set_style(accent_style);
                }
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
    let target_width = max_width - 1;

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
