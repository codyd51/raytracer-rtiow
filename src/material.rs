use std::rc::Rc;
use crate::color::Color;
use crate::hittable::HitRecord;
use crate::ray::Ray;
use crate::vec3::Vec3;

pub trait Material {
    fn scatter(
        &self,
        ray: Ray,
        hit_record: &HitRecord,
    ) -> Option<(Ray, Color)>;
}

pub struct LambertianMaterial {
    albedo: Color,
}

impl LambertianMaterial {
    pub fn new(albedo: Color) -> Rc<Self> {
        Rc::new(
            Self {
                albedo,
            }
        )
    }
}

impl Material for LambertianMaterial {
    fn scatter(&self, ray: Ray, hit_record: &HitRecord) -> Option<(Ray, Color)> {
        let mut scatter_direction = hit_record.normal + Vec3::random_unit_vector();

        // Catch degenerate scatter direction
        if scatter_direction.is_near_zero() {
            scatter_direction = hit_record.normal;
        }

        let scattered = Ray::new(hit_record.pos, scatter_direction);
        Some((scattered, self.albedo))
    }
}

pub struct MetalMaterial {
    albedo: Color,
    fuzz: f64,
}

impl MetalMaterial {
    pub fn new(albedo: Color, fuzz: f64) -> Rc<Self> {
        let fuzz = if fuzz < 1.0 { fuzz } else { 1.0 };
        Rc::new(
            Self {
                albedo,
                fuzz,
            }
        )
    }
}

impl Material for MetalMaterial {
    fn scatter(&self, ray: Ray, hit_record: &HitRecord) -> Option<(Ray, Color)> {
        let reflected = Vec3::reflect(ray.direction.unit_vector(), hit_record.normal);
        let fuzzed_direction = reflected + (self.fuzz * Vec3::random_unit_vector());
        let scattered = Ray::new(hit_record.pos, fuzzed_direction);
        Some((scattered, self.albedo))
    }
}
