use alacritty_terminal::grid::Dimensions;
use alacritty_terminal::term::Term;
use alacritty_terminal::vte::ansi::{Color as AnsiColor, NamedColor};
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Widget},
};

/// Widget for rendering an alacritty terminal in ratatui
pub struct AlacrittyWidget<'a, T> {
    term: &'a Term<T>,
    block: Option<Block<'a>>,
    scroll_offset: usize,
}

impl<'a, T> AlacrittyWidget<'a, T> {
    /// Create a new AlacrittyWidget
    pub fn new(term: &'a Term<T>) -> Self {
        Self {
            term,
            block: None,
            scroll_offset: 0,
        }
    }

    /// Set scroll offset (lines to scroll back from current position)
    pub fn scroll_offset(mut self, offset: usize) -> Self {
        self.scroll_offset = offset;
        self
    }

    /// Wrap the widget in a block
    #[allow(dead_code)]
    pub fn block(mut self, block: Block<'a>) -> Self {
        self.block = Some(block);
        self
    }
}

impl<T> Widget for AlacrittyWidget<'_, T> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let area = match self.block {
            Some(b) => {
                let inner = b.inner(area);
                b.render(area, buf);
                inner
            }
            None => area,
        };

        // Get terminal grid dimensions
        let grid = self.term.grid();
        let screen_lines = grid.screen_lines().min(area.height as usize);
        let columns = grid.columns().min(area.width as usize);

        // Calculate the starting line based on scroll offset
        // Negative indices go into scrollback history
        let scroll_offset = self.scroll_offset.min(grid.history_size());

        // Render each visible row
        for row_idx in 0..screen_lines {
            let y = area.y + row_idx as u16;
            if y >= area.y + area.height {
                break;
            }

            let mut spans = Vec::new();
            let mut current_style = Style::default();
            let mut current_text = String::new();

            // Calculate the line index for this row
            // When scroll_offset = 0: render Line(0) to Line(screen_lines-1) (current view)
            // When scroll_offset > 0: render from scrollback
            let line_offset = row_idx as i32 - scroll_offset as i32;

            // Skip rows that would access invalid history
            // topmost_line is the oldest line in scrollback (most negative)
            if line_offset < -(grid.history_size() as i32) {
                // This line is beyond available history, skip it
                continue;
            }

            // Iterate through columns in this row
            for col_idx in 0..columns {
                let line = alacritty_terminal::index::Line(line_offset);
                let column = alacritty_terminal::index::Column(col_idx);

                let cell = &grid[line][column];

                // Convert cell style to ratatui style
                let cell_style = convert_cell_style(cell);

                // If style changed, push previous span and start new one
                if cell_style != current_style && !current_text.is_empty() {
                    spans.push(Span::styled(current_text.clone(), current_style));
                    current_text.clear();
                    current_style = cell_style;
                } else if current_text.is_empty() {
                    current_style = cell_style;
                }

                // Add character to current span
                current_text.push(cell.c);
            }

            // Push final span if any
            if !current_text.is_empty() {
                spans.push(Span::styled(current_text, current_style));
            }

            // Render the line
            if !spans.is_empty() {
                let line = Line::from(spans);
                buf.set_line(area.x, y, &line, area.width);
            }
        }

        // Render cursor
        let cursor_point = self.term.grid().cursor.point;
        let cursor_line = cursor_point.line.0;
        let cursor_col = cursor_point.column.0;

        // Calculate if cursor is visible in current view (accounting for scroll)
        let cursor_row_in_view = cursor_line + scroll_offset as i32;

        // Only render cursor if it's within the visible area
        if cursor_row_in_view >= 0
            && cursor_row_in_view < screen_lines as i32
            && cursor_col < columns
        {
            let cursor_x = area.x + cursor_col as u16;
            let cursor_y = area.y + cursor_row_in_view as u16;

            // Get the cell at cursor position for the character
            let cursor_cell = &grid[cursor_point.line][cursor_point.column];

            // Render cursor as reversed colors (inverted)
            let cursor_style = Style::default()
                .bg(Color::White)
                .fg(Color::Black)
                .add_modifier(Modifier::REVERSED);

            if let Some(cell) = buf.cell_mut((cursor_x, cursor_y)) {
                cell.set_char(if cursor_cell.c == ' ' {
                    '█'
                } else {
                    cursor_cell.c
                })
                .set_style(cursor_style);
            }
        }
    }
}

