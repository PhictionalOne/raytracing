use raytracer::*;
use raytracer::hittable::hittables::*;
use raytracer::hittable::sphere::*;
use raytracer::hittable::*;
use raytracer::vector3d::*;
use raytracer::camera::*;
use std::rc::Rc;

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
        100,        // samples_per_pixel
        50,         // max_depth
    );

    cam.render(&world);
}
