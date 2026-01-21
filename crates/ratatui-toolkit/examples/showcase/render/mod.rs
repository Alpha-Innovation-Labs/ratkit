//! Render functions for demo tabs.

mod render_ai_chat_demo;
mod render_code_diff_demo;
mod render_markdown_demo;
mod render_split_layout_grid_demo;
mod render_terminal_demo;
mod render_theme_picker;
mod render_trees_demo;

pub use render_ai_chat_demo::render_ai_chat_demo;
pub use render_code_diff_demo::render_code_diff_demo;
pub use render_markdown_demo::render_markdown_demo;
pub use render_split_layout_grid_demo::render_split_layout_grid_demo;
pub use render_terminal_demo::render_terminal_demo;
pub use render_theme_picker::{get_filtered_themes, render_theme_picker};
pub use render_trees_demo::render_trees_demo;
