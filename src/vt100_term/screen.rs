//! Screen state management
//!
//! Handles terminal state including cursor position, attributes,
//! and processing of termwiz escape sequence actions.

use super::cell::{Attrs, Cell};
use super::grid::{Grid, Size};
use termwiz::escape::Action;

/// Cursor position
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Cursor {
    row: usize,
    col: usize,
}

impl Cursor {
    fn new() -> Self {
        Self { row: 0, col: 0 }
    }
}

/// Terminal screen state
#[derive(Debug, Clone)]
pub struct Screen {
    /// The grid of cells
    grid: Grid,

    /// Current cursor position
    cursor: Cursor,

    /// Current text attributes
    attrs: Attrs,

    /// Saved cursor position (for save/restore)
    saved_cursor: Option<Cursor>,

    /// Whether cursor is visible
    #[allow(dead_code)]
    cursor_visible: bool,
}

impl Screen {
    /// Create a new screen
    pub fn new(rows: usize, cols: usize, scrollback_len: usize) -> Self {
        Self {
            grid: Grid::new(rows, cols, scrollback_len),
            cursor: Cursor::new(),
            attrs: Attrs::default(),
            saved_cursor: None,
            cursor_visible: true,
        }
    }

    /// Get screen size
    pub fn size(&self) -> Size {
        self.grid.size()
    }

    /// Get current scrollback offset
    pub fn scrollback(&self) -> usize {
        self.grid.scrollback()
    }

    /// Get maximum scrollback length
    pub fn scrollback_len(&self) -> usize {
        self.grid.scrollback_len()
    }

    /// Set scrollback offset
    pub fn set_scrollback(&mut self, offset: usize) {
        self.grid.set_scrollback(offset);
    }

    /// Scroll screen up (view older content)
    pub fn scroll_screen_up(&mut self, n: usize) {
        self.grid.scroll_screen_up(n);
    }

    /// Scroll screen down (view newer content)
    pub fn scroll_screen_down(&mut self, n: usize) {
        self.grid.scroll_screen_down(n);
    }

    /// Get cell at position (relative to visible area)
    pub fn cell(&self, row: usize, col: usize) -> Option<&Cell> {
        self.grid.cell(row, col)
    }

    /// Resize the screen
    pub fn resize(&mut self, rows: usize, cols: usize) {
        self.grid.resize(rows, cols);
        // Clamp cursor to new bounds
        self.cursor.row = self.cursor.row.min(rows.saturating_sub(1));
        self.cursor.col = self.cursor.col.min(cols.saturating_sub(1));
    }

    /// Get selected text
    pub fn get_selected_text(&self, low_x: i32, low_y: i32, high_x: i32, high_y: i32) -> String {
        self.grid.get_selected_text(low_x, low_y, high_x, high_y)
    }

    /// Handle a termwiz action
    pub fn handle_action(&mut self, action: &Action) {
        match action {
            Action::Print(c) => self.print_char(*c),
            Action::PrintString(s) => {
                for c in s.chars() {
                    self.print_char(c);
                }
            }
            Action::Control(code) => {
                // ControlCode is an enum, convert to u8
                let byte = match code {
                    termwiz::escape::ControlCode::Null => 0x00,
                    termwiz::escape::ControlCode::Bell => 0x07,
                    termwiz::escape::ControlCode::Backspace => 0x08,
                    termwiz::escape::ControlCode::HorizontalTab => 0x09,
                    termwiz::escape::ControlCode::LineFeed => 0x0A,
                    termwiz::escape::ControlCode::VerticalTab => 0x0B,
                    termwiz::escape::ControlCode::FormFeed => 0x0C,
                    termwiz::escape::ControlCode::CarriageReturn => 0x0D,
                    _ => return, // Ignore other control codes
                };
                self.handle_control(byte);
            }
            Action::CSI(csi) => self.handle_csi(csi),
            Action::Esc(esc) => self.handle_esc(esc),
            Action::OperatingSystemCommand(osc) => self.handle_osc(osc),
            _ => {
                // Ignore other action types for now
            }
        }
    }

