//! Methods for [`ClickableScrollbar`] and [`ClickableScrollbarState`].
//!
//! This module organizes methods into focused files for better maintainability.

mod mouse_handler;
mod mouse_handler_helpers;
mod mouse_handler_trait;
mod scroll;
mod scroll_trait;
mod stateful_widget;
mod stateful_widget_trait;

pub use mouse_handler_trait::ClickableScrollbarStateMouseExt;
pub use scroll_trait::ClickableScrollbarStateScrollExt;
pub use stateful_widget_trait::ClickableScrollbarStatefulWidgetExt;
