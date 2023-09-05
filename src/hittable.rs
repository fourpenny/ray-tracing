use crate::ray::Ray;
use crate::vec::Vec3;

#[derive (Copy, Clone)]
pub struct HitRecord {
    pub p: Vec3,
    pub normal: Vec3,
    pub t: f64,
    front_face: bool
}

pub trait Hittable {
    fn hit(&self, r: &Ray, ray_tmin: f64, ray_tmax: f64, mut rec: &HitRecord) -> bool;
}

pub fn set_face_normal(r: &Ray, hr: &HitRecord){
    // Sets a hit record normal vector so it points
    // out of the surface of the object
    // NOTE: we assume `outward_normal` has unit length.

    hr.front_face = crate::vec::dot(&r.direction(), &mut hr.normal) < 0.0;
    if hr.front_face == false {
        hr.normal *= -1.0;
    }
}
