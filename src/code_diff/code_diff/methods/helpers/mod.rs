//! Helper functions for rendering the diff widget.

mod build_aligned_lines;
mod render_diff_content;
mod render_header;
mod render_hunk_header;
mod render_line;
mod render_line_number;
mod render_side_by_side;
mod render_sidebar;

pub use build_aligned_lines::build_aligned_lines;
pub use render_diff_content::render_diff_content;
pub use render_header::render_header;
pub use render_hunk_header::render_hunk_header;
pub use render_line::render_line;
pub use render_line_number::render_line_number;
pub use render_side_by_side::render_side_by_side;
pub use render_sidebar::render_sidebar;
