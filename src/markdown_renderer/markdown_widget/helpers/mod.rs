//! Helper functions for markdown widget.

mod element_kind_to_string;
mod element_to_plain_text;
mod get_line_at_position;
mod handle_click;
mod hash_content;
mod is_in_area;
mod should_render_line;

pub(crate) use element_kind_to_string::element_kind_to_string;
pub(crate) use element_to_plain_text::element_to_plain_text;
pub(crate) use get_line_at_position::get_line_at_position;
pub(crate) use handle_click::handle_click;
pub(crate) use hash_content::hash_content;
pub(crate) use is_in_area::is_in_area;
pub(crate) use should_render_line::should_render_line;
