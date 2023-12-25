use crate::vector3d::{Point3D, Vector3D};

#[derive(Debug, Clone, Copy)]
pub struct Ray {
    origin: Point3D,
    direction: Vector3D,
}

impl Ray {
    pub fn create(origin: Point3D, direction: Vector3D) -> Self {
        Ray {
            origin: origin,
            direction: direction,
        }
    }

    pub const fn origin(self) -> Point3D {
        self.origin
    }

    pub const fn direction(self) -> Point3D {
        self.direction
    }

    // Linear interpolation of Ray at time t
    pub fn at(self, t: f64) -> Point3D {
        self.origin + t * self.direction
    }
}

#[cfg(test)]
mod tests {
    use crate::ray::Ray;
    use crate::vector3d::{Point3D, Vector3D};

    #[test]
    fn linear_interpolation_ok() {
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

        assert_eq!(r_zero.at(-1.0), p_orig);
        assert_eq!(r_zero.at(0.0), p_orig);
        assert_eq!(r_zero.at(1.0), p_orig);

        assert_eq!(r_one.at(-1.0), -v_one);
        assert_eq!(r_one.at(0.0), p_orig);
        assert_eq!(r_one.at(1.0), v_one);

        assert_eq!(r_oone.at(-1.0), p_orig);
        assert_eq!(r_oone.at(0.0), v_one);
        assert_eq!(r_oone.at(1.0), 2.0 * v_one);
    }
}
