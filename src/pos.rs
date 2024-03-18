use std::fmt::{Display, Formatter};
use std::ops::{Add, Deref, Sub};
use crate::vec3::Vec3;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Pos(Vec3);

impl Pos {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self(Vec3::new(x, y, z))
    }
}

impl From<Vec3> for Pos {
    fn from(value: Vec3) -> Self {
        Self(value)
    }
}

impl From<Pos> for Vec3 {
    fn from(value: Pos) -> Self {
        value.0
    }
}

impl Deref for Pos {
    type Target = Vec3;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Display for Pos {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("Pos({}, {}, {})", self.0.x, self.0.y, self.0.z))
    }
}

impl Sub<Vec3> for Pos {
    type Output = Pos;

    fn sub(self, rhs: Vec3) -> Self::Output {
        Pos(self.0 - rhs)
    }
}

impl Add<Vec3> for Pos {
    type Output = Pos;

    fn add(self, rhs: Vec3) -> Self::Output {
        Pos(self.0 + rhs)
    }
}
