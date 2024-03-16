use std::fmt::{Display, Formatter};
use std::ops::Deref;
use crate::vec3::Vec3;

#[derive(Debug, Copy, Clone, PartialEq)]
struct Color(Vec3);

impl Deref for Color {
    type Target = Vec3;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Display for Color {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("Col({}, {}, {})", self.0.x, self.0.y, self.0.z))
    }
}
