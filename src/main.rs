pub mod color;
pub mod vector3d;

fn main() {
    use crate::color::{write_color, Color};
    use crate::vector3d::*;
    use std::io::{self, stderr, Write};

    // Image
    let image_width: u16 = 256;
    let image_height: u16 = 256;

    let mut pixel_color: Color;
    let mut buffer = Vec::new();

    // Render
    write!(buffer, "P3\n{} {}\n255\n", image_width, image_height);

    for j in 0..image_height {
        eprintln!("\rScanlines remaining: {}", image_height - j);
        eprint!("\x1B[2J\x1B[1;1H"); // Clear output

        for i in 0..image_width {
            pixel_color = Color::with_values(
                f64::from(i) / f64::from(image_width - 1),
                f64::from(j) / f64::from(image_height - 1),
                0.0,
            );
            write_color(&mut buffer, pixel_color).expect("Failed to write color");
        }
    }
    println!("{}", String::from_utf8_lossy(&buffer));
    eprintln!("Done.");
}
