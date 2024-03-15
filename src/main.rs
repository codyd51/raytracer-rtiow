use std::io::prelude::*;
use std::fs::File;
use std::time::{Instant, SystemTime};

fn main() -> std::io::Result<()> {
    let width = 256;
    let height = 256;

    let mut out = vec![];
    // Write out the PPM header
    out.extend(format!("P3\n{width} {height}\n255\n").as_bytes());

    for y in 0..height {
        for x in 0..width {
            let r = x as f64 / (width - 1) as f64;
            let g = y as f64 / (height - 1) as f64;
            let b = 0f64;

            let ir = (r * 255.99) as u8;
            let ig = (g * 255.99) as u8;
            let ib = (b * 255.99) as u8;

            out.extend(format!("{ir:3} {ig:3} {ib:3}\n").as_bytes());
        }
    }

    // Write to our output file
    let mut output_file = File::create("./latest_image.ppm")?;
    output_file.write(&out)?;

    // Copy every output image to a secondary 'history' folder, so I can't accidentally forget to record progress
    let dist_from_epoch = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).expect("Failed to retrieve time since epoch");
    let mut cached_output_file = File::create(format!("./images/{}.ppm", dist_from_epoch.as_secs()))?;
    cached_output_file.write(&out)?;

    Ok(())
}
