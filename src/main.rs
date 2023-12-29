use crate::camera::Camera;
use crate::hittable::hittables::*;
use crate::hittable::sphere::*;
use crate::vector3d::*;
use std::rc::Rc;

pub mod camera;
pub mod color;
pub mod hittable;
pub mod interval;
pub mod ray;
pub mod vector3d;

fn main() {
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

    let mut cam: Camera = Camera::new(
        16.0 / 9.0, // aspect_ratio
        400,        // image_width
    );

    cam.render(&world);
}
