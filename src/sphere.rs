use crate::hittable::{HitRecord, Hittable};
use crate::interval::Interval;
use crate::pos::Pos;
use crate::ray::Ray;
use crate::vec3::Vec3;

#[derive(Debug, Copy, Clone)]
pub struct Sphere {
    center: Pos,
    radius: f64,
}

impl Sphere {
    pub fn new(
        center: Pos,
        radius: f64,
    ) -> Self {
        Self {
            center,
            radius,
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: Ray, ray_t: Interval) -> Option<HitRecord> {
        let oc = ray.origin - self.center.into();
        let a = ray.direction.length_squared();
        let half_b = Vec3::dot(oc.into(), ray.direction());
        let c = oc.length_squared() - (self.radius * self.radius);
        let discriminant = (half_b * half_b) - (a * c);

        if discriminant < 0. {
            return None;
        }

        let sqrtd = discriminant.sqrt();

        // Find the nearest root (=intersection point) that lies in the acceptable range
        let mut root = (-half_b - sqrtd) / a;
        if !ray_t.surrounds(root) {
            root = (-half_b + sqrtd) / a;
            if !ray_t.surrounds(root) {
                return None;
            }
        }

        let hit_pos = ray.at(root);
        let normal = *(hit_pos - self.center.into()) / self.radius;
        Some(
            HitRecord::new(
                root,
                hit_pos,
                ray,
                normal,
            )
        )
    }
}