//! Line width calculation for TOC entries.

/// Calculate line width based on heading level.
///
/// # Arguments
///
/// * `canvas_width` - Total canvas width.
/// * `level` - Heading level (1-6).
///
/// # Returns
///
/// The line width in canvas units.
pub(crate) fn calculate_line_width(canvas_width: f64, level: u8) -> f64 {
    let width_ratio = match level {
        1 => 1.0,
        2 => 0.80,
        3 => 0.60,
        4 => 0.45,
        5 => 0.35,
        _ => 0.25,
    };
    (canvas_width * width_ratio).max(2.0)
}
