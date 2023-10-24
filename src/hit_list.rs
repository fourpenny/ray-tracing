use crate::hittable::Hittable;
use crate::hittable::HitRecord;
use crate::ray::Ray;
use crate::vec::Vec3;
use crate::interval::Interval;
use crate::material::{Lambertian, Material};

pub struct HittableList {
    pub objects: Vec<Box<dyn Hittable>>,
}

impl Hittable for HittableList {
    // Check to see if any item in the hittable list has been hit
    fn hit(&self, r: &Ray, ray_t: Interval, mut rec: &mut HitRecord) -> bool {
        let material: Box<dyn Material> = Box::new(Lambertian::new(Vec3::default()));
        let mut temp_rec = HitRecord{
            p: Vec3::default(),
            normal: Vec3::default(),
            t: 0.0,
            front_face: false,
            material: &material,
        }; 
        let mut hit_anything: bool = false;
        let mut closest_so_far: f64 = ray_t.max();

        for obj in self.objects.iter() {
            let obj_hit: bool = obj.hit(r, Interval::new(ray_t.min(), closest_so_far), &mut temp_rec);
            if obj_hit {
                hit_anything = true;
                closest_so_far = temp_rec.t;
                *rec = temp_rec; 
            }
        }
        return hit_anything;
    }
}
