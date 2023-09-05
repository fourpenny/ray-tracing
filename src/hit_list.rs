use crate::hittable::Hittable;
use crate::hittable::HitRecord;
use crate::ray::Ray;
use crate::vec::Vec3;

pub struct HittableList {
    pub objects: Vec<Box<dyn Hittable>>,
}

impl Hittable for HittableList {
    // Check to see if any item in the hittable list has been hit
    fn hit(&self, r: &Ray, ray_tmin: f64, ray_tmax: f64, mut rec: &HitRecord) -> bool {
        let temp_rec = HitRecord{
            p: Vec3::new(),
            normal: Vec3::new(),
            t: 0.0,
            front_face: false
        }; 
        let hit_anything: bool = false;
        let closest_so_far: f64 = ray_tmax.clone();

        for obj in self.objects.iter() {
            let obj_hit: bool = obj.hit(r, ray_tmin, ray_tmax, &temp_rec);
            if obj_hit {
                hit_anything = true;
                closest_so_far = temp_rec.t;
                rec = &temp_rec.clone();
            }
        }
        return hit_anything;
    }
}
