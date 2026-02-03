//! Terminal screen state manager

use crate::attrs::{Attrs, Color};
use crate::grid::{Grid, Pos};
use crate::size::Size;
use termwiz::escape::csi::{Cursor, Edit, EraseInDisplay, EraseInLine, Mode, Sgr};
use termwiz::escape::{Action, ControlCode, Esc, EscCode, OperatingSystemCommand};
use unicode_width::UnicodeWidthChar;

// Terminal mode flags
const MODE_CURSOR_VISIBLE: u8 = 1 << 0;
const MODE_ALTERNATE_SCREEN: u8 = 1 << 1;
#[allow(dead_code)]
const MODE_APPLICATION_CURSOR: u8 = 1 << 2;
#[allow(dead_code)]
const MODE_BRACKETED_PASTE: u8 = 1 << 3;
const MODE_AUTO_WRAP: u8 = 1 << 4;
#[allow(dead_code)]
const MODE_ORIGIN: u8 = 1 << 5;

/// Terminal screen state
#[derive(Clone)]
pub struct Screen {
    /// Primary grid
    grid: Grid,
    /// Alternate grid (for full-screen apps)
    alternate_grid: Grid,
    /// Current text attributes
    attrs: Attrs,
    /// Terminal modes
    modes: u8,
    /// Window title
    title: String,
    /// Icon name
    icon_name: String,
    /// Pending wrap (cursor at end of line)
    pending_wrap: bool,
}

impl Screen {
    /// Create a new screen
    pub fn new(rows: usize, cols: usize, scrollback: usize) -> Self {
        let size = Size::new(cols as u16, rows as u16);

        Self {
            grid: Grid::new(size, scrollback),
            alternate_grid: Grid::new(size, 0), // No scrollback for alternate
            attrs: Attrs::default(),
            modes: MODE_CURSOR_VISIBLE | MODE_AUTO_WRAP,
            title: String::new(),
            icon_name: String::new(),
            pending_wrap: false,
        }
    }

    /// Get the current grid (primary or alternate)
    fn grid(&self) -> &Grid {
        if self.mode(MODE_ALTERNATE_SCREEN) {
            &self.alternate_grid
        } else {
            &self.grid
        }
    }

    /// Get mutable current grid
    fn grid_mut(&mut self) -> &mut Grid {
        if self.mode(MODE_ALTERNATE_SCREEN) {
            &mut self.alternate_grid
        } else {
            &mut self.grid
        }
    }

    /// Get the primary grid (for rendering)
    pub fn primary_grid(&self) -> &Grid {
        &self.grid
    }

    /// Get screen size
    pub fn size(&self) -> Size {
        self.grid().size()
    }

    /// Get cursor position
    pub fn cursor_pos(&self) -> Pos {
        self.grid().pos()
    }

    /// Check if cursor is visible
    pub fn cursor_visible(&self) -> bool {
        self.mode(MODE_CURSOR_VISIBLE)
    }

    /// Get window title
    pub fn title(&self) -> &str {
        &self.title
    }

    /// Get current scrollback offset
    pub fn scrollback(&self) -> usize {
        self.grid().scrollback()
    }

    /// Set scrollback offset
    pub fn set_scrollback(&mut self, offset: usize) {
        self.grid_mut().set_scrollback(offset);
    }

    /// Scroll screen up (for user interaction)
    pub fn scroll_screen_up(&mut self, n: usize) {
        let current = self.grid().scrollback();
        self.grid_mut().set_scrollback(current + n);
    }

    /// Scroll screen down (for user interaction)
    pub fn scroll_screen_down(&mut self, n: usize) {
        let current = self.grid().scrollback();
        self.grid_mut().set_scrollback(current.saturating_sub(n));
    }

    /// Get selected text
    pub fn get_selected_text(&self, low_x: i32, low_y: i32, high_x: i32, high_y: i32) -> String {
        self.grid().get_selected_text(low_x, low_y, high_x, high_y)
    }

    /// Check a mode flag
    fn mode(&self, mode: u8) -> bool {
        self.modes & mode != 0
    }

    /// Set a mode flag
    #[allow(dead_code)]
    fn set_mode(&mut self, mode: u8) {
        self.modes |= mode;
    }

    /// Clear a mode flag
    #[allow(dead_code)]
    fn clear_mode(&mut self, mode: u8) {
        self.modes &= !mode;
    }

    /// Resize the screen
    pub fn resize(&mut self, rows: usize, cols: usize) {
        let size = Size::new(cols as u16, rows as u16);
        self.grid.resize(size);
        self.alternate_grid.resize(size);
    }

