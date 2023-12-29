//! Module for handling colors in the context of a raytracer.

use crate::interval::Interval;
use crate::vector3d::Vector3D;
use std::io::{self, Write};

/// Alias for a color represented as a 3D vector.
pub type Color = Vector3D;

impl Color {
    /// Writes the color components to the specified output.
    ///
    /// # Arguments
    ///
    /// * `out`               - The output stream where the color components will be written.
    /// * `samples_per_pixel` - Count of overlayed rays that the color should be normalized to.
    ///
    /// # Returns
    ///
    /// Returns an `io::Result<()>` indicating the success or failure of the write operation.
    pub fn write<W: Write>(self, out: &mut W, samples_per_pixel: u16) -> io::Result<()> {
        let mut r = self.x();
        let mut g = self.y();
        let mut b = self.z();

        // Divide the color by the number of samples
        let scale = 1.0 / f64::from(samples_per_pixel);
        r *= scale;
        g *= scale;
        b *= scale;

        // Apply the linear to gamma transform
        r = Self::linear_to_gamma(r);
        g = Self::linear_to_gamma(g);
        b = Self::linear_to_gamma(b);

        // Write the translated [0, 255] value of each color component.
        let interval = Interval::new(0.000, 0.999);
        write!(
            out,
            "{} {} {}\n",
            (255.999 * interval.clamp(r)) as u8,
            (255.999 * interval.clamp(g)) as u8,
            (255.999 * interval.clamp(b)) as u8,
        )
    }

    /// Convert from linear color space into gamma color space.
    fn linear_to_gamma(linear_component: f64) -> f64 {
        linear_component.sqrt()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    #[should_panic]
    fn todo() {
        panic!("TODO: Do color tests");
    }
}
