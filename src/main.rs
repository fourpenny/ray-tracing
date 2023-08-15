use std::fs;
use std::io::{BufWriter, Write};

mod vec;

fn main() {
    
    // Image
    let image_width = 256;
    let image_height = 256;

    // Render 
    let f = fs::File::create("./image.ppm").expect("Unable to create file");
    let mut f = BufWriter::new(f);
    let data = format!("P3\n{} {} \n255\n", image_width, image_height);
    f.write_all(data.as_bytes()).expect("Unable to write to file");
    for j in 0..image_height {
        println!("Scanlines remaining: {}", (image_height-j));
        for i in 0..image_width {
            let r : f32 = i as f32 / (image_width - 1) as f32;
            let g : f32 = j as f32 / (image_height-1) as f32;

            let ir : i32 = (255.999 * r) as i32;
            let ig : i32 = (255.999 * g) as i32;
            let ib : i32 = 0;

            let data = format!("{} {} {}\n", ir, ig, ib);
            f.write_all(data.as_bytes()).expect("Unable to write to file");
        }
    }
    println!("Done!");
}
