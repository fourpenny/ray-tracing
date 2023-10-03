use std::fs;
use std::io::BufWriter;

use crate::vec::Vec3;
use crate::ray::Ray;
use crate::interval::Interval;
use crate::camera::Camera;
use crate::hittable::Hittable;

mod vec;
mod ray;
mod hittable;
mod sphere;
mod hit_list;
mod interval;
mod camera;

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

fn ray_color(ray: Ray, world: &crate::hit_list::HittableList) -> Vec3 {
    let mut rec = crate::hittable::HitRecord{
        p: Vec3::default(),
        normal: Vec3::default(),
        t: 0.0,
        front_face: false
    };
    if world.hit(&ray, Interval::new(0.0, f64::INFINITY), &mut rec) {
        return (rec.normal + Vec3::with_values(1.0, 1.0, 1.0)) * 0.5;
    }

    let unit_direction: Vec3 = vec::unit_vector(ray.direction());
    let a = 0.5 * (unit_direction.y() + 1.0);
    return Vec3::with_values(1.0, 1.0, 1.0) * (1.0-a) + Vec3::with_values(0.5, 0.7, 1.0) * a;
}

fn main() {
    // Image
    let aspect_ratio: f64 = 16.0 / 9.0;
    let image_width: i16 = 400;

    // Calculate the image height
    /* let image_height: i16 = (image_width as f64 / aspect_ratio) as i16;
    let image_height: i16 = std::cmp::max(1, image_height); */

    // World
    let mut world = crate::hit_list::HittableList{
        objects: Vec::<Box<dyn crate::hittable::Hittable>>::new()
    };

    world.objects.push(Box::new(crate::sphere::Sphere::with_values(Vec3::with_values(0.0, -50.5, -1.0), 50.0)));
    world.objects.push(Box::new(crate::sphere::Sphere::with_values(Vec3::with_values(0.0, 0.0, -1.0), 0.5)));
    
    // Camera
    let camera: Camera = Camera::new(image_width, aspect_ratio);

    /* let focal_length: f64 = 1.0;
    let viewport_height: f64 = 2.0;
    let viewport_width: f64 = viewport_height * (image_width as f64 /image_height as f64);
    let camera_center: Vec3 = Vec3::default();

    // Calculate the vectors across the horizontal and down the vertical viewport edges.
    let viewport_u: Vec3 = Vec3::with_values(viewport_width, 0.0, 0.0);
    let viewport_v: Vec3 = Vec3::with_values(0.0, -viewport_height, 0.0);

    // Calculate the horizontal and vertical delta vectors from pixel to pixel.
    let pixel_delta_u: Vec3 = viewport_u / image_width;
    let pixel_delta_v: Vec3 = viewport_v / image_height;

    // Calculate the location of the upper left pixel.
    // We only subtract the focal length from the z direction because
    // we assume viewer is in the negative z direction from the camera center.
    let viewport_upper_left: Vec3 = camera_center 
        - Vec3::with_values(0.0, 0.0, focal_length) - viewport_u/2 - viewport_v/2;
    // Get the actual center of the pixel at (0, 0) - the upper left hand corner.
    let pixel00_loc: Vec3 = viewport_upper_left + ((pixel_delta_u + pixel_delta_v) * 0.5); */

    // Render 
    let f = fs::File::create("./image.ppm").expect("Unable to create file");
    let mut f = BufWriter::new(f);
    camera.render(&world, &mut f);
    /* let data = format!("P3\n{} {} \n255\n", image_width, image_height);
    f.write_all(data.as_bytes()).expect("Unable to write to file");
    for j in 0..image_height {
        println!("Scanlines remaining: {}", (image_height-j));
        for i in 0..image_width {
            let pixel_center = pixel00_loc + (pixel_delta_u * i) + (pixel_delta_v * j);
            let ray_direction = pixel_center - camera_center;
            let r: Ray = Ray::new(camera_center, ray_direction);

            let pixel_color: Vec3 = ray_color(r, &world);

            vec::write_color(&mut f, pixel_color);
            //let data = format!("{} {} {}\n", ir, ig, ib);
            //f.write_all(data.as_bytes()).expect("Unable to write to file");
        }
    }
    println!("Done!"); */
}
