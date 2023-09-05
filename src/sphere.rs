use crate::vec::Vec3;
use crate::ray::Ray;
use crate::hittable::HitRecord;

pub struct Sphere {
    center: Vec3,
    radius: f64
}

impl crate::hittable::Hittable for Sphere {
    // Modifies a Hit_Record struct containing information
    // the intersecting ray and its surface normals when
    // the sphere is hit.
    fn hit(&self, r: &Ray, ray_tmin: f64, ray_tmax: f64, rec: &HitRecord) -> bool {
        // We determine whether or not the ray intersects
        // the surface of the sphere using the quadratic
        // formula.
        let oc: Vec3 = r.origin() - self.center;
        let a: f64 = r.direction().length_squared();
        let half_b: f64 = crate::vec::dot(&oc, &r.direction());
        let c: f64 = oc.length_squared() - self.radius*self.radius;

        let discriminant: f64 = half_b * half_b - a*c;
        // If the discriminant is < 0, we don't hit the sphere.
        if discriminant < 0.0 {
            return false;
        }
        let sqrtd: f64 = discriminant.sqrt();

        // Find nearest root in the acceptable range
        let root: f64 = (-half_b - sqrtd) / a;

        // Modify the hit record to include
        // details about the intersecting ray and normal
        rec.t = root;
        rec.p = r.at(rec.t);
        let outward_normal: Vec3 = (rec.p - self.center) / self.radius;
        crate::hittable::set_face_normal(&r, &*rec);

        return true;
    }
}
