use crate::hit::{HitRecord, Hittable};
use crate::ray::Ray;

pub struct HittableList {
    objects: Vec<Box<dyn Hittable>>,
}

impl HittableList {
    pub fn new() -> Self {
        Self {
            objects: vec![],
        }
    }

    pub fn with_object(object: Box<dyn Hittable>) -> Self {
        let mut s = Self::new();
        s.add(object);
        s
    }

    pub fn clear(&mut self) {
        self.objects.clear()
    }

    pub fn add(&mut self, object: Box<dyn Hittable>) {
        self.objects.push(object);
    }
}

impl Hittable for HittableList {
    fn hit(&self, ray: Ray, ray_tmin: f64, ray_tmax: f64) -> Option<HitRecord> {
        let mut did_hit_anything = false;
        let mut closest_so_far = ray_tmax;
        let mut closest_hit_record = None;
        for obj in self.objects.iter() {
            if let Some(hit_record) = obj.hit(ray, ray_tmin, closest_so_far) {
                did_hit_anything = true;
                closest_so_far = hit_record.t;
                closest_hit_record = Some(hit_record);
            }
        }
        closest_hit_record
    }
}
