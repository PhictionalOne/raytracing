use crate::vector3d::Vector3D;
use std::io::{self, Write};

pub type Color = Vector3D;

pub fn write_color<W: Write>(out: &mut W, pixel_color: Color) -> io::Result<()> {
    // Write the translated [0, 255] value of each color component.
    write!(
        out,
        "{} {} {}\n",
        (256.0 * pixel_color.x()) as u8,
        (256.0 * pixel_color.y()) as u8,
        (256.0 * pixel_color.z()) as u8,
    )
}
