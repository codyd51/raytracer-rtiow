use crate::color::Color;
use crate::hittable::HitRecord;
use crate::ray::Ray;

pub trait Material {
    fn scatter(
        &self,
        ray: Ray,
        hit_record: Option<&HitRecord>,
    ) -> Option<(Ray, Color)>;
}
