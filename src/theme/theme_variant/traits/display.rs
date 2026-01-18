//! Display trait implementation for [`ThemeVariant`].

use std::fmt;

use crate::theme::ThemeVariant;

impl fmt::Display for ThemeVariant {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ThemeVariant::Dark => write!(f, "dark"),
            ThemeVariant::Light => write!(f, "light"),
        }
    }
}
