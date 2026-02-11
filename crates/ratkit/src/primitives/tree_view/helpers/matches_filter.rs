//! Helper function to check if a node name matches a filter.

/// Checks if a node name matches the filter (case-insensitive contains).
///
/// # Arguments
///
/// * `name` - The name to check against the filter
/// * `filter` - The filter text, or None to match everything
///
/// # Returns
///
/// `true` if the name matches the filter or no filter is set, `false` otherwise.
///
/// # Example
///
/// ```rust
/// use ratatui_toolkit::tree_view::helpers::matches_filter;
///
/// assert!(matches_filter("MyFile.rs", &None));
/// assert!(matches_filter("MyFile.rs", &Some("".to_string())));
/// assert!(matches_filter("MyFile.rs", &Some("file".to_string())));
/// assert!(matches_filter("MyFile.rs", &Some("FILE".to_string())));
/// assert!(!matches_filter("MyFile.rs", &Some("foo".to_string())));
/// ```
#[must_use]
pub fn matches_filter(name: &str, filter: &Option<String>) -> bool {
    match filter {
        None => true,
        Some(f) if f.is_empty() => true,
        Some(f) => name.to_lowercase().contains(&f.to_lowercase()),
    }
}
