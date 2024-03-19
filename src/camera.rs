use std::fs::File;
use std::io::Write;
use std::time::SystemTime;
use rayon::iter::IntoParallelIterator;
use rayon::iter::ParallelIterator;
use crate::color::Color;
use crate::hittable::Hittable;
use crate::interval::Interval;
use crate::pos::Pos;
use crate::ray::Ray;
use crate::utils::rand_proportion;
use crate::vec3::Vec3;

pub struct Camera {
    image_width: usize,
    image_height: usize,
    samples_per_pixel: usize,
    max_ray_bounces: usize,
    camera_center: Pos,
    top_left_pixel_loc: Pos,
    /// Offset of pixel to the right
    pixel_delta_u: Vec3,
    /// Offset of pixel below
    pixel_delta_v: Vec3,
}

impl Camera {
    pub fn new(
        aspect_ratio: f64,
        image_width: usize,
        samples_per_pixel: usize,
        max_ray_bounces: usize,
    ) -> Self {
        let image_height = match (image_width as f64 / aspect_ratio) as usize {
            // Ensure the height is at least 1
            0 => 1,
            height => height,
        };

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

        Self {
            image_width,
            image_height,
            samples_per_pixel,
            max_ray_bounces,
            camera_center,
            top_left_pixel_loc,
            pixel_delta_u,
            pixel_delta_v,
        }
    }

    pub fn render(&self, world: &dyn Hittable) -> std::io::Result<()> {
        // Render
        let mut out = vec![];
        // Write out the PPM header
        out.extend(format!("P3\n{} {}\n255\n", self.image_width, self.image_height).as_bytes());

        let mut output_lines = vec![vec![]; self.image_height];

        // Render each scanline in parallel
        let scanline_indexes_to_pixel_bytes = (0..self.image_height).into_par_iter().map(|y|{
            println!("Process scanline {}", self.image_height - y);
            let mut scanline_bytes = vec![];
            for x in 0..self.image_width {
                // Accumulate a pixel color through random sampling around the pixel
                let mut pixel_color = Color::black();
                for _sample in 0..self.samples_per_pixel {
                    let ray = self.get_ray(x, y);
                    pixel_color += self.ray_color(ray, world, self.max_ray_bounces);
                }
                self.write_color(&mut scanline_bytes, pixel_color);
            }
            scanline_bytes.extend("\n".as_bytes());
            (y, scanline_bytes)
        }).collect::<Vec<(usize, Vec<u8>)>>();

        // Order the rendered scanlines
        for (y, scanline_bytes) in scanline_indexes_to_pixel_bytes.iter() {
            output_lines[*y] = scanline_bytes.to_vec();
        }
        // And write them out
        for line in output_lines.iter() {
            out.extend(line)
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

    /// Get a randomly sampled camera ray for the pixel at (x, y)
    fn get_ray(&self, x: usize, y: usize) -> Ray {
        let pixel_center = self.top_left_pixel_loc + (x as f64 * self.pixel_delta_u) + (y  as f64 * self.pixel_delta_v);
        let pixel_sample = pixel_center + self.pixel_sample_square();

        let ray_origin = self.camera_center;
        let ray_direction = *pixel_sample - *ray_origin;
        Ray::new(self.camera_center, ray_direction.into())
    }

    /// Randomly sample a point in the square surrounding a pixel
    fn pixel_sample_square(&self) -> Vec3 {
        let px = -0.5 + rand_proportion();
        let py = -0.5 + rand_proportion();
        (px * self.pixel_delta_u) + (py * self.pixel_delta_v)
    }

    fn linear_to_gamma(linear_component: f64) -> f64 {
        linear_component.sqrt()
    }

    fn write_color(&self, out: &mut Vec<u8>, pixel_color: Color) {
        let scale = 1.0 / self.samples_per_pixel as f64;
        let scaled_color = Color::new(
            pixel_color.r() * scale,
            pixel_color.g() * scale,
            pixel_color.b() * scale,
        );

        let corrected_color = Color::new(
            Self::linear_to_gamma(scaled_color.r()),
            Self::linear_to_gamma(scaled_color.g()),
            Self::linear_to_gamma(scaled_color.b()),
        );

        let intensity = Interval::new(0.000, 0.999);
        // TODO(PT): Refactor this into Color?
        out.extend(
            format!(
                "{} {} {}    ",
                (256. * intensity.clamp(corrected_color.r())).floor(),
                (256. * intensity.clamp(corrected_color.g())).floor(),
                (256. * intensity.clamp(corrected_color.b())).floor(),
            ).as_bytes()
        );
    }

    fn ray_color(&self, ray: Ray, world: &dyn Hittable, ray_bounces_remaining: usize) -> Color {
        // If we've exceeded the ray bounce limit, no more light is contributed
        if ray_bounces_remaining <= 0 {
            Color::black()
        }
        // Don't allow intersections too close to this surface
        else if let Some(hit_record) = world.hit(ray, Interval::new(0.001, f64::MAX)) {
            if let Some((scattered_ray, color)) = hit_record.material.scatter(ray, &hit_record) {
                color * self.ray_color(scattered_ray, world, ray_bounces_remaining - 1)
            }
            else {
                Color::black()
            }
        }
        else {
            // Background
            let unit_direction = ray.direction().unit_vector();
            let a = 0.5 * (unit_direction.y + 1.0);
            ((1.0 - a) * Color::new(1., 1., 1.)) + (a * Color::new(0.5, 0.7, 1.0))
        }
    }
}