    /// Handle a termwiz action
    pub fn handle_action(&mut self, action: Action) {
        match action {
            Action::Print(c) => self.text(c),
            Action::PrintString(s) => {
                for c in s.chars() {
                    self.text(c);
                }
            }
            Action::Control(code) => self.handle_control(code),
            Action::Esc(esc) => self.handle_esc(esc),
            Action::CSI(csi) => self.handle_csi(csi),
            Action::OperatingSystemCommand(osc) => self.handle_osc(*osc),
            Action::DeviceControl(_) => {} // DCS sequences not implemented
            Action::Sixel(_) => {}         // Sixel graphics not implemented
            Action::XtGetTcap(_) => {}     // Terminal cap queries not implemented
            Action::KittyImage(_) => {}    // Kitty images not implemented
        }
    }

    /// Handle a printable character
    fn text(&mut self, c: char) {
        let char_width = c.width().unwrap_or(0);
        if char_width == 0 {
            // Combining character - append to previous cell
            // (simplified: skip for now)
            return;
        }

        let size = self.grid().size();

        // Handle pending wrap
        if self.pending_wrap {
            self.pending_wrap = false;

            // Mark current row as wrapped
            if let Some(row) = self.grid_mut().current_row_mut() {
                row.set_wrapped(true);
            }

            // Move to next line
            let pos = self.grid().pos();
            if pos.row + 1 >= size.rows {
                self.grid_mut().scroll_up(1);
            } else {
                self.grid_mut().set_row(pos.row + 1);
            }
            self.grid_mut().set_col(0);
        }

        let pos = self.grid().pos();
        let attrs = self.attrs; // Copy attrs to avoid borrow conflict

        // Write character to current cell
        if let Some(row) = self.grid_mut().drawing_row_mut(pos.row) {
            if let Some(cell) = row.get_mut(pos.col) {
                cell.set_text(c.to_string());
                cell.set_attrs(attrs);
            }

            // Handle wide characters
            if char_width == 2 && pos.col + 1 < size.cols {
                if let Some(next_cell) = row.get_mut(pos.col + 1) {
                    next_cell.set_wide_continuation();
                    next_cell.set_attrs(attrs);
                }
            }
        }

        // Move cursor
        let new_col = pos.col + char_width as u16;
        if new_col >= size.cols {
            if self.mode(MODE_AUTO_WRAP) {
                self.pending_wrap = true;
                self.grid_mut().set_col(size.cols - 1);
            }
        } else {
            self.grid_mut().set_col(new_col);
        }
    }

    /// Handle control codes
    fn handle_control(&mut self, code: ControlCode) {
        match code {
            ControlCode::Bell => {} // Could trigger bell callback
            ControlCode::Backspace => {
                let pos = self.grid().pos();
                if pos.col > 0 {
                    self.grid_mut().set_col(pos.col - 1);
                }
                self.pending_wrap = false;
            }
            ControlCode::HorizontalTab => {
                let pos = self.grid().pos();
                let next_tab = ((pos.col / 8) + 1) * 8;
                let size = self.grid().size();
                self.grid_mut().set_col(next_tab.min(size.cols - 1));
                self.pending_wrap = false;
            }
            ControlCode::LineFeed | ControlCode::VerticalTab | ControlCode::FormFeed => {
                let pos = self.grid().pos();
                let size = self.grid().size();
                if pos.row + 1 >= size.rows {
                    self.grid_mut().scroll_up(1);
                } else {
                    self.grid_mut().set_row(pos.row + 1);
                }
                self.pending_wrap = false;
            }
            ControlCode::CarriageReturn => {
                self.grid_mut().set_col(0);
                self.pending_wrap = false;
            }
            _ => {}
        }
    }

    /// Handle escape sequences
    fn handle_esc(&mut self, esc: Esc) {
        match esc {
            Esc::Code(EscCode::DecSaveCursorPosition) => {
                self.grid_mut().save_pos();
            }
            Esc::Code(EscCode::DecRestoreCursorPosition) => {
                self.grid_mut().restore_pos();
            }
            Esc::Code(EscCode::ReverseIndex) => {
                // Move cursor up, scrolling if needed
                let pos = self.grid().pos();
                if pos.row == 0 {
                    self.grid_mut().scroll_down(1);
                } else {
                    self.grid_mut().set_row(pos.row - 1);
                }
            }
            Esc::Code(EscCode::Index) => {
                // Move cursor down, scrolling if needed
                let pos = self.grid().pos();
                let size = self.grid().size();
                if pos.row + 1 >= size.rows {
                    self.grid_mut().scroll_up(1);
                } else {
                    self.grid_mut().set_row(pos.row + 1);
                }
            }
            Esc::Code(EscCode::NextLine) => {
                // Like Index but also carriage return
                let pos = self.grid().pos();
                let size = self.grid().size();
                if pos.row + 1 >= size.rows {
                    self.grid_mut().scroll_up(1);
                } else {
                    self.grid_mut().set_row(pos.row + 1);
                }
                self.grid_mut().set_col(0);
            }
            Esc::Code(EscCode::FullReset) => {
                self.grid_mut().clear();
                self.grid_mut().set_pos(Pos::new(0, 0));
                self.attrs = Attrs::default();
                self.modes = MODE_CURSOR_VISIBLE | MODE_AUTO_WRAP;
            }
            _ => {}
        }
    }

