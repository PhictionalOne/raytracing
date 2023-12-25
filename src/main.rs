use crate::color::{write_color, Color};
use crate::ray::*;
use crate::vector3d::*;
use std::io::{self, stderr, Write};

pub mod color;
pub mod ray;
pub mod vector3d;

fn ray_color(r: Ray) -> Color {
    let unit_direction: Vector3D = r.direction().unit_vector();
    let a: f64 = 0.5 * unit_direction.y() + 1.0;
    (1.0 - a) * Color::with_values(1.0, 1.0, 1.0) + a * Color::with_values(0.5, 0.7, 1.0)
}

fn main() {
    // Image

    let aspect_ratio: f64 = 16.0 / 9.0;
    let image_width: u16 = 400;

    // Calculate the image height, and ensure that it's at least 1
    let mut image_height: u16 = (f64::from(image_width) / aspect_ratio) as u16;
    image_height = if image_height < 1 { 1 } else { image_height };

    // Camera

    let focal_length: f64 = 1.0;
    let viewport_height: f64 = 2.0;
    let viewport_width: f64 = viewport_height * f64::from(image_width) / f64::from(image_height);
    let camera_center: Point3D = Point3D::new();

    // Calculate the vectors across the horizontal and down the vertical viewport edges.
    let viewport_u: Vector3D = Vector3D::with_values(viewport_width, 0.0, 0.0);
    let viewport_v: Vector3D = Vector3D::with_values(0.0, -viewport_width, 0.0);

    // Calculate the horizontal and vertical delta vectors from pixel to pixel
    let pixel_Δu: Vector3D = viewport_u / f64::from(image_width);
    let pixel_Δv: Vector3D = viewport_v / f64::from(image_height);

    // Calculate the location of the upper left pixel
    let viewport_upper_left: Point3D = camera_center
        - Vector3D::with_values(0.0, 0.0, focal_length)
        - viewport_u / 2.0
        - viewport_v / 2.0;
    let pixel00_loc: Point3D = viewport_upper_left + 0.5 * (pixel_Δu + pixel_Δv);

    let mut pixel_color: Color;
    let mut buffer = Vec::new();

    // Render
    write!(buffer, "P3\n{} {}\n255\n", image_width, image_height);

    for j in 0..image_height {
        eprintln!("Scanlines remaining: {}", image_height - j);
        eprint!("\x1B[2J\x1B[1;1H"); // Clear output

        for i in 0..image_width {
            let pixel_center: Point3D =
                pixel00_loc + (f64::from(i) * pixel_Δu) + (f64::from(j) * pixel_Δv);
            let ray_direction: Vector3D = pixel_center - camera_center;
            let r: Ray = Ray::create(camera_center, ray_direction);

            let pixel_color: Color = ray_color(r);

            write_color(&mut buffer, pixel_color).expect("Failed to write color");
        }
    }
    println!("{}", String::from_utf8_lossy(&buffer));
    eprintln!("Done.");
}
