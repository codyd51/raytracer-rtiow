use std::sync::Arc;
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
    pub fn new(albedo: Color) -> Arc<Self> {
        Arc::new(
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
    fn scatter(&self, ray: Ray, hit_record: &HitRecord) -> Option<(Ray, Color)> {
        let reflected = Vec3::reflect(ray.direction.unit_vector(), hit_record.normal);
        let fuzzed_direction = reflected + (self.fuzz * Vec3::random_unit_vector());
        let scattered = Ray::new(hit_record.pos, fuzzed_direction);
        Some((scattered, self.albedo))
    }
}

pub struct DielectricMaterial {
    index_of_refraction: f64,
}

impl DielectricMaterial {
    pub fn new(index_of_refraction: f64) -> Arc<Self> {
        Arc::new(
            Self {
                index_of_refraction,
            }
        )
    }
}

impl Material for DielectricMaterial {
    fn scatter(&self, ray: Ray, hit_record: &HitRecord) -> Option<(Ray, Color)> {
        let attenuation = Color::white();
        let refraction_ratio = match hit_record.is_front_face {
            true => 1.0 / self.index_of_refraction,
            false => self.index_of_refraction,
        };
        let unit_direction = ray.direction.unit_vector();

        let cos_theta = f64::min(Vec3::dot(-unit_direction, hit_record.normal), 1.0);
        let sin_theta = (1.0 - (cos_theta * cos_theta)).sqrt();

        let cannot_refract = (refraction_ratio * sin_theta) > 1.0;
        let direction = match cannot_refract {
            true => Vec3::reflect(unit_direction, hit_record.normal),
            false => Vec3::refract(unit_direction, hit_record.normal, refraction_ratio),
        };

        Some((Ray::new(hit_record.pos, direction), attenuation))
    }
}