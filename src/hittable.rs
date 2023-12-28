//! Module for representing hittable objects in the context of a raytracer.

use crate::interval::{Interval, EMPTY, UNIVERSE};
use crate::ray::Ray;
use crate::vector3d::{Point3D, Vector3D};

/// Represents the information recorded when a ray hits an object.
#[derive(PartialEq, Debug, Clone, Copy)]
pub struct HitRecord {
    p: Point3D,
    normal: Vector3D,
    t: f64,
    front_face: bool,
}

impl HitRecord {
    /// Creates a new `HitRecord` with default values.
    pub fn default() -> Self {
        HitRecord {
            p: Point3D::new(),
            normal: Vector3D::new(),
            t: 0.0,
            front_face: false,
        }
    }

    pub fn new(p: Point3D, normal: Vector3D, t: f64, front_face: bool) -> Self {
        HitRecord {
            p,
            normal,
            t,
            front_face,
        }
    }

    pub fn p(self) -> Point3D {
        self.p
    }

    pub fn normal(self) -> Vector3D {
        self.normal
    }

    pub fn t(self) -> f64 {
        self.t
    }

    pub fn front_face(self) -> bool {
        self.front_face
    }

    /// Sets the face normal based on the given ray and outward normal.
    ///
    /// # Arguments
    ///
    /// * `r` - The incident ray.
    /// * `outward_normal` - The outward normal at the point of intersection (assumed to have unit length).
    pub fn set_face_normal(&mut self, r: Ray, outward_normal: Vector3D) {
        // debug_assert!(
        //     outward_normal.length() == 1.0,
        //     "outward_normal not of unit length!"
        // );

        self.front_face = r.direction().dot(outward_normal) < 0.0;
        if self.front_face {
            self.normal = outward_normal;
        } else {
            self.normal = -outward_normal;
        };
    }
}

pub trait Hittable {
    /// Checks if a ray intersects with the shape(s) and calculates the hit record.
    ///
    /// # Arguments
    ///
    /// * `r` - The ray to check for intersection.
    /// * `ray_tmin` - The minimum value of the parameter along the ray.
    /// * `ray_tmax` - The maximum value of the parameter along the ray.
    /// * `rec` - A mutable reference to the hit record to be populated.
    ///
    /// # Returns
    ///
    /// Returns `true` if the ray intersects with the shape(s), and the hit record is updated.
    /// Returns `false` otherwise.
    fn hit(&self, r: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool;
}

// --- HITTABLE LIST -----------------------------------------------------------

pub mod hittables {
    //! Module for handling collections of hittable objects in the context of a raytracer.

    use crate::hittable::{HitRecord, Hittable};
    use crate::interval::*;
    use crate::ray::Ray;
    use crate::vector3d::{Point3D, Vector3D};
    use std::rc::Rc;
    use std::vec::Vec;

    /// Alias for a list of hittable objects.
    pub type HittableList = Vec<Rc<dyn Hittable>>;

    /// Implementation of the `Hittable` trait for a list of hittable objects.
    impl Hittable for HittableList {
        /// Determines if the given ray intersects with any hittable object in the list.
        ///
        /// # Arguments
        ///
        /// * `r`     - The ray to be checked for intersection.
        /// * `ray_t` - The interval parameter value along the ray to consider for intersection.
        /// * `rec`   - The hit record to be updated if an intersection is found.
        ///
        /// # Returns
        ///
        /// Returns `true` if the ray intersects with any object, updating the hit record
        /// Returns `false` otherwise.
        fn hit(&self, r: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool {
            let mut temp_rec = HitRecord::default();
            let mut hit_anything = false;
            let mut closest_so_far = ray_t.max;

            for object in self {
                if object.hit(r, Interval::new(ray_t.min, closest_so_far), &mut temp_rec) {
                    hit_anything = true;
                    closest_so_far = temp_rec.t;
                    *rec = temp_rec.clone();
                }
            }

            hit_anything
        }
    }

