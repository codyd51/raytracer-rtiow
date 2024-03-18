use crate::pos::Pos;
use crate::ray::Ray;
use crate::vec3::Vec3;

#[derive(Debug, Copy, Clone)]
pub struct HitRecord {
    pub t: f64,
    pub pos: Pos,
    pub normal: Vec3,
    pub is_front_face: bool,
}

impl HitRecord {
    /// Note that `outward_normal` should be a unit-length vector
    /// TODO(PT): Perhaps we could have a type that encodes/enforces this?
    pub fn new(
        t: f64,
        pos: Pos,
        ray: Ray,
        outward_normal: Vec3,
    ) -> Self {
        let is_front_face = Vec3::dot(ray.direction(), outward_normal) < 0.;
        let normal = match is_front_face {
            true => outward_normal,
            false => -outward_normal,
        };

        Self {
            t,
            pos,
            normal,
            is_front_face,
        }
    }
}

pub trait Hittable {
    fn hit(
        &self,
        ray: Ray,
        ray_tmin: f64,
        ray_tmax: f64,
    ) -> Option<HitRecord>;
}
