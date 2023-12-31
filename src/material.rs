use color::Color;
use hittable::HitRecord;
use ray::Ray;
use vector3d::{Point3D, Vector3D};
use std::rc::Rc;

pub trait Material {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> (bool, Color, Ray);
}

#[derive(PartialEq, Debug, Clone)]
pub struct Lambertian {
    albedo: Color,
}

impl Lambertian {
    pub const default: Self = Self::new(Color::new());

    pub const fn new(a: Color) -> Self {
        Lambertian { albedo: a }
    }
}

impl Material for Lambertian {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> (bool, Color, Ray) {
        let _scatter_direction = rec.clone().normal() + Vector3D::random_unit_vector();
        let _scattered = Ray::create(rec.clone().p(), _scatter_direction);
        let _attenuation = self.albedo;

        (true, _attenuation, _scattered)
    }
}
