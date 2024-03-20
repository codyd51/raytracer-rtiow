use std::sync::Arc;
use crate::color::Color;
use crate::hittable::HitRecord;
use crate::material::Material;
use crate::ray::Ray;
use crate::utils::rand_proportion;
use crate::vec3::Vec3;

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

fn reflectance(cosine: f64, ref_idx: f64) -> f64 {
    // Schlick's approximation for reflectance
    let mut r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
    r0 = r0 * r0;
    r0 + (1.0 - r0) * f64::powf((1.0 - cosine), 5.0)
}

impl Material for DielectricMaterial {
    fn scatter(&self, ray: Ray, hit_record: Option<&HitRecord>) -> Option<(Ray, Color)> {
        let hit_record = hit_record.expect("Expected a hit record to be available");
        let attenuation = Color::white();
        let refraction_ratio = match hit_record.is_front_face {
            true => 1.0 / self.index_of_refraction,
            false => self.index_of_refraction,
        };
        let unit_direction = ray.direction.unit_vector();

        let cos_theta = f64::min(Vec3::dot(-unit_direction, hit_record.normal), 1.0);
        let sin_theta = (1.0 - (cos_theta * cos_theta)).sqrt();

        let cannot_refract = (refraction_ratio * sin_theta) > 1.0;
        let direction = if cannot_refract || reflectance(cos_theta, refraction_ratio) > rand_proportion() {
            Vec3::reflect(unit_direction, hit_record.normal)
        }
        else {
            Vec3::refract(unit_direction, hit_record.normal, refraction_ratio)
        };

        Some((Ray::new(hit_record.pos, direction), attenuation))
    }
}
