//! Pane identifier type

use std::fmt;

/// Unique identifier for a pane
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PaneId(u64);

static NEXT_ID: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);

impl PaneId {
    /// Create a new unique PaneId
    pub fn new(_name: &str) -> Self {
        // For now, use a simple hash of the name + counter
        // The name parameter is for future use when we want to associate names with IDs
        let id = NEXT_ID.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        Self(id)
    }

    /// Get the raw ID value
    pub fn raw(&self) -> u64 {
        self.0
    }
}

impl fmt::Display for PaneId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "PaneId({})", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pane_id_creation() {
        let id1 = PaneId::new("test");
        let id2 = PaneId::new("test");

        // IDs should be unique even with same name
        assert_ne!(id1, id2);
    }

    #[test]
    fn test_pane_id_equality() {
        let id = PaneId::new("test");
        assert_eq!(id, id);
    }

    #[test]
    fn test_pane_id_display() {
        let id = PaneId::new("test");
        let display = format!("{}", id);
        assert!(display.starts_with("PaneId("));
    }
}
