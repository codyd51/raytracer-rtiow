use crate::pos::Pos;
use crate::vec3::Vec3;

#[derive(Debug, Copy, Clone)]
pub struct Ray {
    pub origin: Pos,
    pub direction: Vec3,
}

impl Ray {
    pub fn new(
        origin: Pos,
        direction: Vec3,
    ) -> Self {
        Self {
            origin,
            direction,
        }
    }

    pub fn origin(&self) -> Pos {
        self.origin
    }

    pub fn direction(&self) -> Vec3 {
        self.direction
    }

    pub fn at(&self, t: f64) -> Pos {
        Pos::from(*self.origin + (t * self.direction))
    }
}