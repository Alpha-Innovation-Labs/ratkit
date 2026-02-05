//! Simple hash function for content change detection.

use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

/// Simple hash function for content change detection.
///
/// # Arguments
///
/// * `content` - The content to hash
///
/// # Returns
///
/// A 64-bit hash of the content.
pub fn hash_content(content: &str) -> u64 {
    let mut hasher = DefaultHasher::new();
    content.hash(&mut hasher);
    hasher.finish()
}
