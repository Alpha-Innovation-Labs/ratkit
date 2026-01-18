//! Compact mode rendering for TOC using Canvas with Braille markers for thin lines.

use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::Color,
    symbols::Marker,
    widgets::{
        canvas::{Canvas, Line},
        Widget,
    },
};

use super::super::Toc;
use super::calculate_line_width::calculate_line_width;

impl<'a> Toc<'a> {
    /// Render the TOC in compact mode using Canvas with Braille markers.
    ///
    /// Braille gives 2x4 dots per cell for thin lines with sub-pixel positioning.
    /// `line_spacing` controls the spacing between lines in dot units.
    /// Uses two-pass rendering to ensure active line color is correct.
    pub(crate) fn render_compact(&self, area: Rect, buf: &mut Buffer) {
        if area.height == 0 || area.width == 0 {
            return;
        }

        // Fill entire area with background first (including under border)
        fill_background(buf, area, self.config.background_style);

        // Draw border on top of background
        let content_area = if self.config.show_border {
            self.render_border(area, buf)
        } else {
            area
        };

        if content_area.width == 0 || content_area.height == 0 {
            return;
        }

        if self.entries.is_empty() {
            return;
        }

        render_compact_lines(
            &self.entries,
            content_area,
            buf,
            self.config.line_spacing,
            self.config.line_style.fg.unwrap_or(Color::Gray),
            self.config.active_line_style.fg.unwrap_or(Color::Yellow),
            self.active_index,
        );
    }
}

/// Fill an area with background style.
fn fill_background(buf: &mut Buffer, area: Rect, style: ratatui::style::Style) {
    for y in area.y..area.y + area.height {
        for x in area.x..area.x + area.width {
            if let Some(cell) = buf.cell_mut((x, y)) {
                cell.set_char(' ').set_style(style);
            }
        }
    }
}

/// Render compact lines using Canvas with two-pass rendering.
fn render_compact_lines(
    entries: &[super::super::TocEntry],
    content_area: Rect,
    buf: &mut Buffer,
    line_spacing: u8,
    normal_color: Color,
    active_color: Color,
    active_index: Option<usize>,
) {
    let spacing = line_spacing.max(1) as f64;

    // Canvas coordinates: x = 0..width*2, y = 0..height*4 (Braille: 2x4 dots per cell)
    let canvas_width = (content_area.width as f64) * 2.0;
    let canvas_height = (content_area.height as f64) * 4.0;

    // Two-pass rendering: non-active lines first, then active line
    let canvas = Canvas::default()
        .marker(Marker::Braille)
        .x_bounds([0.0, canvas_width])
        .y_bounds([0.0, canvas_height])
        .paint(move |ctx| {
            // Pass 1: Draw all non-active lines
            for (idx, entry) in entries.iter().enumerate() {
                if Some(idx) == active_index {
                    continue;
                }

                let pixel_y = canvas_height - (idx as f64 * spacing);
                if pixel_y <= 0.0 {
                    break;
                }

                let line_width = calculate_line_width(canvas_width, entry.level);
                let x_start = canvas_width - line_width;

                ctx.draw(&Line {
                    x1: x_start,
                    y1: pixel_y,
                    x2: canvas_width,
                    y2: pixel_y,
                    color: normal_color,
                });
            }

            // Pass 2: Draw active line last so it wins shared cells
            if let Some(active_idx) = active_index {
                if let Some(entry) = entries.get(active_idx) {
                    let pixel_y = canvas_height - (active_idx as f64 * spacing);
                    if pixel_y > 0.0 {
                        let line_width = calculate_line_width(canvas_width, entry.level);
                        let x_start = canvas_width - line_width;

                        ctx.draw(&Line {
                            x1: x_start,
                            y1: pixel_y,
                            x2: canvas_width,
                            y2: pixel_y,
                            color: active_color,
                        });
                    }
                }
            }
        });

    canvas.render(content_area, buf);
}
