use std::fmt::{Display, Formatter};
use std::ops::Deref;
use crate::vec3::Vec3;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Color(Vec3);

impl Color {
    pub fn new(r: f64, g: f64, b: f64) -> Self {
        Self(Vec3::new(r, g, b))
    }

    pub fn triplet_str(&self) -> String {
        format!("{} {} {}    ", (self.0.x * 255.99).floor(), (self.0.y * 255.99).floor(), (self.0.z * 255.99).floor())
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
