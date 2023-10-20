use rand::prelude::*;

pub fn degrees_to_radians(degrees: f64) -> f64 {
    return degrees * std::f64::consts::PI / 180.0;
}

pub fn random_double() -> f64 {
    let mut rng = thread_rng();
    return rng.gen();
}

pub fn random_double_in_range(min: f64, max: f64) -> f64 {
    let mut rng = thread_rng();
    return rng.gen_range(min..max);
}