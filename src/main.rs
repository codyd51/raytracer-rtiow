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
use rand::prelude::IndexedRandom;
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
use crate::utils::{rand_double, rand_proportion};
use crate::vec3::Vec3;

fn main() -> std::io::Result<()> {
    let mut world = HittableList::new();

    let material_ground = LambertianMaterial::new(Color::new(0.5, 0.5, 0.5));
    world.add(Box::new(Sphere::new(Pos::new(0.0, -1000.0, 0.0), 1000.0, &(material_ground as Arc<dyn Material>))));

    for a in -11..11 {
        for b in -11..11 {
            let center = Pos::new(
                a as f64 + (0.9 * rand_proportion()),
                0.2,
                b as f64 + (0.9 * rand_proportion()),
            );
            if (center - *Pos::new(4.0, 0.2, 0.0)).length() > 0.9 {
                let mat_selection = [0, 1, 2].choose(&mut rand::thread_rng()).expect("Failed to make a selection");
                let material: Arc<dyn Material> = match mat_selection {
                    0 => {
                        // Diffuse
                        let albedo = Color::random() * Color::random();
                        LambertianMaterial::new(albedo)
                    }
                    1 => {
                        // Metal
                        let albedo = Color::random_in_range(0.5, 1.0);
                        let fuzz = rand_double(0.0, 0.5);
                        MetalMaterial::new(albedo, fuzz)
                    }
                    2 => {
                        // Glass
                        DielectricMaterial::new(1.5)
                    }
                    _ => panic!("Should never happen"),
                };
                world.add(Box::new(Sphere::new(center, 0.2, &material)));
            }
        }
    }

    let mat1 = DielectricMaterial::new(1.5);
    world.add(Box::new(Sphere::new(Pos::new(0.0, 1.0, 0.0), 1.0, &(mat1 as Arc<dyn Material>))));

    let mat2 = LambertianMaterial::new(Color::new(0.4, 0.2, 0.1));
    world.add(Box::new(Sphere::new(Pos::new(-4.0, 1.0, 0.0), 1.0, &(mat2 as Arc<dyn Material>))));

    let mat3 = MetalMaterial::new(Color::new(0.7, 0.6, 0.5), 0.0);
    world.add(Box::new(Sphere::new(Pos::new(4.0, 1.0, 0.0), 1.0, &(mat3 as Arc<dyn Material>))));

    let camera = Camera::new(
        16.0 / 9.0,
        1200,
        20.0,
        Pos::new(13.0, 2.0, 3.0),
        Pos::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
        0.0,
        3.4,
        500,
        50,
    );
    camera.render(&world)
}
