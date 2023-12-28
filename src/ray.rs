//! Module for representing rays in the context of a raytracer.

use crate::vector3d::{Point3D, Vector3D};

/// Represents a ray with an origin and direction in 3D space.
#[derive(Debug, Clone, Copy)]
pub struct Ray {
    origin: Point3D,
    direction: Vector3D,
}

impl Ray {
    /// Creates a new ray with the specified origin and direction.
    ///
    /// # Arguments
    ///
    /// * `origin` - The origin point of the ray.
    /// * `direction` - The direction vector of the ray.
    pub fn create(origin: Point3D, direction: Vector3D) -> Self {
        Ray {
            origin: origin,
            direction: direction,
        }
    }

    /// Gets the origin point of the ray.
    pub const fn origin(self) -> Point3D {
        self.origin
    }

    /// Gets the direction vector of the ray.
    pub const fn direction(self) -> Point3D {
        self.direction
    }

    /// Performs linear interpolation of the ray at a given time `t`.
    ///
    /// # Arguments
    ///
    /// * `t` - The time parameter for linear interpolation.
    ///
    /// # Returns
    ///
    /// Returns a new point on the ray corresponding to the given time `t`.
    pub fn at(self, t: f64) -> Point3D {
        self.origin + t * self.direction
    }
}

#[cfg(test)]
mod tests {
    use crate::ray::Ray;
    use crate::vector3d::{Point3D, Vector3D};

    #[test]
    fn linear_interpolation() {
        let p_orig: Point3D = Point3D::new();
        let v_zero: Vector3D = Vector3D::new();
        let v_one: Vector3D = Vector3D::with_values(1.0, 1.0, 1.0);

        let r_zero: Ray = Ray {
            origin: p_orig,
            direction: v_zero,
        };
        let r_one: Ray = Ray {
            origin: p_orig,
            direction: v_one,
        };
        let r_oone: Ray = Ray {
            origin: v_one,
            direction: v_one,
        };

        assert_eq!(
            r_zero.at(-1.0),
            p_orig,
            "Ray (<0,0,0>, <0,0,0>) at t=-1.0 not at <0,0,0>"
        );
        assert_eq!(
            r_zero.at(0.0),
            p_orig,
            "Ray (<0,0,0>, <0,0,0>) at t=0.0 not at <0,0,0>"
        );
        assert_eq!(
            r_zero.at(1.0),
            p_orig,
            "Ray (<0,0,0>, <0,0,0>) at t=1.0 not at <0,0,0>"
        );

        assert_eq!(
            r_one.at(-1.0),
            -v_one,
            "Ray (<0,0,0>, <1,1,1>) at t=-1.0 not at <-1,-1,-1>"
        );
        assert_eq!(
            r_one.at(0.0),
            p_orig,
            "Ray (<0,0,0>, <1,1,1>) at t=0.0 not at <0,0,0>"
        );
        assert_eq!(
            r_one.at(1.0),
            v_one,
            "Ray (<0,0,0>, <1,1,1>) at t=1.0 not at <1,1,1>"
        );

        assert_eq!(
            r_oone.at(-1.0),
            p_orig,
            "Ray (<1,1,1>, <1,1,1>) at t=-1.0 not at <0,0,0>"
        );
        assert_eq!(
            r_oone.at(0.0),
            v_one,
            "Ray (<1,1,1>, <1,1,1>) at t=-1.0 not at <1,1,1>"
        );
        assert_eq!(
            r_oone.at(1.0),
            2.0 * v_one,
            "Ray (<1,1,1>, <1,1,1>) at t=-1.0 not at <2,2,2>"
        );
    }
}
