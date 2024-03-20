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
mod gradient;

use std::f64::consts::PI;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::sync::Arc;
use std::time::SystemTime;
use rand::prelude::IndexedRandom;
use crate::camera::Camera;
use crate::color::Color;
use crate::dielectric::DielectricMaterial;
use crate::gradient::GradientMaterial;
use crate::hittable::Hittable;
use crate::hittable_list::HittableList;
use crate::lambertian::LambertianMaterial;
use crate::material::Material;
use crate::metal::MetalMaterial;
use crate::pos::Pos;
use crate::ray::Ray;
use crate::sphere::Sphere;
use crate::utils::{degrees_to_radians, rand_double, rand_proportion};
use crate::vec3::Vec3;

#[derive(Copy, Clone)]
struct SceneParameters {
    aspect_ratio: f64,
    image_width: usize,
    samples_per_pixel: usize,
    max_ray_bounces: usize,
}

fn camera_with_params(
    scene_params: SceneParameters,
    background_material: Arc<GradientMaterial>,
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
        background_material,
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
            GradientMaterial::new(
                Color::new(0.5, 0.7, 1.0),
                Color::white(),
            ),
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
            GradientMaterial::new(
                Color::new(0.5, 0.7, 1.0),
                Color::white(),
            ),
            Pos::new(-2.0, 2.0, 1.0),
            Pos::new(0.0, 0.0, -1.0),
            Vec3::new(0.0, 1.0, 0.0),
            20.0,
            10.0,
            3.4,
        ),
    )
}

struct Hsv {
    // 0 - 360
    hue: f64,
    // 0 - 1
    sat: f64,
    // 0 - 1
    brightness: f64,
}

impl From<Hsv> for Color {
    fn from(val: Hsv) -> Self {
        let h = val.hue;
        let s = val.sat;
        let v = val.brightness;

        let c = v * s;
        let x = c * (1.0 - (((h / 60.0) % 2.0) - 1.0).abs());
        let m = v - c;
        let (r_, g_, b_) = if h >= 0.0 && h < 60.0 {
            (c, x, 0.0)
        } else if h >= 60.0 && h < 120.0 {
            (x, c, 0.0)
        } else if h >= 120.0 && h < 180.0 {
            (0.0, c, x)
        } else if h >= 180.0 && h < 240.0 {
            (0.0, x, c)
        } else if h >= 240.0 && h < 300.0 {
            (x, 0.0, c)
        } else {
            (c, 0.0, x)
        };

        Color::rgb(
            ((r_ + m) * 255.0).round() as u8,
            ((g_ + m) * 255.0).round() as u8,
            ((b_ + m) * 255.0).round() as u8,
        )
    }
}

