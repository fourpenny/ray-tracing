use rand::prelude::*;

pub fn degrees_to_radians(degrees: f64) -> f64 {
    let x:f64 = degrees * std::f64::consts::PI / 180.0;
    x
}

pub fn random_double() -> f64 {
    let mut rng = thread_rng();
    let x: f64 = rng.gen();
    x
}

pub fn random_double_in_range(min: f64, max: f64) -> f64 {
    let mut rng = thread_rng();
    let x: f64 = rng.gen_range(min..max);
    x
}