//! Convert a pair of density values to a Braille character.

use super::BRAILLE_BASE;

/// Convert a pair of density values to a Braille character.
///
/// This allows representing two columns of density in one character.
///
/// # Arguments
///
/// * `left_density` - Density for the left column (0.0 to 1.0)
/// * `right_density` - Density for the right column (0.0 to 1.0)
///
/// # Returns
///
/// A single Braille character representing both densities.
pub fn density_pair_to_braille(left_density: f32, right_density: f32) -> char {
    let left = left_density.clamp(0.0, 1.0);
    let right = right_density.clamp(0.0, 1.0);

    let left_dots = (left * 4.0).round() as usize;
    let right_dots = (right * 4.0).round() as usize;

    let mut code = BRAILLE_BASE;

    // Left column dots (positions 0, 1, 2, 3 -> bits 1, 2, 4, 64)
    let left_bits = [0x01, 0x02, 0x04, 0x40];
    for i in 0..left_dots.min(4) {
        code |= left_bits[i];
    }

    // Right column dots (positions 4, 5, 6, 7 -> bits 8, 16, 32, 128)
    let right_bits = [0x08, 0x10, 0x20, 0x80];
    for i in 0..right_dots.min(4) {
        code |= right_bits[i];
    }

    char::from_u32(code).unwrap_or(' ')
}