/// Convert alacritty cell style to ratatui style
fn convert_cell_style(cell: &alacritty_terminal::term::cell::Cell) -> Style {
    let mut style = Style::default();

    // Convert foreground color
    if let Some(fg) = convert_color(&cell.fg) {
        style = style.fg(fg);
    }

    // Convert background color
    if let Some(bg) = convert_color(&cell.bg) {
        style = style.bg(bg);
    }

    // Convert attributes
    let flags = cell.flags;
    if flags.contains(alacritty_terminal::term::cell::Flags::BOLD) {
        style = style.add_modifier(Modifier::BOLD);
    }
    if flags.contains(alacritty_terminal::term::cell::Flags::ITALIC) {
        style = style.add_modifier(Modifier::ITALIC);
    }
    if flags.contains(alacritty_terminal::term::cell::Flags::UNDERLINE) {
        style = style.add_modifier(Modifier::UNDERLINED);
    }
    if flags.contains(alacritty_terminal::term::cell::Flags::INVERSE) {
        style = style.add_modifier(Modifier::REVERSED);
    }
    if flags.contains(alacritty_terminal::term::cell::Flags::STRIKEOUT) {
        style = style.add_modifier(Modifier::CROSSED_OUT);
    }

    style
}

/// Convert alacritty color to ratatui color
fn convert_color(color: &AnsiColor) -> Option<Color> {
    match color {
        AnsiColor::Named(named) => Some(convert_named_color(*named)),
        AnsiColor::Spec(rgb) => Some(Color::Rgb(rgb.r, rgb.g, rgb.b)),
        AnsiColor::Indexed(idx) => {
            // Basic ANSI colors (0-15)
            match idx {
                0 => Some(Color::Black),
                1 => Some(Color::Red),
                2 => Some(Color::Green),
                3 => Some(Color::Yellow),
                4 => Some(Color::Blue),
                5 => Some(Color::Magenta),
                6 => Some(Color::Cyan),
                7 => Some(Color::Gray),
                8 => Some(Color::DarkGray),
                9 => Some(Color::LightRed),
                10 => Some(Color::LightGreen),
                11 => Some(Color::LightYellow),
                12 => Some(Color::LightBlue),
                13 => Some(Color::LightMagenta),
                14 => Some(Color::LightCyan),
                15 => Some(Color::White),
                _ => Some(Color::Indexed(*idx)),
            }
        }
    }
}

/// Convert alacritty named color to ratatui color
fn convert_named_color(named: NamedColor) -> Color {
    match named {
        NamedColor::Black => Color::Black,
        NamedColor::Red => Color::Red,
        NamedColor::Green => Color::Green,
        NamedColor::Yellow => Color::Yellow,
        NamedColor::Blue => Color::Blue,
        NamedColor::Magenta => Color::Magenta,
        NamedColor::Cyan => Color::Cyan,
        NamedColor::White => Color::White,
        NamedColor::BrightBlack => Color::DarkGray,
        NamedColor::BrightRed => Color::LightRed,
        NamedColor::BrightGreen => Color::LightGreen,
        NamedColor::BrightYellow => Color::LightYellow,
        NamedColor::BrightBlue => Color::LightBlue,
        NamedColor::BrightMagenta => Color::LightMagenta,
        NamedColor::BrightCyan => Color::LightCyan,
        NamedColor::BrightWhite => Color::White,
        _ => Color::Reset,
    }
}
