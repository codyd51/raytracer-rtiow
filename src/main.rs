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

use std::sync::Arc;
use crate::camera::Camera;
use crate::color::Color;
use crate::hittable::Hittable;
use crate::hittable_list::HittableList;
use crate::material::{DielectricMaterial, LambertianMaterial, Material, MetalMaterial};
use crate::pos::Pos;
use crate::sphere::Sphere;

fn main() -> std::io::Result<()> {
    let mut world = HittableList::new();

    let material_ground = LambertianMaterial::new(Color::new(0.8, 0.8, 0.));
    let material_center = DielectricMaterial::new(1.5);
    let material_left = DielectricMaterial::new(1.5);
    let material_right = MetalMaterial::new(Color::new(0.8, 0.6, 0.2), 0.8);

    world.add(Box::new(Sphere::new(Pos::new(0., -100.5, -1.), 100., &(material_ground as Arc<dyn Material>))));
    world.add(Box::new(Sphere::new(Pos::new(0., 0., -1.), 0.5, &(material_center as Arc<dyn Material>))));
    world.add(Box::new(Sphere::new(Pos::new(-1., 0., -1.), 0.5, &(material_left as Arc<dyn Material>))));
    world.add(Box::new(Sphere::new(Pos::new(1., 0., -1.), 0.5, &(material_right as Arc<dyn Material>))));

    let camera = Camera::new(16.0 / 9.0, 400, 100, 50);
    camera.render(&world)
}
