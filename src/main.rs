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

struct SceneParameters {
    aspect_ratio: f64,
    image_width: usize,
    samples_per_pixel: usize,
    max_ray_bounces: usize,
}

fn camera_with_params(
    scene_params: SceneParameters,
    look_from: Pos,
    look_at: Pos,
    up: Vec3,
    vertical_field_of_view_angle: f64,
    defocus_angle: f64,
    focus_distance: f64,
) -> Camera {
    Camera::new(
        scene_params.aspect_ratio,
        scene_params.image_width,
        vertical_field_of_view_angle,
        look_from,
        look_at,
        up,
        defocus_angle,
        focus_distance,
        scene_params.samples_per_pixel,
        scene_params.max_ray_bounces,
    )
}

fn main_cover(scene_parameters: SceneParameters) -> (HittableList, Camera) {
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

    (
        world,
        camera_with_params(
            scene_parameters,
            Pos::new(13.0, 2.0, 3.0),
            Pos::new(0.0, 0.0, 0.0),
            Vec3::new(0.0, 1.0, 0.0),
            20.0,
            0.6,
            10.0,
        ),
    )
}

fn three_balls(scene_params: SceneParameters) -> (HittableList, Camera) {
    let mut world = HittableList::new();

    let material_ground = LambertianMaterial::new(Color::new(0.8, 0.8, 0.0));
    let material_center = LambertianMaterial::new(Color::new(0.1, 0.2, 0.5));
    let material_left = DielectricMaterial::new(1.5);
    let material_right = MetalMaterial::new(Color::new(0.8, 0.6, 0.2), 0.0);

    world.add(Box::new(Sphere::new(Pos::new(0.0, -100.5, -1.0), 100.0, &(material_ground as Arc<dyn Material>))));
    world.add(Box::new(Sphere::new(Pos::new(0.0, 0.0, -1.0), 0.5, &(material_center as Arc<dyn Material>))));
    world.add(Box::new(Sphere::new(Pos::new(-1.0, 0.0, -1.0), 0.5, &(Arc::clone(&material_left) as Arc<dyn Material>))));
    world.add(Box::new(Sphere::new(Pos::new(-1.0, 0.0, -1.0), -0.4, &(material_left as Arc<dyn Material>))));
    world.add(Box::new(Sphere::new(Pos::new(1.0, 0.0, -1.0), 0.5, &(material_right as Arc<dyn Material>))));

    (
        world,
        camera_with_params(
            scene_params,
            Pos::new(-2.0, 2.0, 1.0),
            Pos::new(0.0, 0.0, -1.0),
            Vec3::new(0.0, 1.0, 0.0),
            20.0,
            10.0,
            3.4,
        ),
    )
}

fn main() -> std::io::Result<()> {
    let scene = 1;
    let scene_params = SceneParameters {
        aspect_ratio: 16.0 / 9.0,
        image_width: 400,
        samples_per_pixel: 50,
        max_ray_bounces: 50,
    };
    let (world, camera) = match scene {
        0 => three_balls(scene_params),
        1 => main_cover(scene_params),
        val => panic!("Unknown scene {val}"),
    };
    camera.render(&world)
}
