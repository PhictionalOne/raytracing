//! Module for handling colors in the context of a raytracer.

use crate::vector3d::Vector3D;
use std::io::{self, Write};

/// Alias for a color represented as a 3D vector.
pub type Color = Vector3D;

impl Color {
    /// Writes the color components to the specified output.
    ///
    /// # Arguments
    ///
    /// * `out` - The output stream where the color components will be written.
    ///
    /// # Returns
    ///
    /// Returns an `io::Result<()>` indicating the success or failure of the write operation.
    pub fn write<W: Write>(self, out: &mut W) -> io::Result<()> {
        // Write the translated [0, 255] value of each color component.
        write!(
            out,
            "{} {} {}\n",
            (256.0 * self.x()) as u8,
            (256.0 * self.y()) as u8,
            (256.0 * self.z()) as u8,
        )
    }
}