    /// Handle CSI sequences
    fn handle_csi(&mut self, csi: termwiz::escape::csi::CSI) {
        use termwiz::escape::csi::CSI;

        match csi {
            CSI::Cursor(cursor) => self.handle_cursor(cursor),
            CSI::Edit(edit) => self.handle_edit(edit),
            CSI::Sgr(sgr) => self.handle_sgr(sgr),
            CSI::Mode(mode) => self.handle_mode(mode),
            CSI::Window(_) => {}   // Window manipulation not implemented
            CSI::Keyboard(_) => {} // Keyboard modes not implemented
            CSI::Mouse(_) => {}    // Mouse reporting not implemented
            CSI::Device(_) => {}   // Device queries not implemented
            _ => {}
        }
    }

    /// Handle cursor movement
    fn handle_cursor(&mut self, cursor: Cursor) {
        let size = self.grid().size();
        let pos = self.grid().pos();

        match cursor {
            Cursor::Position { line, col } => {
                let row = line.as_zero_based().min(size.rows.saturating_sub(1) as u32) as u16;
                let col = col.as_zero_based().min(size.cols.saturating_sub(1) as u32) as u16;
                self.grid_mut().set_pos(Pos::new(col, row));
            }
            Cursor::Up(n) => {
                let new_row = pos.row.saturating_sub(n as u16);
                self.grid_mut().set_row(new_row);
            }
            Cursor::Down(n) => {
                let new_row = (pos.row + n as u16).min(size.rows - 1);
                self.grid_mut().set_row(new_row);
            }
            Cursor::Left(n) => {
                let new_col = pos.col.saturating_sub(n as u16);
                self.grid_mut().set_col(new_col);
            }
            Cursor::Right(n) => {
                let new_col = (pos.col + n as u16).min(size.cols - 1);
                self.grid_mut().set_col(new_col);
            }
            Cursor::CharacterAbsolute(col) => {
                let col = col.as_zero_based().min(size.cols.saturating_sub(1) as u32) as u16;
                self.grid_mut().set_col(col);
            }
            Cursor::NextLine(n) => {
                let new_row = (pos.row + n as u16).min(size.rows - 1);
                self.grid_mut().set_pos(Pos::new(0, new_row));
            }
            Cursor::PrecedingLine(n) => {
                let new_row = pos.row.saturating_sub(n as u16);
                self.grid_mut().set_pos(Pos::new(0, new_row));
            }
            Cursor::SaveCursor => {
                self.grid_mut().save_pos();
            }
            Cursor::RestoreCursor => {
                self.grid_mut().restore_pos();
            }
            _ => {}
        }
        self.pending_wrap = false;
    }

