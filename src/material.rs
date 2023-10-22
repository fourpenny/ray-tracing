use crate::ray::Ray;
use crate::hittable::HitRecord;
use crate::vec::Vec3;

trait Material {
    fn scatter(ray: Ray, hit_record: HitRecord, attenuation: Vec3, scattered: Ray) -> bool;
}