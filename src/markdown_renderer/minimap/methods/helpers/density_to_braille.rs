//! Convert a density value to a Braille character.

use super::{BRAILLE_BASE, BRAILLE_DOTS};

/// Convert a density value (0.0 to 1.0) to a Braille character.
///
/// Higher density values result in more filled Braille dots.
///
/// # Arguments
///
/// * `density` - A value from 0.0 (empty) to 1.0 (full)
///
/// # Returns
///
/// A single Braille character representing the density.
pub fn density_to_braille(density: f32) -> char {
    let density = density.clamp(0.0, 1.0);
    let dots = (density * 8.0).round() as usize;

    let mut code = BRAILLE_BASE;
    for i in 0..dots.min(8) {
        code |= BRAILLE_DOTS[i];
    }

    char::from_u32(code).unwrap_or(' ')
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_density_to_braille_empty() {
        let ch = density_to_braille(0.0);
        assert_eq!(ch, '\u{2800}'); // Empty Braille
    }

    #[test]
    fn test_density_to_braille_full() {
        let ch = density_to_braille(1.0);
        assert_eq!(ch, '\u{28FF}'); // Full Braille
    }
}
