use crate::vec::Vec3;
use crate::ray::Ray;

pub struct Hit_Record {
    pub p: Vec3;
    pub normal: Vec3;
    pub t: f64;
};

pub trait Hittable {
    fn hit(&self, r: &Ray, ray_tmin: f64, ray_tmax: f64, rec: &Hit_Record) -> bool
}