    /// Print a character at cursor position
    fn print_char(&mut self, c: char) {
        let size = self.grid.size();

        // Handle newline
        if c == '\n' {
            self.cursor.col = 0;
            self.cursor.row += 1;
            if self.cursor.row >= size.rows {
                self.grid.scroll_up(1);
                self.cursor.row = size.rows - 1;
            }
            return;
        }

        // Handle carriage return
        if c == '\r' {
            self.cursor.col = 0;
            return;
        }

        // Handle tab
        if c == '\t' {
            self.cursor.col = ((self.cursor.col + 8) / 8) * 8;
            if self.cursor.col >= size.cols {
                self.cursor.col = 0;
                self.cursor.row += 1;
                if self.cursor.row >= size.rows {
                    self.grid.scroll_up(1);
                    self.cursor.row = size.rows - 1;
                }
            }
            return;
        }

        // Print normal character
        if let Some(cell) = self.grid.cell_mut(self.cursor.row, self.cursor.col) {
            cell.text = c.to_string();
            cell.attrs = self.attrs;
        }

        // Advance cursor
        self.cursor.col += 1;
        if self.cursor.col >= size.cols {
            self.cursor.col = 0;
            self.cursor.row += 1;
            if self.cursor.row >= size.rows {
                self.grid.scroll_up(1);
                self.cursor.row = size.rows - 1;
            }
        }
    }

    /// Handle control codes
    fn handle_control(&mut self, code: u8) {
        match code {
            0x08 => {
                // Backspace
                if self.cursor.col > 0 {
                    self.cursor.col -= 1;
                }
            }
            0x0A => {
                // Line feed
                self.cursor.row += 1;
                if self.cursor.row >= self.grid.size().rows {
                    self.grid.scroll_up(1);
                    self.cursor.row = self.grid.size().rows - 1;
                }
            }
            0x0D => {
                // Carriage return
                self.cursor.col = 0;
            }
            _ => {}
        }
    }

    /// Handle CSI (Control Sequence Introducer) sequences
    fn handle_csi(&mut self, csi: &termwiz::escape::CSI) {
        use termwiz::escape::CSI;

        match csi {
            CSI::Cursor(cursor) => self.handle_cursor(cursor),
            CSI::Sgr(sgr) => self.handle_sgr(sgr),
            CSI::Edit(edit) => self.handle_edit(edit),
            CSI::Mode(mode) => self.handle_mode(mode),
            _ => {}
        }
    }

    /// Handle cursor movement
    fn handle_cursor(&mut self, cursor: &termwiz::escape::csi::Cursor) {
        use termwiz::escape::csi::Cursor;

        let size = self.grid.size();

        match cursor {
            Cursor::Position { line, col } => {
                self.cursor.row = (line.as_zero_based() as usize).min(size.rows - 1);
                self.cursor.col = (col.as_zero_based() as usize).min(size.cols - 1);
            }
            Cursor::Up(n) => {
                self.cursor.row = self.cursor.row.saturating_sub(*n as usize);
            }
            Cursor::Down(n) => {
                self.cursor.row = (self.cursor.row + *n as usize).min(size.rows - 1);
            }
            Cursor::Right(n) => {
                self.cursor.col = (self.cursor.col + *n as usize).min(size.cols - 1);
            }
            Cursor::Left(n) => {
                self.cursor.col = self.cursor.col.saturating_sub(*n as usize);
            }
            Cursor::CharacterAbsolute(col) => {
                self.cursor.col = (col.as_zero_based() as usize).min(size.cols - 1);
            }
            Cursor::LineTabulation(n) => {
                self.cursor.row = (self.cursor.row + *n as usize).min(size.rows - 1);
            }
            Cursor::SaveCursor => {
                self.saved_cursor = Some(self.cursor);
            }
            Cursor::RestoreCursor => {
                if let Some(saved) = self.saved_cursor {
                    self.cursor = saved;
                }
            }
            _ => {}
        }
    }

