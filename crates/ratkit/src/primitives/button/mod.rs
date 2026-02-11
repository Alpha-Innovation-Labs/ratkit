//! Button component for terminal UI applications.

pub mod render;

pub use render::render_title_with_buttons;
pub use widget::Button;

mod widget;