    /// Handle edit operations
    fn handle_edit(&mut self, edit: Edit) {
        let size = self.grid().size();
        let pos = self.grid().pos();

        match edit {
            Edit::EraseInDisplay(mode) => match mode {
                EraseInDisplay::EraseToEndOfDisplay => {
                    self.grid_mut().clear_below();
                }
                EraseInDisplay::EraseToStartOfDisplay => {
                    self.grid_mut().clear_above();
                }
                EraseInDisplay::EraseDisplay => {
                    self.grid_mut().clear();
                }
                EraseInDisplay::EraseScrollback => {
                    // Clear scrollback - not implemented yet
                }
            },
            Edit::EraseInLine(mode) => {
                if let Some(row) = self.grid_mut().drawing_row_mut(pos.row) {
                    match mode {
                        EraseInLine::EraseToEndOfLine => {
                            row.erase(pos.col, size.cols);
                        }
                        EraseInLine::EraseToStartOfLine => {
                            row.erase(0, pos.col + 1);
                        }
                        EraseInLine::EraseLine => {
                            row.clear();
                        }
                    }
                }
            }
            Edit::InsertCharacter(n) => {
                if let Some(row) = self.grid_mut().drawing_row_mut(pos.row) {
                    for _ in 0..n {
                        row.insert(pos.col, Default::default());
                    }
                }
            }
            Edit::DeleteCharacter(n) => {
                if let Some(row) = self.grid_mut().drawing_row_mut(pos.row) {
                    for _ in 0..n {
                        row.remove(pos.col);
                    }
                }
            }
            Edit::EraseCharacter(n) => {
                if let Some(row) = self.grid_mut().drawing_row_mut(pos.row) {
                    row.erase(pos.col, pos.col + n as u16);
                }
            }
            Edit::InsertLine(n) => {
                for _ in 0..n {
                    self.grid_mut().scroll_down(1);
                }
            }
            Edit::DeleteLine(n) => {
                for _ in 0..n {
                    self.grid_mut().scroll_up(1);
                }
            }
            Edit::ScrollDown(n) => {
                self.grid_mut().scroll_down(n as usize);
            }
            Edit::ScrollUp(n) => {
                self.grid_mut().scroll_up(n as usize);
            }
            _ => {}
        }
    }

    /// Handle SGR (Select Graphic Rendition) - text styling
    fn handle_sgr(&mut self, sgr: Sgr) {
        match sgr {
            Sgr::Reset => {
                self.attrs.reset();
            }
            Sgr::Intensity(intensity) => match intensity {
                termwiz::cell::Intensity::Bold => {
                    self.attrs.set_bold(true);
                }
                termwiz::cell::Intensity::Normal => {
                    self.attrs.set_bold(false);
                }
                termwiz::cell::Intensity::Half => {
                    self.attrs.set_bold(false);
                }
            },
            Sgr::Italic(on) => {
                self.attrs.set_italic(on);
            }
            Sgr::Underline(underline) => {
                self.attrs
                    .set_underline(underline != termwiz::cell::Underline::None);
            }
            Sgr::Inverse(on) => {
                self.attrs.set_inverse(on);
            }
            Sgr::StrikeThrough(on) => {
                self.attrs.set_strikethrough(on);
            }
            Sgr::Foreground(color) => {
                self.attrs.fg = Color::from(color);
            }
            Sgr::Background(color) => {
                self.attrs.bg = Color::from(color);
            }
            _ => {}
        }
    }

    /// Handle mode changes
    fn handle_mode(&mut self, mode: Mode) {
        // Standard ANSI modes - not commonly used
        let _ = mode;
    }

    /// Handle OSC (Operating System Command)
    fn handle_osc(&mut self, osc: OperatingSystemCommand) {
        match osc {
            OperatingSystemCommand::SetWindowTitle(title)
            | OperatingSystemCommand::SetWindowTitleSun(title) => {
                self.title = title;
            }
            OperatingSystemCommand::SetIconName(name)
            | OperatingSystemCommand::SetIconNameSun(name) => {
                self.icon_name = name;
            }
            OperatingSystemCommand::SetIconNameAndWindowTitle(title) => {
                self.title = title.clone();
                self.icon_name = title;
            }
            _ => {}
        }
    }

    /// Get visible rows iterator (for rendering)
    pub fn visible_rows(&self) -> impl Iterator<Item = &crate::row::Row> {
        self.grid().visible_rows()
    }

    /// Check if in alternate screen mode
    pub fn is_alternate_screen(&self) -> bool {
        self.mode(MODE_ALTERNATE_SCREEN)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_screen_new() {
        let screen = Screen::new(24, 80, 1000);
        assert_eq!(screen.size().rows, 24);
        assert_eq!(screen.size().cols, 80);
    }

    #[test]
    fn test_screen_text() {
        let mut screen = Screen::new(24, 80, 1000);

        screen.text('H');
        screen.text('i');

        assert_eq!(screen.cursor_pos().col, 2);
    }

    #[test]
    fn test_screen_newline() {
        let mut screen = Screen::new(24, 80, 1000);

        screen.text('A');
        screen.handle_control(ControlCode::LineFeed);
        screen.handle_control(ControlCode::CarriageReturn);
        screen.text('B');

        assert_eq!(screen.cursor_pos().row, 1);
        assert_eq!(screen.cursor_pos().col, 1);
    }

    #[test]
    fn test_screen_scroll() {
        let mut screen = Screen::new(24, 80, 100);

        // Fill screen and trigger scroll
        for _ in 0..30 {
            screen.handle_control(ControlCode::LineFeed);
        }

        assert!(screen.grid().scrollback_available() > 0);
    }
}