    #[cfg(test)]
    mod tests {
        use crate::hittable::hittables::HittableList;
        use crate::hittable::sphere::Sphere;
        use crate::hittable::{HitRecord, Hittable};
        use crate::interval::*;
        use crate::ray::Ray;
        use crate::vector3d::{Point3D, Vector3D};
        use std::rc::Rc;

        #[test]
        fn list_creation() {
            let mut hittables: HittableList = HittableList::new();

            assert!(hittables.is_empty(), "New HittableList not empty");

            for i in 1..4 {
                // TODO: Add other 3D Shapes
                hittables.push(Rc::new(Sphere::default()));
                assert_eq!(hittables.len(), i);
            }
        }

        #[test]
        fn list_hit() {
            let mut hittables: HittableList = HittableList::new();

            let ray: Ray = Ray::create(Point3D::new(), Vector3D::with_values(1.0, 0.0, 0.0));
            let sphere: Sphere = Sphere::new(Point3D::with_values(2.0, 0.0, 0.0), 1.0);
            let ray_t: Interval = Interval::new(0.5, 1.5);
            let rec: &mut HitRecord = &mut HitRecord::default();

            hittables.push(Rc::new(sphere));

            assert!(hittables.hit(&ray, ray_t, rec), "Sphere in List not hit");
            assert_eq!(
                *rec,
                HitRecord::new(
                    Point3D::with_values(1.0, 0.0, 0.0),
                    Vector3D::with_values(-1.0, 0.0, 0.0),
                    1.0,
                    true
                ),
                "Hit Record not as expected"
            );

            // TODO: Add further tests for longer more diverse lists
        }
    }
}

// --- HITTABLE SHAPES ---------------------------------------------------------

// SPHERE
pub mod sphere {
    //! Module for handling spheres in the context of a raytracer.
    use crate::hittable::{HitRecord, Hittable};
    use crate::interval::*;
    use crate::ray::Ray;
    use crate::vector3d::{Point3D, Vector3D};

    /// Represents a sphere in 3D space.
    #[derive(Debug, PartialEq)]
    pub struct Sphere {
        center: Point3D,
        radius: f64,
    }

    impl Sphere {
        /// Creates a new sphere with the default parameters (center at the origin, radius 0.0).
        pub fn default() -> Self {
            Self::new(Point3D::new(), 0.0)
        }

        /// Creates a new sphere with the specified center and radius.
        ///
        /// # Arguments
        ///
        /// * `center` - The center of the sphere.
        /// * `radius` - The radius of the sphere.
        pub fn new(center: Point3D, radius: f64) -> Self {
            Sphere { center, radius }
        }

        /// Gets the center of the sphere.
        pub fn center(self) -> Point3D {
            self.center
        }

        /// Gets the radius of the sphere.
        pub fn radius(self) -> f64 {
            self.radius
        }
    }

    impl Hittable for Sphere {
        fn hit(&self, r: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool {
            let oc: Vector3D = r.origin() - self.center;
            let a: f64 = r.direction().length_squared();
            let half_b: f64 = oc.dot(r.direction());
            let c = oc.length_squared() - self.radius * self.radius;

            let discriminant: f64 = half_b * half_b - a * c;
            if discriminant < 0.0 {
                return false;
            }
            let sqrtd: f64 = discriminant.sqrt();

            // Find the nearest root that lies in the acceptable range.
            let mut root: f64 = (-half_b - sqrtd) / a;
            if !ray_t.surrounds(root) {
                root = (-half_b + sqrtd) / a;
                if !ray_t.surrounds(root) {
                    return false;
                }
            }

            rec.t = root;
            rec.p = r.at(rec.t);
            let outward_normal: Vector3D = (rec.p - self.center) / self.radius;
            rec.set_face_normal(*r, outward_normal.unit_vector());

            true
        }
    }

