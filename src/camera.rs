use crate::vec::Vec3;
use crate::ray::Ray;
use crate::interval::Interval;
use crate::hittable::Hittable;
use crate::utility::random_double;
use crate::material::{Lambertian,Material};
use std::io::Write;

pub struct Camera {
    pub samples_per_pixel: i16, // Count of random samples for each pixel
    pub aspect_ratio: f64, // Ratio of image width to height
    pub image_width: i32, // Rendered image width in pixels
    image_height: i32,
    center: Vec3,
    pixel00_loc: Vec3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
    max_depth: i16,
}

impl Camera {
    pub fn new(samples_per_pixel: i16, aspect_ratio: f64, image_width: i32, max_depth: i16) -> Self {
        // Get image dimensions
        let image_height: i32 = (image_width as f64 / aspect_ratio) as i32;
        let image_height: i32 = std::cmp::max(1, image_height);

        // Calculate viewport dimensions
        let focal_length: f64 = 1.0;
        let viewport_height: f64 = 2.0;
        let viewport_width: f64 = viewport_height * (image_width as f64 / image_height as f64);

        // Calculate vectors on the horizontal and vertical viewport edges
        let viewport_u: Vec3 = Vec3::with_values(viewport_width, 0.0, 0.0);
        let viewport_v: Vec3 = Vec3::with_values(0.0, -viewport_height, 0.0);

        // Calculate the delta vectors from pixel to pixel
        let pixel_delta_u: Vec3 = viewport_u / image_width;
        let pixel_delta_v: Vec3 = viewport_v / image_height;

        // Calculate the location of the upper left pixel
        let camera_center: Vec3 = Vec3::default();
        let viewport_upper_left: Vec3 = camera_center 
            - Vec3::with_values(0.0, 0.0, focal_length) - viewport_u/2 - viewport_v/2;
        // Get the actual center of the pixel at (0, 0) - the upper left hand corner.
        let pixel00_loc: Vec3 = viewport_upper_left + ((pixel_delta_u + pixel_delta_v) * 0.5);
        
        return Camera {
            samples_per_pixel: samples_per_pixel,
            aspect_ratio: aspect_ratio,
            image_width: image_width,
            image_height: image_height,
            center: camera_center,
            pixel00_loc: pixel00_loc,
            pixel_delta_u: pixel_delta_u,
            pixel_delta_v: pixel_delta_v,
            max_depth: max_depth
        };
    }

    // Renders the state of the world from the camera into the provided
    // output buffer
    pub fn render<W>(&self, world: &crate::hit_list::HittableList, mut output: &mut std::io::BufWriter<W>) 
    where
        W: Write
    {
        let data = format!("P3\n{} {} \n255\n", self.image_width, self.image_height);
        output.write_all(data.as_bytes()).expect("Unable to write to file");
        
        for j in 0..self.image_height {
            println!("Scanlines remaining: {}", (self.image_height-j));
            for i in 0..self.image_width {
                let mut pixel_color = Vec3::default();
                for sample in 0..self.samples_per_pixel {
                    let r: Ray = self.get_ray(i, j);
                    pixel_color = pixel_color + ray_color(r, world, self.max_depth.clone());
                }
                crate::vec::write_color(&mut output, pixel_color, self.samples_per_pixel);
            }
        }
        println!("Done!");
    }

    fn get_ray(&self, i: i32, j: i32) -> Ray{
        // Get a randomly sampled point within the pixel at location (i, j)
        let pixel_center: Vec3 = 
            self.pixel00_loc + (self.pixel_delta_u * i) + (self.pixel_delta_v * j);
        let pixel_sample: Vec3 = pixel_center + self.sample_within_pixel();

        let ray_origin: Vec3 = self.center;
        let ray_direction: Vec3 = pixel_sample - ray_origin;

        return Ray::new(ray_origin, ray_direction);
    }

    fn sample_within_pixel(&self) -> Vec3 {
        let px: f64 = -0.5 + random_double();
        let py: f64 = -0.5 + random_double();
        (self.pixel_delta_u * px) + (self.pixel_delta_v * py)
    }
}

pub fn ray_color(ray: Ray, world: &crate::hit_list::HittableList, mut depth: i16) -> Vec3 {
    let material: Box<dyn Material> = Box::new(Lambertian::new(Vec3::default()));
    let mut rec = crate::hittable::HitRecord{
        p: Vec3::default(),
        normal: Vec3::default(),
        t: 0.0,
        front_face: false,
        material: &material
    };

    // If we hit the bounce limit, no more light is gained
    if depth <= 0 {
        return Vec3::default();
    }
    
    // Use value slightly above 0 to avoid issues with FP precision
    if world.hit(&ray, Interval::new(0.001, f64::INFINITY), &mut rec) {
        let direction: Vec3 = rec.normal + Vec3::random_unit_vector();
        return ray_color(Ray::new(rec.p, direction), world, depth-1) * 0.5;
    }

    let unit_direction: Vec3 = crate::vec::unit_vector(ray.direction());
    let a = 0.5 * (unit_direction.y() + 1.0);
    return Vec3::with_values(1.0, 1.0, 1.0) * (1.0-a) + Vec3::with_values(0.5, 0.7, 1.0) * a;
}