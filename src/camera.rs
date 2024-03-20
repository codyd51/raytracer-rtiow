use std::fs::File;
use std::io::Write;
use std::sync::Arc;
use std::time::SystemTime;
use rayon::iter::IntoParallelIterator;
use rayon::iter::ParallelIterator;
use crate::color::Color;
use crate::gradient::GradientMaterial;
use crate::hittable::{HitRecord, Hittable};
use crate::interval::Interval;
use crate::material::Material;
use crate::pos::Pos;
use crate::ray::Ray;
use crate::utils::{degrees_to_radians, rand_proportion};
use crate::vec3::Vec3;

pub struct Camera {
    image_width: usize,
    image_height: usize,
    vertical_field_of_view_angle: f64,
    samples_per_pixel: usize,
    max_ray_bounces: usize,
    camera_center: Pos,
    top_left_pixel_loc: Pos,
    /// Offset of pixel to the right
    pixel_delta_u: Vec3,
    /// Offset of pixel below
    pixel_delta_v: Vec3,
    /// Variation angle of rays through each pixel
    defocus_angle: f64,
    /// Defocus disk horizontal radius
    defocus_disk_u: Vec3,
    /// Defocus disk vertical radius
    defocus_disk_v: Vec3,
    background_material: Arc<GradientMaterial>,
}

impl Camera {
    pub fn new(
        aspect_ratio: f64,
        image_width: usize,
        background_material: Arc<GradientMaterial>,
        vertical_field_of_view_angle: f64,
        // Camera position in the world
        look_from: Pos,
        // Camera target/direction point.
        look_at: Pos,
        // Camera-relative up direction. Specifies camera rotation.
        up: Vec3,
        defocus_angle: f64,
        // Distance from camera to plane of perfect focus
        focus_distance: f64,
        samples_per_pixel: usize,
        max_ray_bounces: usize,
    ) -> Self {
        let image_height = match (image_width as f64 / aspect_ratio) as usize {
            // Ensure the height is at least 1
            0 => 1,
            height => height,
        };

        // Viewport dimensions
        let theta = degrees_to_radians(vertical_field_of_view_angle);
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h * focus_distance;
        let viewport_width = viewport_height * (image_width as f64 / image_height as f64);

        // Calculate u, v, w unit basis vectors for the camera coordinate frame
        let w = (look_from - *look_at).unit_vector();
        let u = Vec3::cross(&up, &w).unit_vector();
        let v = Vec3::cross(&w, &u);

        // Vectors across horizontal and down vertical edges of the viewport
        let viewport_u = viewport_width * u;
        let viewport_v = viewport_height * -v;

        // Pixel-to-pixel vectors in each direction
        let pixel_delta_u = viewport_u / image_width as f64;
        let pixel_delta_v = viewport_v / image_height as f64;

        // Location of upper left pixel of viewport
        let camera_center = look_from;
        let viewport_upper_left = camera_center - (focus_distance * w) - (viewport_u / 2.0) - (viewport_v / 2.0);
        let top_left_pixel_loc = viewport_upper_left + (0.5 * (pixel_delta_u + pixel_delta_v));

        // Camera defocus disk basis vectors
        let defocus_radius = focus_distance * degrees_to_radians(defocus_angle / 2.0).tan();
        let defocus_disk_u = u * defocus_radius;
        let defocus_disk_v = v * defocus_radius;

        Self {
            image_width,
            image_height,
            vertical_field_of_view_angle,
            samples_per_pixel,
            max_ray_bounces,
            camera_center,
            top_left_pixel_loc,
            pixel_delta_u,
            pixel_delta_v,
            defocus_angle,
            defocus_disk_u,
            defocus_disk_v,
            background_material,
        }
    }

    pub fn render(&self, world: &dyn Hittable) -> std::io::Result<Vec<u8>> {
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

        Ok(out)
    }

    /// Get a randomly sampled camera ray for the pixel at (x, y),
    /// originating from the camera defocus disk
    fn get_ray(&self, x: usize, y: usize) -> Ray {
        let pixel_center = self.top_left_pixel_loc + (x as f64 * self.pixel_delta_u) + (y  as f64 * self.pixel_delta_v);
        let pixel_sample = pixel_center + self.pixel_sample_square();

        let ray_origin = if self.defocus_angle <= 0.0 {
            self.camera_center
        } else {
            self.defocus_disk_sample()
        };
        let ray_direction = *pixel_sample - *ray_origin;
        Ray::new(ray_origin, ray_direction.into())
    }

    /// Randomly sample a point in the camera defocus disk
    fn defocus_disk_sample(&self) -> Pos {
        let v = Vec3::random_in_unit_disk();
        self.camera_center + (v.x * self.defocus_disk_u) + (v.y * self.defocus_disk_v)
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
            if let Some((scattered_ray, color)) = hit_record.material.scatter(ray, Some(&hit_record)) {
                color * self.ray_color(scattered_ray, world, ray_bounces_remaining - 1)
            }
            else {
                Color::black()
            }
        }
        else {
            // Background
            let (_, background_color) = self.background_material.scatter(ray, None).expect("Failed to get a ray color for the background");
            background_color
        }
    }
}