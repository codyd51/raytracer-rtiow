mod vec3;
mod pos;
mod color;

use std::io::prelude::*;
use std::fs::File;
use std::ops::{Add, AddAssign, DivAssign, Index, MulAssign, Neg, Sub};
use std::time::SystemTime;
use crate::color::Color;

fn main() -> std::io::Result<()> {
    let width = 256;
    let height = 256;

    let mut out = vec![];
    // Write out the PPM header
    out.extend(format!("P3\n{width} {height}\n255\n").as_bytes());

    for y in 0..height {
        println!("{} scanlines remaining", height - y);
        for x in 0..width {
            let color = Color::new(
                x as f64 / (width - 1) as f64,
                y as f64 / (height - 1) as f64,
                0f64,
            );
            out.extend(color.triplet_str().as_bytes())
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
