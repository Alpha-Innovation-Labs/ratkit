//! Render functions for demo tabs.

mod render_code_diff_demo;
mod render_dialogs_demo;
mod render_file_tree_demo;
mod render_markdown_demo;
mod render_scrollbar_demo;
mod render_statusline_demo;
mod render_terminal_demo;
mod render_theme_picker;
mod render_tree_demo;

pub use render_code_diff_demo::render_code_diff_demo;
pub use render_dialogs_demo::render_dialogs_demo;
pub use render_file_tree_demo::render_file_tree_demo;
pub use render_markdown_demo::render_markdown_demo;
pub use render_scrollbar_demo::render_scrollbar_demo;
pub use render_statusline_demo::render_statusline_demo;
pub use render_terminal_demo::render_terminal_demo;
pub use render_theme_picker::{get_filtered_themes, render_theme_picker};
pub use render_tree_demo::render_tree_demo;
