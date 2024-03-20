use std::sync::Arc;
use crate::color::Color;
use crate::hittable::HitRecord;
use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::Vec3;

pub struct MetalMaterial {
    albedo: Color,
    fuzz: f64,
}

impl MetalMaterial {
    pub fn new(albedo: Color, fuzz: f64) -> Arc<Self> {
        let fuzz = if fuzz < 1.0 { fuzz } else { 1.0 };
        Arc::new(
            Self {
                albedo,
                fuzz,
            }
        )
    }
}

impl Material for MetalMaterial {
    fn scatter(&self, ray: Ray, hit_record: Option<&HitRecord>) -> Option<(Ray, Color)> {
        let hit_record = hit_record.expect("Expected a hit record to be available");
        let reflected = Vec3::reflect(ray.direction.unit_vector(), hit_record.normal);
        let fuzzed_direction = reflected + (self.fuzz * Vec3::random_unit_vector());
        let scattered = Ray::new(hit_record.pos, fuzzed_direction);
        Some((scattered, self.albedo))
    }
}

