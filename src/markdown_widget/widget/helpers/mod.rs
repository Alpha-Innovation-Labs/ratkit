//! Helper functions for MarkdownWidget.

mod apply_selection_highlighting;
mod filter;

pub use apply_selection_highlighting::apply_selection_highlighting;
pub use filter::{
    element_to_plain_text_for_filter, find_next_filtered_line, find_prev_filtered_line,
    get_filtered_visual_lines,
};

pub use crate::markdown_widget::extensions::selection::should_render_line as should_render_collapsed_line;
