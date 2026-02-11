//! Scroll utilities for scrollable widgets.
//!
//! This crate provides generic scroll offset calculation algorithms for keeping
//! selected items visible and centered in scrollable containers.
//!
//! # Example
//!
//! ```rust
//! use crate::primitives::scroll::calculate_scroll_offset;
//!
//! let offset = calculate_scroll_offset(10, 5, 20);
//! assert_eq!(offset, 8);
//! ```

/// Calculate the scroll offset to keep the selected item visible and centered.
///
/// This utility function computes an appropriate scroll offset for any scrollable
/// widget, ensuring the selected item remains visible while attempting to keep
/// it centered within the visible viewport.
///
/// # Algorithm
///
/// - If all items fit in the viewport, returns 0 (no scroll needed)
/// - If the selected item is in the first half, scroll to the top
/// - If the selected item is in the last half, scroll to show the last item
/// - Otherwise, center the selected item by calculating an offset that places
///   it in the middle of the visible area
///
/// # Arguments
///
/// * `selected_index` - The index of the currently selected item (0-based)
/// * `visible_count` - The number of items visible in the viewport
/// * `total_count` - The total number of items in the list
///
/// # Returns
///
/// The scroll offset to apply (number of items to skip from the beginning).
/// Returns 0 if `total_count <= visible_count`.
///
/// # Examples
///
/// ```
/// use crate::primitives::scroll::calculate_scroll_offset;
///
/// // All items visible - no scroll needed
/// assert_eq!(calculate_scroll_offset(5, 10, 10), 0);
///
/// // Selected at start - scroll to top
/// assert_eq!(calculate_scroll_offset(2, 5, 20), 0);
///
/// // Selected at end - scroll to show last items
/// assert_eq!(calculate_scroll_offset(18, 5, 20), 15);
///
/// // Selected in middle - center it
/// assert_eq!(calculate_scroll_offset(10, 5, 20), 8);
/// ```
pub fn calculate_scroll_offset(
    selected_index: usize,
    visible_count: usize,
    total_count: usize,
) -> usize {
    if total_count <= visible_count {
        return 0;
    }

    let half_visible = visible_count / 2;

    if selected_index <= half_visible {
        0
    } else if selected_index >= total_count - half_visible {
        total_count.saturating_sub(visible_count)
    } else {
        selected_index.saturating_sub(half_visible)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_items_visible() {
        assert_eq!(calculate_scroll_offset(0, 10, 5), 0);
        assert_eq!(calculate_scroll_offset(4, 10, 5), 0);
        assert_eq!(calculate_scroll_offset(5, 10, 5), 0);
    }

    #[test]
    fn test_selected_at_start() {
        assert_eq!(calculate_scroll_offset(0, 5, 20), 0);
        assert_eq!(calculate_scroll_offset(1, 5, 20), 0);
        assert_eq!(calculate_scroll_offset(2, 5, 20), 0);
    }

    #[test]
    fn test_selected_at_end() {
        assert_eq!(calculate_scroll_offset(18, 5, 20), 15);
        assert_eq!(calculate_scroll_offset(19, 5, 20), 15);
    }

    #[test]
    fn test_selected_in_middle() {
        assert_eq!(calculate_scroll_offset(10, 5, 20), 8);
        assert_eq!(calculate_scroll_offset(9, 5, 20), 7);
        assert_eq!(calculate_scroll_offset(11, 5, 20), 9);
    }

    #[test]
    fn test_edge_cases() {
        assert_eq!(calculate_scroll_offset(0, 1, 1), 0);
        assert_eq!(calculate_scroll_offset(0, 1, 100), 0);
        assert_eq!(calculate_scroll_offset(99, 1, 100), 99);
        assert_eq!(calculate_scroll_offset(0, 3, 10), 0);
        assert_eq!(calculate_scroll_offset(1, 3, 10), 0);
        assert_eq!(calculate_scroll_offset(2, 3, 10), 0);
        assert_eq!(calculate_scroll_offset(3, 3, 10), 1);
        assert_eq!(calculate_scroll_offset(6, 3, 10), 4);
        assert_eq!(calculate_scroll_offset(7, 3, 10), 5);
        assert_eq!(calculate_scroll_offset(8, 3, 10), 7);
        assert_eq!(calculate_scroll_offset(9, 3, 10), 7);
    }
}
