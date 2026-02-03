//! Cache state for markdown widget.
//!
//! Manages parsed and rendered markdown caches for efficient rendering.

pub mod constructors;
pub mod methods;
pub mod parsed_cache;
pub mod render_cache;
pub mod traits;

pub use constructors::*;
pub use methods::*;
pub use parsed_cache::ParsedCache;
pub use render_cache::RenderCache;
pub use traits::*;

/// Cache state for markdown rendering.
///
/// Maintains two levels of caching:
/// - Parsed cache: Content-dependent, width-independent
/// - Render cache: Content and width-dependent
#[derive(Debug, Clone)]
pub struct CacheState {
    /// Cache for parsed markdown elements (doesn't depend on width).
    pub(crate) parsed: Option<ParsedCache>,
    /// Cache for rendered lines (depends on width).
    pub(crate) render: Option<RenderCache>,
}

impl CacheState {
    /// Clear the render cache (e.g., when exiting filter mode).
    pub fn clear_render_cache(&mut self) {
        self.render = None;
    }
}
