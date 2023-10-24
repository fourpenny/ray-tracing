use crate::ray::Ray;
use crate::vec::Vec3;
use crate::interval::Interval;
use crate::material::Material;

#[derive (Copy, Clone)]
pub struct HitRecord<'a> {
    pub p: Vec3,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool,
    pub material: &'a Box<dyn Material>
}

pub trait Hittable {
    fn hit(&self, r: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool;
}

pub fn set_face_normal(r: &Ray, hr: &mut HitRecord){
    // Sets a hit record normal vector so it points
    // out of the surface of the object
    // NOTE: we assume `outward_normal` has unit length.

    hr.front_face = crate::vec::dot(&r.direction(), &mut hr.normal) < 0.0;
    if hr.front_face == false {
        hr.normal *= -1.0;
    }
}
