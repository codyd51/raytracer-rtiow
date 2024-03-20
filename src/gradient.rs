use std::sync::Arc;
use crate::color::Color;
use crate::hittable::HitRecord;
use crate::material::Material;
use crate::ray::Ray;

pub struct GradientMaterial {
    from_color: Color,
    to_color: Color,
}

impl GradientMaterial {
    pub fn new(
        from_color: Color,
        to_color: Color,
    ) -> Arc<Self> {
        Arc::new(
            Self {
                from_color,
                to_color,
            }
        )
    }
}

impl Material for GradientMaterial {
    fn scatter(&self, ray: Ray, _hit_record: Option<&HitRecord>) -> Option<(Ray, Color)> {
        let unit_direction = ray.direction().unit_vector();
        let a = unit_direction.y + 1.0;
        Some((Ray::zero(), ((1.0 - a) * self.to_color) + (a * self.from_color)))
    }
}
