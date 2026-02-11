//! VT100 terminal emulation extracted from mprocs.

pub mod io;
pub mod protocol;
pub mod ratatui_render;
pub mod vt100;

pub use io::write_screen_diff;
pub use protocol::CursorStyle;
pub use ratatui_render::render_screen;
pub use vt100::{
  attrs, cell, grid, parser, row, screen, screen_differ, size, Attrs,
  BorderType, BufferView, Cell, Color, Grid, Margin, MouseProtocolMode,
  Parser, Pos, Rect, Screen, ScreenDiffer, Size, VtEvent,
};
