use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color as RatColor, Modifier, Style},
};

use crate::primitives::termtui::vt100::{Attrs, Color, Screen};

pub fn render_screen(screen: &Screen, area: Rect, buf: &mut Buffer) {
    if area.width == 0 || area.height == 0 {
        return;
    }

    for row in 0..area.height {
        for col in 0..area.width {
            let Some(buf_cell) = buf.cell_mut((area.x + col, area.y + row)) else {
                continue;
            };
            if let Some(cell) = screen.cell(row, col) {
                let symbol = if cell.has_contents() {
                    cell.contents()
                } else {
                    " "
                };
                buf_cell.set_symbol(symbol);
                buf_cell.set_style(style_from_attrs(*cell.attrs()));
            } else {
                buf_cell.set_symbol("?");
            }
        }
    }

    let scrollback = screen.scrollback();
    if scrollback > 0 {
        let label = format!(" -{} ", scrollback);
        let label_width = label.len() as u16;
        if label_width <= area.width {
            let x = area.x + area.width - label_width;
            let style =
                style_from_attrs(Attrs::default().fg(Color::BLACK).bg(Color::BRIGHT_YELLOW));
            buf.set_string(x, area.y, label, style);
        }
    }
}

fn style_from_attrs(attrs: Attrs) -> Style {
    let mut style = Style::default()
        .fg(to_ratatui_color(attrs.fgcolor))
        .bg(to_ratatui_color(attrs.bgcolor));

    let mut modifier = Modifier::empty();
    if attrs.bold() {
        modifier |= Modifier::BOLD;
    }
    if attrs.italic() {
        modifier |= Modifier::ITALIC;
    }
    if attrs.underline() {
        modifier |= Modifier::UNDERLINED;
    }
    if attrs.inverse() {
        modifier |= Modifier::REVERSED;
    }

    style = style.add_modifier(modifier);
    style
}

fn to_ratatui_color(color: Color) -> RatColor {
    match color {
        Color::Default => RatColor::Reset,
        Color::Idx(idx) => match idx {
            0 => RatColor::Black,
            1 => RatColor::Red,
            2 => RatColor::Green,
            3 => RatColor::Yellow,
            4 => RatColor::Blue,
            5 => RatColor::Magenta,
            6 => RatColor::Cyan,
            7 => RatColor::Gray,
            8 => RatColor::DarkGray,
            9 => RatColor::LightRed,
            10 => RatColor::LightGreen,
            11 => RatColor::LightYellow,
            12 => RatColor::LightBlue,
            13 => RatColor::LightMagenta,
            14 => RatColor::LightCyan,
            15 => RatColor::White,
            _ => RatColor::Indexed(idx),
        },
        Color::Rgb(r, g, b) => RatColor::Rgb(r, g, b),
    }
}
