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

use crate::camera::Camera;
use crate::hittable::Hittable;
use crate::hittable_list::HittableList;
use crate::pos::Pos;
use crate::sphere::Sphere;

fn main() -> std::io::Result<()> {
    let mut world = HittableList::new();
    world.add(Box::new(Sphere::new(Pos::new(0., 0., -1.), 0.5)));
    world.add(Box::new(Sphere::new(Pos::new(0., -100.5, -1.), 100.)));

    let camera = Camera::new(16.0 / 9.0, 400, 100, 50);
    camera.render(&world)
}
