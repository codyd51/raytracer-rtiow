use std::rc::Rc;
use crate::interval::Interval;
use crate::material::Material;
use crate::pos::Pos;
use crate::ray::Ray;
use crate::vec3::Vec3;

pub struct HitRecord {
    pub t: f64,
    pub pos: Pos,
    pub normal: Vec3,
    pub is_front_face: bool,
    pub material: Rc<dyn Material>,
}

impl HitRecord {
    /// Note that `outward_normal` should be a unit-length vector
    /// TODO(PT): Perhaps we could have a type that encodes/enforces this?
    pub fn new(
        t: f64,
        pos: Pos,
        ray: Ray,
        outward_normal: Vec3,
        material: &Rc<dyn Material>,
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
            material: Rc::clone(material),
        }
    }
}

pub trait Hittable {
    fn hit(
        &self,
        ray: Ray,
        ray_t: Interval,
    ) -> Option<HitRecord>;
}
