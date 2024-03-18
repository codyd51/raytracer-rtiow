use std::fs::File;
use std::io::Write;
use std::time::SystemTime;
use crate::color::Color;
use crate::hittable::Hittable;
use crate::interval::Interval;
use crate::pos::Pos;
use crate::ray::Ray;
use crate::vec3::Vec3;

pub struct Camera {
    image_width: usize,
    image_height: usize,
    camera_center: Pos,
    top_left_pixel_loc: Pos,
    /// Offset of pixel to the right
    pixel_delta_u: Vec3,
    /// Offset of pixel below
    pixel_delta_v: Vec3,
}

impl Camera {
    pub fn new(aspect_ratio: f64, image_width: usize) -> Self {
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

        for y in 0..self.image_height {
            println!("{} scanlines remaining", self.image_height - y);
            for x in 0..self.image_width {
                let pixel_center = self.top_left_pixel_loc + (x as f64 * self.pixel_delta_u) + (y  as f64 * self.pixel_delta_v);
                let ray_direction = pixel_center - self.camera_center.into();
                let ray = Ray::new(self.camera_center, ray_direction.into());
                let pixel_color = self.ray_color(ray, world);
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

    fn ray_color(&self, ray: Ray, world: &dyn Hittable) -> Color {
        if let Some(hit_record) = world.hit(ray, Interval::new(0., f64::MAX)) {
            0.5 * (Color::from(hit_record.normal) + Color::white())
        }
        else {
            // Background
            let unit_direction = ray.direction().unit_vector();
            let a = 0.5 * (unit_direction.y + 1.0);
            ((1.0 - a) * Color::new(1., 1., 1.)) + (a * Color::new(0.5, 0.7, 1.0))
        }
    }
}