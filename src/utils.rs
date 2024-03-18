use std::f64::consts::PI;
use rand;
use rand::Rng;

pub fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * PI / 180.
}

pub fn rand_proportion() -> f64 {
    rand::thread_rng().gen_range(0.0..1.0)
}

pub fn rand_double(min: f64, max: f64) -> f64 {
    rand::thread_rng().gen_range(min..max)
}
