mod color;
mod hittable;
mod ray;
mod sphere;
mod vec3;
mod camera;

use crate::ray::Ray;
use crate::vec3::Vec3;
use crate::hittable::HittableList;
use crate::camera::Camera;
use crate::color::Color;
use crate::sphere::Sphere;

use std::{fs::File, io::Write, thread, time};
use rand::prelude::*;


const ASPECT_RATIO: f64 = 16./9.;
// const ASPECT_RATIO: f64 = 4. / 3.;
const IMAGE_HEIGHT: u32 = 1080;
// const IMAGE_HEIGHT: u32 = 256;
const VIEWPORT_HEIGHT: f64 = 2.;
const FOCAL_LENGTH: f64 = 1.;
const ORIGIN: Vec3 = Vec3(0., 0., 0.);
const SAMPLES_PER_PIXEL: u32 = 100;

pub fn clamp<T: PartialOrd>(v: T, min: T, max: T) -> T {
    if v < min { min }
    else if v > max { max }
    else { v }
}

pub fn rand_f64(min: f64, max: f64) -> f64 {
    // [min, max) other notations: [min, max[
    min + (max-min)*random::<f64>()
}

fn main() -> std::io::Result<()> {
    let aspect_ratio = ASPECT_RATIO;
    let image_height: u32 = IMAGE_HEIGHT;
    let image_width: u32 = (IMAGE_HEIGHT as f64 * ASPECT_RATIO) as u32;
    let viewport_height: f64 = VIEWPORT_HEIGHT;
    let max_color: u32 = 255;
    let focal_length: f64 = FOCAL_LENGTH;
    let origin: Vec3 = ORIGIN;
    let samples_per_pixel: u32 = SAMPLES_PER_PIXEL;

    // World

    let mut world: HittableList = Default::default();
    world.add(Sphere::new_boxed(Vec3(0., 0., -1.), 0.5));
    world.add(Sphere::new_boxed(Vec3(0., -100.5, -1.), 100.));

    // Camera

    let cam: Camera = Camera::new(aspect_ratio, viewport_height, focal_length, origin);

    // prepare output ppm-file

    let mut file = File::create("test1.ppm")?;
    file.write_all(format!("P3\n{} {}\n{}\n", image_width, image_height, max_color).as_bytes())?;

    let start_time = time::Instant::now();
    for i_y in (0..image_height).rev() {
        // go from top to bottom
        print!("\rLines remaining: {}  ", i_y);
        for i_x in 0..image_width {
            let mut pixel_color = Color::new(0., 0., 0., samples_per_pixel, 255.999);

            for _ in 0..samples_per_pixel {
                let u: f64 = (i_x as f64 + random::<f64>()) / (image_width) as f64;
                let v: f64 = (i_y as f64 + random::<f64>()) / (image_height) as f64;
                let ray = cam.get_ray(u, v);
                pixel_color.color += ray.ray_color(&world).color;
            }

            file.write_all(format!("{}", pixel_color).as_bytes())?;
        }
    }
    let duration = time::Instant::elapsed(&start_time);

    println!("\nDone.");
    println!("Execution time: {:?}", duration);

    Ok(())
}
