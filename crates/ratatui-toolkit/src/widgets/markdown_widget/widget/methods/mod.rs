//! Methods for MarkdownWidget.

mod calculate_scrollbar_area;
mod calculate_toc_area;
mod git_stats;
mod handle_key_event;
mod handle_mouse_event;
mod handle_toc_click;
mod handle_toc_hover;
mod is_resizing;
mod last_double_click;
mod mode;
mod render_statusline;
mod sync_state_back;

pub use sync_state_back::WidgetStateSync;
