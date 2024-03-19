mod vec3;
mod pos;
mod color;
mod ray;
mod hittable;
mod sphere;
mod hittable_list;
mod utils;
mod interval;
mod camera;
mod material;
mod lambertian;
mod dielectric;
mod metal;

use std::f64::consts::PI;
use std::sync::Arc;
use crate::camera::Camera;
use crate::color::Color;
use crate::dielectric::DielectricMaterial;
use crate::hittable::Hittable;
use crate::hittable_list::HittableList;
use crate::lambertian::LambertianMaterial;
use crate::material::Material;
use crate::metal::MetalMaterial;
use crate::pos::Pos;
use crate::sphere::Sphere;
use crate::vec3::Vec3;

fn main() -> std::io::Result<()> {
    let mut world = HittableList::new();

    let material_ground = LambertianMaterial::new(Color::new(0.8, 0.8, 0.0));
    let material_center = LambertianMaterial::new(Color::new(0.1, 0.2, 0.5));
    let material_left = DielectricMaterial::new(1.5);
    let material_right = MetalMaterial::new(Color::new(0.8, 0.6, 0.2), 0.0);

    world.add(Box::new(Sphere::new(Pos::new(0., -100.5, -1.), 100., &(material_ground as Arc<dyn Material>))));
    world.add(Box::new(Sphere::new(Pos::new(0., 0., -1.), 0.5, &(material_center as Arc<dyn Material>))));
    world.add(Box::new(Sphere::new(Pos::new(-1., 0., -1.), 0.5, &(Arc::clone(&material_left) as Arc<dyn Material>))));
    world.add(Box::new(Sphere::new(Pos::new(-1., 0., -1.), -0.4, &(material_left as Arc<dyn Material>))));
    world.add(Box::new(Sphere::new(Pos::new(1., 0., -1.), 0.5, &(material_right as Arc<dyn Material>))));

    let camera = Camera::new(
        16.0 / 9.0,
        800,
        50.0,
        Pos::new(-1.0, 1.0, 1.0),
        Pos::new(0.0, 0.0, -1.0),
        Vec3::new(0.0, 1.0, 0.0),
        100,
        50,
    );
    camera.render(&world)
}
