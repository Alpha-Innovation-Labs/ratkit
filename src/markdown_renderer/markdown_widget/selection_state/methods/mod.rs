//! Methods for SelectionState.

mod enter;
mod exit;
mod get_selected_text;
mod get_selection;
mod is_in_selection;
mod update_cursor;

pub use enter::*;
pub use exit::*;
pub use get_selected_text::*;
pub use get_selection::*;
pub use is_in_selection::*;
pub use update_cursor::*;
