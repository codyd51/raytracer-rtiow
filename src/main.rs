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

use std::rc::Rc;
use crate::camera::Camera;
use crate::color::Color;
use crate::hittable::Hittable;
use crate::hittable_list::HittableList;
use crate::material::{LambertianMaterial, Material, MetalMaterial};
use crate::pos::Pos;
use crate::sphere::Sphere;

fn main() -> std::io::Result<()> {
    let mut world = HittableList::new();

    let material_ground = LambertianMaterial::new(Color::new(0.8, 0.8, 0.));
    let material_center = LambertianMaterial::new(Color::new(0.7, 0.3, 0.3));
    let material_left = MetalMaterial::new(Color::new(0.8, 0.8, 0.8), 0.3);
    let material_right = MetalMaterial::new(Color::new(0.8, 0.6, 0.2), 0.8);

    world.add(Box::new(Sphere::new(Pos::new(0., -100.5, -1.), 100., &(material_ground as Rc<dyn Material>))));
    world.add(Box::new(Sphere::new(Pos::new(0., 0., -1.), 0.5, &(material_center as Rc<dyn Material>))));
    world.add(Box::new(Sphere::new(Pos::new(-1., 0., -1.), 0.5, &(material_left as Rc<dyn Material>))));
    world.add(Box::new(Sphere::new(Pos::new(1., 0., -1.), 0.5, &(material_right as Rc<dyn Material>))));

    let camera = Camera::new(16.0 / 9.0, 800, 100, 50);
    camera.render(&world)
}
