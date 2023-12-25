use std::fmt;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub};

#[derive(Debug, Clone, Copy)]
pub struct Vector3D {
    x: f64,
    y: f64,
    z: f64,
}

impl Vector3D {
    // Default constructor
    pub fn new() -> Self {
        Vector3D {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }

    // Constructor with specific values
    pub fn with_values(x: f64, y: f64, z: f64) -> Self {
        Vector3D { x, y, z }
    }

    // Getters
    pub const fn x(self) -> f64 {
        self.x
    }

    pub const fn y(self) -> f64 {
        self.y
    }

    pub const fn z(self) -> f64 {
        self.z
    }

    // Length
    pub fn length(self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn length_squared(self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    // Products
    pub fn dot(self, other: Vector3D) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn cross(self, other: Vector3D) -> Vector3D {
        Vector3D {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }

    // Unit Vector, normalized by its length
    pub fn unit_vector(self) -> Vector3D {
        self / self.length()
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

// Alias for future convenience
type Point3D = Vector3D;

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
    fn negation_ok() {
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
    fn add_assign_ok() {
        let mut v = Vector3D::new();
        v += V_X;
        v += V_Y;
        v += V_Z;
        assert_eq!(v, V_ONE);
    }

    #[test]
    fn add_ok() {
        assert_eq!(V_X + V_Y + V_Z, V_ONE);
        assert_eq!(V_X + 2.0 * V_Y + 3.0 * V_Z, V_123);
    }

    #[test]
    fn sub_ok() {
        assert_eq!(V_ONE - V_X - V_Y - V_Z, Vector3D::new());
    }

    #[test]
    fn mul_assign_ok() {
        let mut v = V_ONE;
        v *= 2.0;
        assert_eq!(v, V_TWO);
    }

    #[test]
    fn mul_ok() {
        // v * v'
        assert_eq!(V_123 * V_ONE, V_123);

        // v * t
        assert_eq!(V_ONE * 2.0, V_TWO);

        // t * v
        assert_eq!(2.0 * V_ONE, V_TWO);
    }

    #[test]
    fn div_assign_ok() {
        let mut v = V_TWO;
        v /= 2.0;
        assert_eq!(v, V_ONE);
    }

    #[test]
    fn div_ok() {
        assert_eq!(V_TWO / 2.0, V_ONE);
    }

    #[test]
    fn length_ok() {
        assert_eq!(V_X.length(), 1.0);
        assert_eq!(V_Y.length(), 1.0);
        assert_eq!(V_Z.length(), 1.0);
    }

    #[test]
    fn dot_ok() {
        assert_eq!(V_123.dot(V_RND), 6.0);
        assert_eq!(V_TWO.dot(V_TWO), 12.0);
    }

    #[test]
    fn cross_ok() {
        assert_eq!(V_ONE.cross(V_ONE), Vector3D::new());
    }
}
