use rand::{self, Rng};
use std::f64::consts::PI;
// Constants
pub fn degrees_to_radian(degrees: f64) -> f64 {
    degrees * PI / 180.0
}

pub fn random_number() -> f64 {
    let mut rng = rand::thread_rng();

    rng.gen()
}

pub fn random_boundaries(min: f64, max: f64) -> f64 {
    min + (max - min) * random_number()
}

pub fn clamp(x: f64, min: f64, max: f64) -> f64 {
    if x < min {
        return min;
    } else if x > max {
        return max;
    }
    x
}
