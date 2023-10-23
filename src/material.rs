use crate::ray::Ray;
use crate::hittable::HitRecord;
use crate::vec::Vec3;

pub trait Material {
    fn scatter(&self, ray: Ray, hit_record: HitRecord, attenuation: &Vec3, scattered: &Ray) -> bool;
}

#[derive (Copy, Clone, Debug)]
pub struct Lambertian {
    albedo: Vec3,
}

impl Lambertian {
    pub fn new(albedo: Vec3) -> Lambertian {
        return Lambertian{
            albedo: albedo,
        };
    }
}

impl Material for Lambertian {
    fn scatter(&self, ray: Ray, hit_record: HitRecord, mut attenuation: &Vec3, mut scattered: &Ray) -> bool{
        let mut scatter_direction: Vec3 = hit_record.normal + Vec3::random_unit_vector();
        
        if scatter_direction.close_to_zero() {
            scatter_direction = hit_record.normal;
        }

        *scattered = Ray::new(hit_record.p, scatter_direction);
        attenuation = &self.albedo;
        return true;
    }
}