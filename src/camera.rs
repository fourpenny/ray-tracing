use crate::vec::Vec3;
use crate::ray::Ray;
use crate::interval::Interval;
use crate::hittable::Hittable;
use std::io::Write;

pub struct Camera {
    pub aspect_ratio: f64, // Ratio of image width to height
    pub image_width: i16, // Rendered image width in pixels
    image_height: i16,
    center: Vec3,
    pixel00_loc: Vec3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3
}

impl Camera {
    pub fn new(image_width: i16, aspect_ratio: f64) -> Self {
        // Get image dimensions
        let image_height: i16 = (image_width as f64 / aspect_ratio) as i16;
        let image_height: i16 = std::cmp::max(1, image_height);

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
            aspect_ratio: aspect_ratio,
            image_width: image_width,
            image_height: image_height,
            center: camera_center,
            pixel00_loc: pixel00_loc,
            pixel_delta_u: pixel_delta_u,
            pixel_delta_v: pixel_delta_v
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
                let pixel_center = self.pixel00_loc + (self.pixel_delta_u * i) + (self.pixel_delta_v * j);
                let ray_direction = pixel_center - self.center;
                let r: Ray = Ray::new(self.center, ray_direction);
    
                let pixel_color: Vec3 = ray_color(r, world);
    
                crate::vec::write_color(&mut output, pixel_color);
            }
        }
        println!("Done!");
    }
}

pub fn ray_color(ray: Ray, world: &crate::hit_list::HittableList) -> Vec3 {
    let mut rec = crate::hittable::HitRecord{
        p: Vec3::default(),
        normal: Vec3::default(),
        t: 0.0,
        front_face: false
    };
    if world.hit(&ray, Interval::new(0.0, f64::INFINITY), &mut rec) {
        return (rec.normal + Vec3::with_values(1.0, 1.0, 1.0)) * 0.5;
    }

    let unit_direction: Vec3 = crate::vec::unit_vector(ray.direction());
    let a = 0.5 * (unit_direction.y() + 1.0);
    return Vec3::with_values(1.0, 1.0, 1.0) * (1.0-a) + Vec3::with_values(0.5, 0.7, 1.0) * a;
}