use std::fs;
use std::io::BufWriter;

use crate::vec::Vec3;
use crate::ray::Ray;
use crate::camera::Camera;

mod vec;
mod ray;
mod hittable;
mod sphere;
mod hit_list;
mod interval;
mod camera;
mod utility;

fn hit_sphere(center: &Vec3, radius: f64, ray: &Ray) -> f64 {
    let oc: Vec3 = ray.origin() - *center;
    let a: f64 = ray.direction().length_squared(); 
    let half_b: f64 = vec::dot(&oc, &ray.direction());
    let c: f64 = oc.length_squared() - (radius * radius);
    let discriminant: f64 = half_b * half_b - a*c;

    if discriminant < 0.0 {
        return -1.0;
    } else {
        return (-half_b - discriminant.sqrt()) / a;
        //return (-b - discriminant.sqrt() ) / (2.0*a);
    }
}

fn main() {
    // Image
    let aspect_ratio: f64 = 16.0 / 9.0;
    let image_width: i32 = 400;
    let samples_per_pixel: i16 = 100;

    // World
    let mut world = crate::hit_list::HittableList{
        objects: Vec::<Box<dyn crate::hittable::Hittable>>::new()
    };

    world.objects.push(Box::new(crate::sphere::Sphere::with_values(Vec3::with_values(0.0, -50.5, -1.0), 50.0)));
    world.objects.push(Box::new(crate::sphere::Sphere::with_values(Vec3::with_values(0.0, 0.0, -1.0), 0.5)));
    
    // Camera
    let camera: Camera = Camera::new(samples_per_pixel, aspect_ratio, image_width);

    // Render 
    let f = fs::File::create("./image.ppm").expect("Unable to create file");
    let mut f = BufWriter::new(f);
    camera.render(&world, &mut f);
}
