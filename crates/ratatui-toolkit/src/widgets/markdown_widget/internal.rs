//! Internal re-exports for cross-module use within markdown_widget.
//!
//! This module provides `pub(crate)` re-exports for functions and types
//! that need to be shared across different parts of the markdown_widget
//! module but should not be part of the public API.

// Foundation helpers
pub(crate) use super::foundation::helpers::{
    element_to_plain_text, get_line_at_position, hash_content, is_in_area,
};

// Selection handlers (internal use for widget)
pub(crate) use super::extensions::selection::handlers::{
    handle_mouse_event as selection_handle_mouse_event,
    handle_mouse_event_with_double_click as selection_handle_mouse_event_with_double_click,
};

// Selection helpers
pub(crate) use super::extensions::selection::helpers::{
    handle_click as selection_handle_click, should_render_line as selection_should_render_line,
};
