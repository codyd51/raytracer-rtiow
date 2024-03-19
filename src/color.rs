use std::fmt::{Display, Formatter};
use std::ops::{Add, AddAssign, Deref, Mul};
use crate::pos::Pos;
use crate::vec3::Vec3;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Color(Vec3);

impl Color {
    pub fn new(r: f64, g: f64, b: f64) -> Self {
        Self(Vec3::new(r, g, b))
    }

    pub fn r(&self) -> f64 {
        self.0.x
    }

    pub fn g(&self) -> f64 {
        self.0.y
    }

    pub fn b(&self) -> f64 {
        self.0.z
    }

    pub fn white() -> Self {
        Self::new(1., 1., 1.)
    }

    pub fn black() -> Self {
        Self::new(0., 0., 0.)
    }

    pub fn triplet_str(&self) -> String {
        format!("{} {} {}    ", (self.r() * 255.99).floor(), (self.g() * 255.99).floor(), (self.b() * 255.99).floor())
    }
}

impl Deref for Color {
    type Target = Vec3;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Display for Color {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("Color({}, {}, {})", self.0.x, self.0.y, self.0.z))
    }
}

impl From<Vec3> for Color {
    fn from(value: Vec3) -> Self {
        Self(value)
    }
}

impl Mul<Color> for f64 {
    type Output = Color;

    fn mul(self, rhs: Color) -> Self::Output {
        Color::from(self * (*rhs))
    }
}

impl Add<Color> for Color {
    type Output = Color;

    fn add(self, rhs: Self) -> Self::Output {
        Color::from(*self + *rhs)
    }
}

impl AddAssign<Color> for Color {
    fn add_assign(&mut self, rhs: Color) {
        self.0.x += rhs.r();
        self.0.y += rhs.g();
        self.0.z += rhs.b();
    }
}

impl Mul<Color> for Color {
    type Output = Color;

    fn mul(self, rhs: Color) -> Self::Output {
        Self::new(
            self.r() * rhs.r(),
            self.g() * rhs.g(),
            self.b() * rhs.b(),
        )
    }
}