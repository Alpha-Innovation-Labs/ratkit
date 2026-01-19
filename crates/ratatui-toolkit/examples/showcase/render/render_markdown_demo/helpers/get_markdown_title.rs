//! Get markdown panel title based on state.

/// Get the title for the markdown panel based on selection state.
///
/// # Arguments
///
/// * `selection_active` - Whether text selection is currently active.
///
/// # Returns
///
/// The appropriate title string.
pub fn get_markdown_title(selection_active: bool) -> &'static str {
    if selection_active {
        " Markdown Renderer (Selection Mode - y to copy, Esc to exit) "
    } else {
        " Markdown Renderer "
    }
}
