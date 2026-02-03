//! Helper functions for MarkdownWidget.

mod apply_selection_highlighting;
mod filter;

pub use apply_selection_highlighting::apply_selection_highlighting;
pub use filter::element_to_plain_text_for_filter;

pub use crate::widgets::markdown_widget::extensions::selection::should_render_line as should_render_collapsed_line;
