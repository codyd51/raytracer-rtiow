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

pub struct DielectricMaterial {
    index_of_refraction: f64,
}

impl DielectricMaterial {
    pub fn new(index_of_refraction: f64) -> Rc<Self> {
        Rc::new(
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
        let refracted = Vec3::refract(unit_direction, hit_record.normal, refraction_ratio);

        Some((Ray::new(hit_record.pos, refracted), attenuation))
    }
}