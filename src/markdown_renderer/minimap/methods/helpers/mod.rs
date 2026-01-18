//! Helper functions for minimap methods.

mod density_pair_to_braille;
mod density_to_braille;
mod line_to_density;

pub use density_pair_to_braille::*;
pub use density_to_braille::*;
pub use line_to_density::*;

/// Braille character base (U+2800 - empty Braille pattern).
pub(crate) const BRAILLE_BASE: u32 = 0x2800;

/// Braille dot positions (bit values):
/// ```text
/// 1  8
/// 2  16
/// 4  32
/// 64 128
/// ```
pub(crate) const BRAILLE_DOTS: [u32; 8] = [0x01, 0x02, 0x04, 0x40, 0x08, 0x10, 0x20, 0x80];
