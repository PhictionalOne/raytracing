//! Module for representing 3D vectors in the context of a raytracer.

use rand::prelude::*;
use std::fmt;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub};

/// Represents a 3D vector with components `x`, `y`, and `z`.
#[derive(Debug, Clone, Copy)]
pub struct Vector3D {
    x: f64,
    y: f64,
    z: f64,
}

impl Vector3D {
    /// Creates a new vector with components initialized to zero.
    pub fn new() -> Self {
        Vector3D {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }

    /// Creates a new vector with specified values for `x`, `y`, and `z`.
    ///
    /// # Arguments
    ///
    /// * `x` - The x-component of the vector.
    /// * `y` - The y-component of the vector.
    /// * `z` - The z-component of the vector.
    pub fn with_values(x: f64, y: f64, z: f64) -> Self {
        Vector3D { x, y, z }
    }

    // Getters

    /// Gets the x-component of the vector.
    pub const fn x(self) -> f64 {
        self.x
    }

    /// Gets the y-component of the vector.
    pub const fn y(self) -> f64 {
        self.y
    }

    /// Gets the z-component of the vector.
    pub const fn z(self) -> f64 {
        self.z
    }

    // Length

    /// Calculates the length of the vector.
    pub fn length(self) -> f64 {
        self.length_squared().sqrt()
    }

    /// Calculates the squared length of the vector.
    pub fn length_squared(self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    /// Create a random Vector
    pub fn random() -> Self {
        let mut rng = thread_rng();
        Self::with_values(rng.gen::<f64>(), rng.gen::<f64>(), rng.gen::<f64>())
    }

    /// Create a random Vector whereas each component is within `min` and `max`
    pub fn random_within(min: f64, max: f64) -> Self {
        let mut rng = thread_rng();
        Self::with_values(
            rng.gen_range(min..max),
            rng.gen_range(min..max),
            rng.gen_range(min..max),
        )
    }

    // Products

    /// Calculates the dot product of the vector with another vector.
    ///
    /// # Arguments
    ///
    /// * `other` - The other vector for dot product calculation.
    pub fn dot(self, other: Vector3D) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    /// Calculates the cross product of the vector with another vector.
    ///
    /// # Arguments
    ///
    /// * `other` - The other vector for cross product calculation.
    pub fn cross(self, other: Vector3D) -> Vector3D {
        Vector3D {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }

    /// Returns a unit vector in the direction of the original vector.
    pub fn unit_vector(self) -> Vector3D {
        self / self.length()
    }

    /// Returns a random vector inside the unit sphere
    pub fn random_in_unit_sphere() -> Vector3D {
        loop {
            let p = Self::random_within(-1.0, 1.0);
            if p.length_squared() < 1.0 {
                return p;
            }
        }
    }

    /// Returns a random unit vector
    pub fn random_unit_vector() -> Vector3D {
        Self::random_in_unit_sphere().unit_vector()
    }

    /// Generates a random vector on the hemisphere oriented by the given normal.
    ///
    /// The resulting vector is uniformly distributed over the hemisphere defined by the provided `normal`.
    pub fn random_on_hemisphere(normal: Vector3D) -> Vector3D {
        let on_unit_hemisphere = Self::random_unit_vector();
        if on_unit_hemisphere.dot(normal) > 0.0 {
            on_unit_hemisphere
        } else {
            -on_unit_hemisphere
        }
    }
}

// Implement Eq and PartialEq for Vector3D
impl Eq for Vector3D {}

impl PartialEq for Vector3D {
    // Implement equality comparison for Vector3D
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y && self.z == other.z
    }
}

impl Neg for Vector3D {
    type Output = Vector3D;

    // Implement negation for Vector3D
    fn neg(self) -> Vector3D {
        Vector3D {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl AddAssign for Vector3D {
    // Implement addition assignment for Vector3D
    fn add_assign(&mut self, other: Vector3D) {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
    }
}

impl MulAssign<f64> for Vector3D {
    // Implement multiplication assignment for Vector3D
    fn mul_assign(&mut self, t: f64) {
        self.x *= t;
        self.y *= t;
        self.z *= t;
    }
}

impl DivAssign<f64> for Vector3D {
    // Implement division assignment for Vector3D
    fn div_assign(&mut self, t: f64) {
        *self *= 1.0 / t
    }
}

// Vector Utility Functions

/// Alias for a 3D point represented as a vector.
pub type Point3D = Vector3D;

impl fmt::Display for Vector3D {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}

impl Add for Vector3D {
    type Output = Vector3D;

    fn add(self, other: Vector3D) -> Vector3D {
        Vector3D::with_values(self.x + other.x, self.y + other.y, self.z + other.z)
    }
}

impl Sub for Vector3D {
    type Output = Vector3D;

    fn sub(self, other: Vector3D) -> Vector3D {
        Vector3D::with_values(self.x - other.x, self.y - other.y, self.z - other.z)
    }
}

impl Mul for Vector3D {
    type Output = Vector3D;

    fn mul(self, other: Vector3D) -> Vector3D {
        Vector3D::with_values(self.x * other.x, self.y * other.y, self.z * other.z)
    }
}

impl Mul<f64> for Vector3D {
    type Output = Vector3D;

    fn mul(self, t: f64) -> Vector3D {
        Vector3D::with_values(self.x * t, self.y * t, self.z * t)
    }
}

impl Mul<Vector3D> for f64 {
    type Output = Vector3D;

    fn mul(self, v: Vector3D) -> Vector3D {
        v * self
    }
}

impl Div<f64> for Vector3D {
    type Output = Vector3D;

    fn div(self, t: f64) -> Vector3D {
        self * (1.0 / t)
    }
}

#[cfg(test)]
mod tests {
    use crate::interval::Interval;
    use crate::vector3d::Vector3D;

    static V_123: Vector3D = Vector3D {
        x: 1.0,
        y: 2.0,
        z: 3.0,
    };
    static V_ONE: Vector3D = Vector3D {
        x: 1.0,
        y: 1.0,
        z: 1.0,
    };
    static V_TWO: Vector3D = Vector3D {
        x: 2.0,
        y: 2.0,
        z: 2.0,
    };
    static V_RND: Vector3D = Vector3D {
        x: 1.0,
        y: -2.0,
        z: 3.0,
    };
    static V_X: Vector3D = Vector3D {
        x: 1.0,
        y: 0.0,
        z: 0.0,
    };
    static V_Y: Vector3D = Vector3D {
        x: 0.0,
        y: 1.0,
        z: 0.0,
    };
    static V_Z: Vector3D = Vector3D {
        x: 0.0,
        y: 0.0,
        z: 1.0,
    };

    #[test]
    fn negation() {
        assert_eq!(
            -V_123,
            Vector3D {
                x: -1.0,
                y: -2.0,
                z: -3.0
            }
        );
        assert_eq!(
            -V_ONE,
            Vector3D {
                x: -1.0,
                y: -1.0,
                z: -1.0
            }
        );
        assert_eq!(
            -V_RND,
            Vector3D {
                x: -1.0,
                y: 2.0,
                z: -3.0
            }
        );
    }

    #[test]
    fn add_assign() {
        let mut v = Vector3D::new();
        v += V_X;
        v += V_Y;
        v += V_Z;
        assert_eq!(v, V_ONE);
    }

    #[test]
    fn add() {
        assert_eq!(V_X + V_Y + V_Z, V_ONE);
        assert_eq!(V_X + 2.0 * V_Y + 3.0 * V_Z, V_123);
    }

    #[test]
    fn sub() {
        assert_eq!(V_ONE - V_X - V_Y - V_Z, Vector3D::new());
    }

    #[test]
    fn mul_assign() {
        let mut v = V_ONE;
        v *= 2.0;
        assert_eq!(v, V_TWO);
    }

    #[test]
    fn mul() {
        // v * v'
        assert_eq!(V_123 * V_ONE, V_123);

        // v * t
        assert_eq!(V_ONE * 2.0, V_TWO);

        // t * v
        assert_eq!(2.0 * V_ONE, V_TWO);
    }

    #[test]
    fn div_assign() {
        let mut v = V_TWO;
        v /= 2.0;
        assert_eq!(v, V_ONE);
    }

    #[test]
    fn div() {
        assert_eq!(V_TWO / 2.0, V_ONE);
    }

    #[test]
    fn length() {
        assert_eq!(V_X.length(), 1.0);
        assert_eq!(V_Y.length(), 1.0);
        assert_eq!(V_Z.length(), 1.0);
    }

    #[test]
    fn dot_product() {
        assert_eq!(V_123.dot(V_RND), 6.0);
        assert_eq!(V_TWO.dot(V_TWO), 12.0);
    }

    #[test]
    fn cross_product() {
        assert_eq!(V_ONE.cross(V_ONE), Vector3D::new());
    }

    #[test]
    fn random_different_components() {
        let r = Vector3D::random();
        let rw = Vector3D::random_within(0.0, 100.0);
        let ruv = Vector3D::random_unit_vector();
        let roh = Vector3D::random_on_hemisphere(Vector3D::with_values(0.0, 1.0, 0.0));
        let rius = Vector3D::random_in_unit_sphere();

        assert!(
            r.x() != r.y() || r.x() != r.z() || r.y() != r.z(),
            "random() - The components should differ!"
        );

        assert!(
            rw.x() != rw.y() || rw.x() != rw.z() || rw.y() != rw.z(),
            "random_within() - The components should differ!"
        );

        assert!(
            ruv.x() != ruv.y() || ruv.x() != ruv.z() || ruv.y() != ruv.z(),
            "random_unit_vector() - The components should differ!"
        );

        assert!(
            roh.x() != roh.y() || roh.x() != roh.z() || roh.y() != roh.z(),
            "random_on_hemisphere() - The components should differ!"
        );

        assert!(
            rius.x() != rius.y() || rius.x() != rius.z() || rius.y() != rius.z(),
            "random_in_unit_sphere() - The components should differ!"
        );
    }

    #[test]
    fn random_constraints() {
        let r = Vector3D::random();
        let rw = Vector3D::random_within(0.0, 100.0);
        let ruv = Vector3D::random_unit_vector();
        let rius = Vector3D::random_in_unit_sphere();
        let v = Vector3D::with_values(1.0, 1.0, 1.0);
        let roh = Vector3D::random_on_hemisphere(v);

        assert!(
            (0.0 <= r.x() && r.x() < 1.0)
                && (0.0 <= r.y() && r.y() < 1.0)
                && (0.0 <= r.z() && r.z() < 1.0),
            "random() - components not between [0, 1)!"
        );

        assert!(
            (0.0 <= r.x() && r.x() < 100.0)
                && (0.0 <= r.y() && r.y() < 100.0)
                && (0.0 <= r.z() && r.z() < 100.0),
            "random_within() - components not between [0, 100)!"
        );

        assert!(
            Interval::new(1.0 - 1e-15, 1.0 + 1e-15).contains(ruv.length()),
            "random_unit_vector() - {} Not a unit vector!",
            ruv.length()
        );

        assert!(
            (-1.0 <= rius.x() && rius.x() < 1.0)
                && (-1.0 <= rius.y() && rius.y() < 1.0)
                && (-1.0 <= rius.z() && rius.z() < 1.0),
            "random_in_unit_sphere() - components not between [-1, 1)!"
        );

        assert!(
            rius.length_squared() < 1.0,
            "random_in_unit_sphere() - not in unit_sphere!"
        );

        assert!(
            roh.dot(v) > 0.0,
            "random_on_hemisphere() - not in correct hemisphere"
        );
    }
}
