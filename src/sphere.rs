using crate::vec::Vec3;
using crate::vec::Ray;
using crate::hittable::Hit_Record;

pub struct Sphere {
    center: Vec3,
    radius: f64
}

impl crate::hittable::Hittable for Sphere {
    fn hit(&self, r: &Ray, ray_tmin: f64, ray_tmax: f64, rec: &Hit_Record) -> bool {
        oc: Vec3 = r.origin() - self.center();
        a: f64 = r.direction().length_squared();
        half_b: f64 = double_dot(oc, r.direction());
        c: f64 = oc.length_squared() - radius*radius;

        discriminant: f64 = half_b * half_b - a*c;
        if discriminant < 0.0 {
            return false;
        }
        sqrtd: f64 = discriminant.sqrt();

        // Find nearest root in the acceptable range
        root: f64 = (-half_b - sqrtd) / a;

        // Modify the hit record to include
        // details about the intersecting ray and normal
        rec.t = root;
        rec.p = r.at(rec.t);
        rec.normal = (rec.p - center) / radius;

        return true;
    }
}
