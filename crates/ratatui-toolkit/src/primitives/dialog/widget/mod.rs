//! Mutable widget variant

mod constructors;
mod traits;

use crate::primitives::dialog::Dialog;

/// A mutable wrapper around Dialog for widget rendering
///
/// This struct provides a mutable reference to Dialog for rendering purposes.
#[allow(dead_code)]
pub struct DialogWidget<'a> {
    /// Mutable reference to the dialog
    pub(crate) dialog: &'a mut Dialog<'a>,
}