    /// Handle SGR (Select Graphic Rendition) - text attributes
    fn handle_sgr(&mut self, sgr: &termwiz::escape::csi::Sgr) {
        use termwiz::escape::csi::Sgr;

        match sgr {
            Sgr::Reset => {
                self.attrs = Attrs::default();
            }
            Sgr::Intensity(intensity) => {
                self.attrs.intensity = *intensity;
            }
            Sgr::Underline(underline) => {
                self.attrs.underline = *underline != termwiz::cell::Underline::None;
            }
            Sgr::Blink(blink) => {
                self.attrs.blink = *blink != termwiz::cell::Blink::None;
            }
            Sgr::Inverse(inverse) => {
                self.attrs.reverse = *inverse;
            }
            Sgr::Italic(italic) => {
                self.attrs.italic = *italic;
            }
            Sgr::StrikeThrough(strike) => {
                self.attrs.strikethrough = *strike;
            }
            Sgr::Foreground(color) => {
                // Convert ColorSpec to ColorAttribute
                self.attrs.fgcolor = match color {
                    termwiz::color::ColorSpec::Default => termwiz::color::ColorAttribute::Default,
                    termwiz::color::ColorSpec::PaletteIndex(idx) => {
                        termwiz::color::ColorAttribute::PaletteIndex(*idx)
                    }
                    termwiz::color::ColorSpec::TrueColor(rgb) => {
                        termwiz::color::ColorAttribute::TrueColorWithDefaultFallback(*rgb)
                    }
                };
            }
            Sgr::Background(color) => {
                // Convert ColorSpec to ColorAttribute
                self.attrs.bgcolor = match color {
                    termwiz::color::ColorSpec::Default => termwiz::color::ColorAttribute::Default,
                    termwiz::color::ColorSpec::PaletteIndex(idx) => {
                        termwiz::color::ColorAttribute::PaletteIndex(*idx)
                    }
                    termwiz::color::ColorSpec::TrueColor(rgb) => {
                        termwiz::color::ColorAttribute::TrueColorWithDefaultFallback(*rgb)
                    }
                };
            }
            _ => {}
        }
    }

    /// Handle edit operations
    fn handle_edit(&mut self, edit: &termwiz::escape::csi::Edit) {
        use termwiz::escape::csi::Edit;

        match edit {
            Edit::EraseInLine(erase) => {
                use termwiz::escape::csi::EraseInLine;
                let size = self.grid.size();
                let row = self.cursor.row;

                match erase {
                    EraseInLine::EraseToEndOfLine => {
                        for col in self.cursor.col..size.cols {
                            if let Some(cell) = self.grid.cell_mut(row, col) {
                                *cell = Cell::default();
                            }
                        }
                    }
                    EraseInLine::EraseToStartOfLine => {
                        for col in 0..=self.cursor.col {
                            if let Some(cell) = self.grid.cell_mut(row, col) {
                                *cell = Cell::default();
                            }
                        }
                    }
                    EraseInLine::EraseLine => {
                        for col in 0..size.cols {
                            if let Some(cell) = self.grid.cell_mut(row, col) {
                                *cell = Cell::default();
                            }
                        }
                    }
                }
            }
            Edit::EraseInDisplay(erase) => {
                use termwiz::escape::csi::EraseInDisplay;

                match erase {
                    EraseInDisplay::EraseToEndOfDisplay => {
                        // Clear from cursor to end of screen
                        let size = self.grid.size();

                        // Clear rest of current line
                        for col in self.cursor.col..size.cols {
                            if let Some(cell) = self.grid.cell_mut(self.cursor.row, col) {
                                *cell = Cell::default();
                            }
                        }

                        // Clear all lines below
                        for row in (self.cursor.row + 1)..size.rows {
                            for col in 0..size.cols {
                                if let Some(cell) = self.grid.cell_mut(row, col) {
                                    *cell = Cell::default();
                                }
                            }
                        }
                    }
                    EraseInDisplay::EraseToStartOfDisplay => {
                        let size = self.grid.size();

                        // Clear all lines above
                        for row in 0..self.cursor.row {
                            for col in 0..size.cols {
                                if let Some(cell) = self.grid.cell_mut(row, col) {
                                    *cell = Cell::default();
                                }
                            }
                        }

                        // Clear start of current line
                        for col in 0..=self.cursor.col {
                            if let Some(cell) = self.grid.cell_mut(self.cursor.row, col) {
                                *cell = Cell::default();
                            }
                        }
                    }
                    EraseInDisplay::EraseDisplay => {
                        self.grid.clear();
                    }
                    _ => {}
                }
            }
            _ => {}
        }
    }

    /// Handle mode changes
    fn handle_mode(&mut self, _mode: &termwiz::escape::csi::Mode) {
        // Mode changes like show/hide cursor, alternate screen, etc.
        // For now, we'll keep it simple and ignore mode changes
        // TODO: Handle DECTCEM (cursor visibility) and other modes
    }

    /// Handle ESC sequences
    fn handle_esc(&mut self, _esc: &termwiz::escape::Esc) {
        // Most ESC sequences are handled by CSI
    }

    /// Handle OSC (Operating System Command) sequences
    fn handle_osc(&mut self, _osc: &termwiz::escape::OperatingSystemCommand) {
        // OSC sequences like clipboard (OSC 52) could be handled here
        // For now, we'll handle clipboard in the VT100Term widget
    }
}