fn pyramid(
    scene_params: SceneParameters,
    look_from: Pos,
    look_to: Pos,
) -> (HittableList, Camera) {
    let mut world = HittableList::new();

    let radius = 0.5;
    //let height = ((3.0f64).sqrt() / 2.0) * (radius * 1.0);
    let height = 0.5;
    let l2_y = height;
    let layer1 = vec![
        // Back left corner
        Pos::new(-2.0,  0.0, -2.0),
        Pos::new(-2.0,  0.0, -1.0),
        Pos::new(-2.0,  0.0,  0.0),
        Pos::new(-2.0,  0.0,  1.0),
        // Front left corner
        Pos::new(-2.0,  0.0,  2.0),
        Pos::new(-1.0,  0.0,  2.0),
        Pos::new( 0.0,  0.0,  2.0),
        Pos::new( 1.0,  0.0,  2.0),
        // Front right corner
        Pos::new( 2.0,  0.0,  2.0),
        Pos::new( 2.0,  0.0,  1.0),
        Pos::new( 2.0,  0.0,  0.0),
        Pos::new( 2.0,  0.0, -1.0),
        // Back right corner
        Pos::new( 2.0,  0.0, -2.0),
        Pos::new( 1.0,  0.0, -2.0),
        Pos::new( 0.0,  0.0, -2.0),
        Pos::new(-1.0,  0.0, -2.0),
    ];
    // Translate to the origin
    /*
    layer1 = layer1.iter().map(|p| {
        (*p + Vec3::new(0.0, 0.0, 2.5)).into()
    }).collect();
     */
    for (i, pos) in layer1.iter().enumerate() {
        let hue = (360.0 / layer1.len() as f64) * i as f64;
        let hsv = Hsv {
            hue,
            sat: 0.5,
            brightness: 0.8,
        };
        let rgb = Color::from(hsv);
        let mat = LambertianMaterial::new(rgb);
        world.add(Box::new(Sphere::new(*pos, 0.5, &(mat as Arc<dyn Material>))));
    }

    let layer2 = vec![
        // Back left corner
        Pos::new(-1.5,  l2_y, -1.5),
        Pos::new(-1.5,  l2_y, -0.5),
        Pos::new(-1.5,  l2_y,  0.5),
        // Front left corner
        Pos::new( -1.5,  l2_y,  1.5),
        Pos::new( -0.5,  l2_y,  1.5),
        Pos::new( 0.5,  l2_y,  1.5),
        // Front right corner
        Pos::new( 1.5, l2_y,  1.5),
        Pos::new( 1.5, l2_y,  0.5),
        Pos::new( 1.5, l2_y,  -0.5),
        // Back right corner
        Pos::new( 1.5,  l2_y, -1.5),
        Pos::new( 0.5,  l2_y, -1.5),
        Pos::new(-0.5,  l2_y, -1.5),
    ];
    let material_layer2 = DielectricMaterial::new(1.5);
    for pos in layer2.iter() {
        world.add(Box::new(Sphere::new(*pos, 0.5, &(Arc::clone(&material_layer2) as Arc<dyn Material>))));
    }

    let l3_y = height * 2.0;
    let layer3 = vec![
        // Back left corner
        Pos::new(-1.0,  l3_y, -1.0),
        Pos::new(-1.0,  l3_y,  0.0),
        // Front left corner
        Pos::new(-1.0,  l3_y,  1.0),
        Pos::new( 0.0, l3_y,  1.0),
        // Front right corner
        Pos::new( 1.0, l3_y,  1.0),
        Pos::new( 1.0, l3_y,  0.0),
        // Back right corner
        Pos::new( 1.0,  l3_y, -1.0),
        Pos::new( 0.0,  l3_y, -1.0),
    ];
    //let material_layer3 = LambertianMaterial::new(Color::rgb(108, 160, 245));
    let material_layer3 = MetalMaterial::new(Color::rgb(250, 211, 102), 0.);
    for pos in layer3.iter() {
        world.add(Box::new(Sphere::new(*pos, 0.5, &(Arc::clone(&material_layer3) as Arc<dyn Material>))));
    }

    let l4_y = height * 3.0;
    let layer4 = vec![
        // Back left corner
        Pos::new(-0.5,  l4_y, -0.5),
        // Front left corner
        Pos::new(-0.5,  l4_y,  0.5),
        // Front right corner
        Pos::new( 0.5, l4_y,  0.5),
        // Back right corner
        Pos::new( 0.5,  l4_y, -0.5),
    ];
    for pos in layer4.iter() {
        world.add(Box::new(Sphere::new(*pos, 0.5, &(Arc::clone(&material_layer2) as Arc<dyn Material>))));
    }

    let l5_y = height * 4.0;
    let layer5 = vec![Pos::new(0.0,  l5_y, 0.0)];
    let l5_material = MetalMaterial::new(Color::rgb(255, 43, 10), 0.);
    for pos in layer5.iter() {
        world.add(Box::new(Sphere::new(*pos, 0.5, &(Arc::clone(&l5_material) as Arc<dyn Material>))));
    }

    (
        world,
        camera_with_params(
            scene_params,
            GradientMaterial::new(
                Color::white(),
                Color::rgb(100, 150, 130),
            ),
            look_from,
            look_to,
            Vec3::new(0.0, 1.0, 0.0),
            20.0,
            0.0,
            3.4,
        ),
    )
}

fn main3() -> std::io::Result<()> {
    let scene = 2;
    let scene_params = SceneParameters {
        aspect_ratio: 16.0 / 9.0,
        image_width: 400,
        samples_per_pixel: 10,
        max_ray_bounces: 50,
    };
    let (world, camera) = match scene {
        0 => three_balls(scene_params),
        1 => main_cover(scene_params),
        2 => pyramid(
            scene_params,
            Pos::new(0.0, 4.0, 12.0),
            Pos::new(0.0, 0.0, 0.0),
        ),
        val => panic!("Unknown scene {val}"),
    };
    camera.render(&world).map(|_| ())
}

fn main() -> std::io::Result<()> {
    let look_from = Pos::new(0.0, 4.0, -6.0);
    let look_to = Pos::new(0.0, 0.5, 0.0);
    let scene_params = SceneParameters {
        aspect_ratio: 16.0 / 9.0,
        image_width: 400,
        samples_per_pixel: 100,
        max_ray_bounces: 50,
    };

    let radius = look_from.z;
    let step_count = 300;
    let start_time = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).expect("Failed to retrieve time since epoch");
    let output_dir_path = format!("./movie_{}", start_time.as_secs());
    fs::create_dir(output_dir_path.clone())?;

    let camera_distance = 12.0;
    for rotation_step in 0..step_count {
        println!("Process rotation step #{rotation_step} / {step_count}");
        let angle = (rotation_step as f64 * 2.0 * PI) / step_count as f64;
        let look_from_x = camera_distance * angle.sin();
        let look_from_z = camera_distance * angle.cos();
        let look_from = Pos::new(look_from_x, look_from.y, look_from_z);
        //let look_to = look_to + Vec3::new(0.0, 0.0, 0.0);
        let (world, camera) = pyramid(
            scene_params,
            look_from,
            look_to,
        );
        let ppm = camera.render(&world).expect("Failed to render");
        let mut frame_file = File::create(format!("{output_dir_path}/{}.ppm", rotation_step))?;
        frame_file.write(&ppm)?;
    }

    Ok(())
}
