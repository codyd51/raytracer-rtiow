use std::fmt::{Display, Formatter};
use std::ops::{AddAssign, Deref, DivAssign, Index, MulAssign, Neg};

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3 {
    fn zero() -> Self {
        Self {
            x: 0f64,
            y: 0f64,
            z: 0f64,
        }
    }

    fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z, }
    }

    fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    fn length_squared(&self) -> f64 {
        self.x.powi(2) + self.y.powi(2) + self.z.powi(2)
    }
}

impl Index<usize> for Vec3 {
    type Output = f64;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => *self.x,
            1 => *self.y,
            2 => *self.z,
            _ => panic!("Invalid Vec3 index: {index}"),
        }
    }
}

impl Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self::new(
            -self.x,
            -self.y,
            -self.z,
        )
    }
}

impl AddAssign<Vec3> for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, rhs: f64) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
    }
}

impl DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, rhs: f64) {
        // Reuse our MulAssign operation
        self *= 1f64 / rhs;
    }
}

impl Display for Vec3 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("Vec3({}, {}, {})", self.x, self.y, self.z))
    }
}
