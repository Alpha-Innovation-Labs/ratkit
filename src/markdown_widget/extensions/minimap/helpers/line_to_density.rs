//! Convert line content to a density value.

/// Convert line content to a density value based on character count.
///
/// # Arguments
///
/// * `line` - The text line to analyze
/// * `max_width` - The maximum expected line width for normalization
///
/// # Returns
///
/// A density value from 0.0 (empty) to 1.0 (full width).
pub fn line_to_density(line: &str, max_width: usize) -> f32 {
    if max_width == 0 {
        return 0.0;
    }

    let char_count = line.chars().filter(|c| !c.is_whitespace()).count();
    (char_count as f32 / max_width as f32).min(1.0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_line_to_density() {
        assert_eq!(line_to_density("", 80), 0.0);
        assert_eq!(line_to_density("hello", 10), 0.5);
        assert_eq!(line_to_density("  hello  ", 10), 0.5); // whitespace ignored
    }
}