    #[cfg(test)]
    mod tests {
        use crate::hittable::sphere::*;
        use crate::hittable::{HitRecord, Hittable};
        use crate::interval::*;
        use crate::ray::Ray;
        use crate::vector3d::{Point3D, Vector3D};

        #[test]
        fn sphere_new() {
            let p: Point3D = Point3D::with_values(1.0, 1.0, 1.0);
            let sphere: Sphere = Sphere::new(p, 1.0);

            assert_eq!(
                Sphere::new(Point3D::with_values(0.0, 0.0, 0.0), 0.0),
                Sphere::default(),
                "Sphere at origin, with radius 0 not the default sphere"
            );
            assert_eq!(sphere.center, p, "Sphere center not point p");
            assert_eq!(sphere.radius, 1.0, "Sphere radius not 1.0");
        }

        #[test]
        fn sphere_hit_outside() {
            //      t=1.0--.    .-*****-.   ---
            //              \  *    |    *   ↑ r=1
            //       (1,0)   \*     |     *  ↓
            //   o----------->X     c     * ---
            // (0,0)        / *   (2,0)   *
            //         HIT AT  *         *
            //          (1,0)   ''*****''
            //          |        |
            //          [0.5, 1.5] =: [tmin, tmax]

            let ray: Ray = Ray::create(Point3D::new(), Vector3D::with_values(1.0, 0.0, 0.0));
            let sphere: Sphere = Sphere::new(Point3D::with_values(2.0, 0.0, 0.0), 1.0);
            let ray_t: Interval = Interval::new(0.5, 1.5);
            let rec: &mut HitRecord = &mut HitRecord::default();

            assert!(sphere.hit(&ray, ray_t, rec), "Test sphere not hit by ray");
            assert_eq!(
                *rec,
                HitRecord::new(
                    Point3D::with_values(1.0, 0.0, 0.0),
                    Vector3D::with_values(-1.0, 0.0, 0.0),
                    1.0,
                    true
                ),
                "Hit Record not as expected"
            );
        }

        #[test]
        fn sphere_hit_inside() {
            //              ------*****-.
            //          r=1 ↑  *    |    *  .--t=3
            //              ↓ *     |     */
            //   o----------->*-----c--+->X HIT AT (3,0)
            // (0,0)  (1,0)   *   (2,0)|  *     |
            //                 *       | *      |
            //                  ''*****|'       |
            //                         |        |
            //         [tmin, tmax] := [2.5, 3.5]

            let ray: Ray = Ray::create(Point3D::new(), Vector3D::with_values(1.0, 0.0, 0.0));
            let sphere: Sphere = Sphere::new(Point3D::with_values(2.0, 0.0, 0.0), 1.0);
            let ray_t: Interval = Interval::new(2.5, 3.5);
            let rec: &mut HitRecord = &mut HitRecord::default();

            assert!(sphere.hit(&ray, ray_t, rec), "Test sphere not hit by ray");
            assert_eq!(
                *rec,
                HitRecord::new(
                    Point3D::with_values(3.0, 0.0, 0.0),
                    Vector3D::with_values(-1.0, 0.0, 0.0),
                    3.0,
                    false
                ),
                "Hit Record not as expected"
            );
        }

        #[test]
        fn sphere_not_hit() {
            //                            .-*****-.
            //                           *         *
            //                          *           *
            //   <---------o            *     c     *
            // (-1,0)    (0,0)          *   (2,0)   *
            //                           *         *
            //                            ''*****''

            let ray: Ray = Ray::create(Point3D::new(), Vector3D::with_values(-1.0, 0.0, 0.0));
            let sphere: Sphere = Sphere::new(Point3D::with_values(2.0, 0.0, 0.0), 1.0);
            let ray_t: Interval = Interval::new(2.5, 3.5);
            let rec: &mut HitRecord = &mut HitRecord::default();

            assert!(!sphere.hit(&ray, ray_t, rec), "Test sphere hit by ray");
            assert_eq!(*rec, HitRecord::default(), "Hit Record not as expected");
        }
    }
}
