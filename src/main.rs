use crate::color::Color;
use crate::hittable::hittables::*;
use crate::hittable::sphere::*;
use crate::hittable::{HitRecord, Hittable};
use crate::interval::Interval;
use crate::ray::*;
use crate::vector3d::*;
use std::io::{self, stderr, Write};
use std::rc::Rc;

pub mod color;
pub mod hittable;
pub mod interval;
pub mod ray;
pub mod vector3d;

fn ray_color(r: &Ray, world: &HittableList) -> Color {
    let mut rec = HitRecord::default();
    if world.hit(r, Interval::new(0.0, f64::INFINITY), &mut rec) {
        return 0.5 * (rec.normal() + Color::with_values(1.0, 1.0, 1.0));
    }

    let unit_direction: &Vector3D = &r.direction().unit_vector();
    let a: f64 = 0.5 * unit_direction.y() + 1.0;
    (1.0 - a) * Color::with_values(1.0, 1.0, 1.0) + a * Color::with_values(0.5, 0.7, 1.0)
}

fn main() {
    // Image

    let aspect_ratio: f64 = 16.0 / 9.0;
    let image_width: u16 = 400; // 1080;

    // Calculate the image height, and ensure that it's at least 1
    let mut image_height: u16 = (f64::from(image_width) / aspect_ratio) as u16;
    image_height = if image_height < 1 { 1 } else { image_height };

    // World

    let mut world: HittableList = HittableList::new();

    world.push(Rc::new(Sphere::new(
        Point3D::with_values(0.0, 0.0, -1.0),
        0.5,
    )));
    world.push(Rc::new(Sphere::new(
        Point3D::with_values(0.0, -100.5, -1.0),
        100.0,
    )));

    // Camera

    let focal_length: f64 = 1.0;
    let viewport_height: f64 = 2.0;
    let viewport_width: f64 = viewport_height * f64::from(image_width) / f64::from(image_height);
    let camera_center: Point3D = Point3D::new();

    // Calculate the vectors across the horizontal and down the vertical viewport edges.
    let viewport_u: Vector3D = Vector3D::with_values(viewport_width, 0.0, 0.0);
    let viewport_v: Vector3D = Vector3D::with_values(0.0, -viewport_height, 0.0);

    // Calculate the horizontal and vertical delta vectors from pixel to pixel
    let pixel_Δu: Vector3D = viewport_u / f64::from(image_width);
    let pixel_Δv: Vector3D = viewport_v / f64::from(image_height);

    // Calculate the location of the upper left pixel
    let viewport_upper_left: Point3D = camera_center
        - Vector3D::with_values(0.0, 0.0, focal_length)
        - viewport_u / 2.0
        - viewport_v / 2.0;
    let pixel00_loc: Point3D = viewport_upper_left + 0.5 * (pixel_Δu + pixel_Δv);

    let mut buffer = Vec::new();

    // Render
    write!(buffer, "P3\n{} {}\n255\n", image_width, image_height).expect("Failed to write header");

    for j in 0..image_height {
        eprintln!("Scanlines remaining: {}", image_height - j);
        eprint!("\x1B[2J\x1B[1;1H"); // Clear output

        for i in 0..image_width {
            let _pixel_center: Point3D =
                pixel00_loc + (f64::from(i) * pixel_Δu) + (f64::from(j) * pixel_Δv);
            let _ray_direction: Vector3D = _pixel_center - camera_center;
            let _r: Ray = Ray::create(camera_center, _ray_direction);

            ray_color(&_r, &world)
                .write(&mut buffer)
                .expect("Failed to write color");
        }
    }
    println!("{}", String::from_utf8_lossy(&buffer));
    eprintln!("Done.");
}
