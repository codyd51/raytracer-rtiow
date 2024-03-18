use std::fmt::{Display, Formatter};
use std::ops::{Add, AddAssign, Deref, Div, DivAssign, Index, Mul, MulAssign, Neg, Sub};
use crate::utils::{rand_double, rand_proportion};

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3 {
    pub fn zero() -> Self {
        Self {
            x: 0f64,
            y: 0f64,
            z: 0f64,
        }
    }

    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    fn random_proportion() -> Self {
        Self::new(rand_proportion(), rand_proportion(), rand_proportion())
    }

    fn random(min: f64, max: f64) -> Self {
        Self::new(
            rand_double(min, max),
            rand_double(min, max),
            rand_double(min, max),
        )
    }

    fn random_in_unit_sphere() -> Self {
        loop {
            let v = Self::random_proportion();
            if v.length_squared() < 1. {
                return v;
            }
        }
    }

    fn random_unit_vector() -> Self {
        Self::random_in_unit_sphere().unit_vector()
    }

    pub fn random_matching_hemisphere_of_vec(v: Vec3) -> Self {
        let on_unit_sphere = Self::random_unit_vector();
        // Positive dot product if the vectors lie on the same hemisphere
        if Vec3::dot(v, on_unit_sphere) > 0. {
            on_unit_sphere
        }
        else {
            -on_unit_sphere
        }
    }

    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn length_squared(&self) -> f64 {
        self.x.powi(2) + self.y.powi(2) + self.z.powi(2)
    }

    pub fn dot(lhs: Self, rhs: Self) -> f64 {
        (lhs.x * rhs.x) +
        (lhs.y * rhs.y) +
        (lhs.z * rhs.z)
    }

    pub fn cross(&self, rhs: &Self) -> Self {
        Self::new(
            (self.y * rhs.z) - (self.z * rhs.y),
            (self.z * rhs.x) - (self.x * rhs.z),
            (self.x * rhs.y) - (self.y * rhs.x),
        )
    }

    pub fn unit_vector(self) -> Self {
        self / self.length()
    }
}

impl Index<usize> for Vec3 {
    type Output = f64;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
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
        *self *= 1f64 / rhs;
    }
}

impl Display for Vec3 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("Vec3({}, {}, {})", self.x, self.y, self.z))
    }
}

impl Add for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new(
            self.x + rhs.x,
            self.y + rhs.y,
            self.z + rhs.z,
        )
    }
}

impl Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(
            self.x - rhs.x,
            self.y - rhs.y,
            self.z - rhs.z,
        )
    }
}

impl Mul<Vec3> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: Self) -> Self::Output {
        Self::new(
            self.x * rhs.x,
            self.y * rhs.y,
            self.z * rhs.z,
        )
    }
}

impl Mul<f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: f64) -> Self::Output {
        Self::new(
            self.x * rhs,
            self.y * rhs,
            self.z * rhs,
        )
    }
}

// TODO(PT): Is this reverse order necessary?
impl Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        Vec3::new(
            self * rhs.x,
            self * rhs.y,
            self * rhs.z,
        )
    }
}

impl Div<Vec3> for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: Self) -> Self::Output {
        Self::new(
            self.x / rhs.x,
            self.y / rhs.y,
            self.z / rhs.z,
        )
    }
}

impl Div<f64> for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: f64) -> Self::Output {
        Self::new(
            self.x / rhs,
            self.y / rhs,
            self.z / rhs,
        )
    }
}
