mod vec3;
mod pos;
mod color;
mod ray;

use std::io::prelude::*;
use std::fs::File;
use std::ops::{Add, AddAssign, DivAssign, Index, MulAssign, Neg, Sub};
use std::time::SystemTime;
use crate::color::Color;
use crate::pos::Pos;
use crate::ray::Ray;
use crate::vec3::Vec3;

fn hit_sphere(center: Pos, radius: f64, ray: Ray) -> bool {
    let oc = ray.origin - *center;
    let a = Vec3::dot(ray.direction(), ray.direction());
    let b = 2. * Vec3::dot(oc.into(), ray.direction());
    let c = Vec3::dot(oc.into(), oc.into()) - (radius * radius);
    let discriminant  = (b * b) - (4. * a *  c);
    discriminant >= 0.
}

fn ray_color(ray: Ray) -> Color {
    if hit_sphere(Pos::new(0., 0., -1.), 0.5, ray) {
        Color::new(1., 0., 0.)
    }
    else {
        let unit_direction = ray.direction().unit_vector();
        let a = 0.5 * (unit_direction.y + 1.0);
        ((1.0 - a) * Color::new(1., 1., 1.)) + (a * Color::new(0.5, 0.7, 1.0))
    }
}

fn main() -> std::io::Result<()> {
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;

    let image_height = match (image_width as f64 / aspect_ratio) as usize {
        // Ensure the height is at least 1
        0 => 1,
        height => height,
    };

    // Camera
    let focal_length = 1.0;
    let viewport_height = 2.0;
    let viewport_width = viewport_height * (image_width as f64 / image_height as f64);
    let camera_center = Pos::new(0., 0., 0.);

    // Vectors across horizontal and down vertical edges of the viewport
    let viewport_u = Vec3::new(viewport_width, 0., 0.);
    let viewport_v = Vec3::new(0., -viewport_height, 0.);

    // Pixel-to-pixel vectors in each direction
    let pixel_delta_u = viewport_u / image_width as f64;
    let pixel_delta_v = viewport_v / image_height as f64;

    // Location of upper left pixel of viewport
    let viewport_upper_left = camera_center - Vec3::new(0., 0., focal_length) - (viewport_u / 2.) - (viewport_v / 2.);
    let top_left_pixel_loc = viewport_upper_left + (0.5 * (pixel_delta_u + pixel_delta_v));

    // Render
    let mut out = vec![];
    // Write out the PPM header
    out.extend(format!("P3\n{image_width} {image_height}\n255\n").as_bytes());

    for y in 0..image_height {
        println!("{} scanlines remaining", image_height - y);
        for x in 0..image_width {
            let pixel_center = top_left_pixel_loc + (x as f64 * pixel_delta_u) + (y  as f64 * pixel_delta_v);
            let ray_direction = pixel_center - *camera_center;
            let ray = Ray::new(camera_center, *ray_direction);
            let pixel_color = ray_color(ray);
            out.extend(pixel_color.triplet_str().as_bytes())
        }
        out.extend("\n".as_bytes());
    }
    println!("Done! Writing output files...");

    // Write to our output file
    let mut output_file = File::create("./latest_image.ppm")?;
    output_file.write(&out)?;

    // Copy every output image to a secondary 'history' folder, so I can't accidentally forget to record progress
    let dist_from_epoch = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).expect("Failed to retrieve time since epoch");
    let mut cached_output_file = File::create(format!("./images/{}.ppm", dist_from_epoch.as_secs()))?;
    cached_output_file.write(&out)?;

    Ok(())
}
