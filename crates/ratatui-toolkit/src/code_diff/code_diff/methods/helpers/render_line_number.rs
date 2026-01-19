use ratatui::buffer::Buffer;
use ratatui::layout::Position;
use ratatui::style::Style;

use crate::code_diff::diff_config::DiffConfig;

/// Renders a line number in the gutter.
///
/// # Arguments
///
/// * `line_num` - The line number to render (None for empty space)
/// * `config` - Display configuration
/// * `x` - Starting x position
/// * `y` - Y position
/// * `buf` - The buffer to render to
/// * `bg_style` - Background style to apply
///
/// # Returns
///
/// The x position after rendering the line number
pub fn render_line_number(
    line_num: Option<usize>,
    config: &DiffConfig,
    x: u16,
    y: u16,
    buf: &mut Buffer,
    bg_style: Style,
) -> u16 {
    if !config.show_line_numbers {
        return x;
    }

    let width = config.line_number_width as usize;
    let num_style = bg_style.fg(config.line_number_fg);

    let num_str = match line_num {
        Some(n) => format!("{:>width$}", n, width = width),
        None => " ".repeat(width),
    };

    let mut current_x = x;
    for ch in num_str.chars() {
        if current_x < x + config.line_number_width {
            if let Some(cell) = buf.cell_mut(Position::new(current_x, y)) {
                cell.set_char(ch);
                cell.set_style(num_style);
            }
            current_x += 1;
        }
    }

    // Add separator space
    if current_x < x + config.line_number_width + 1 {
        if let Some(cell) = buf.cell_mut(Position::new(current_x, y)) {
            cell.set_char(' ');
            cell.set_style(bg_style);
        }
        current_x += 1;
    }

    current_x
}
