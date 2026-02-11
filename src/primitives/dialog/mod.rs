//! Dialog component
//!
//! Provides modal dialog widgets with customizable buttons and styles.

pub mod builders;
pub mod methods;
pub mod render;
pub mod types;

pub use render::DialogWidget;
pub use types::{Dialog, DialogState, DialogType};
