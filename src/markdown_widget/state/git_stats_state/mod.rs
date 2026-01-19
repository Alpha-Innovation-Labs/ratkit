//! Git stats state for markdown widget.
//!
//! Tracks git diff statistics for the markdown source file.

pub mod constructors;
pub mod helpers;
pub mod methods;
pub mod traits;

pub use constructors::*;
pub use methods::*;
pub use traits::*;

use crate::markdown_widget::foundation::types::GitStats;
use std::time::Instant;

/// Git stats state for markdown source files.
///
/// Tracks additions, modifications, and deletions from git diff.
#[derive(Debug, Clone)]
pub struct GitStatsState {
    /// Whether to show git stats in the statusline.
    show: bool,
    /// Cached git stats for the source file.
    cache: Option<GitStats>,
    /// Last time git stats were updated.
    last_update: Option<Instant>,
}
