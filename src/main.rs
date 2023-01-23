mod color;
mod hittable;
mod ray;
mod sphere;
mod vec3;

use ray::Ray;
use vec3::Vec3;
use hittable::HittableList;

use crate::color::Color;
use std::{fs::File, io::Write, thread, time};
use crate::sphere::Sphere;

// const ASPECT_RATIO: f64 = 16./9.;
const ASPECT_RATIO: f64 = 4. / 3.;
// const IMAGE_HEIGHT: u32 = 1080;
const IMAGE_HEIGHT: u32 = 256;
const VIEWPORT_HEIGHT: f64 = 2.;
const FOCAL_LENGTH: f64 = 1.;
const ORIGIN: Vec3 = Vec3(0., 0., 0.);

fn main() -> std::io::Result<()> {
    let image_height: u32 = IMAGE_HEIGHT;
    let image_width: u32 = (IMAGE_HEIGHT as f64 * ASPECT_RATIO) as u32;
    let viewport_height: f64 = VIEWPORT_HEIGHT;
    let viewport_width: f64 = VIEWPORT_HEIGHT * ASPECT_RATIO;
    let max_color: u8 = 255;
    let focal_length: f64 = FOCAL_LENGTH;
    let origin: Vec3 = ORIGIN;

    let mut world: HittableList = Default::default();
    world.add(Sphere::new_boxed(Vec3(0., 0., -0.5), 0.05));
    world.add(Sphere::new_boxed(Vec3(-0.1, 0., -0.5), 0.05));
    world.add(Sphere::new_boxed(Vec3(-0.2, 0., -0.5), 0.05));
    world.add(Sphere::new_boxed(Vec3(-0.3, 0., -0.5), 0.05));
    world.add(Sphere::new_boxed(Vec3(-0.4, 0., -0.5), 0.05));
    world.add(Sphere::new_boxed(Vec3(-0.5, 0., -0.5), 0.05));
    world.add(Sphere::new_boxed(Vec3(-0.6, 0., -0.5), 0.05));
    world.add(Sphere::new_boxed(Vec3(0.4, 0., -0.8), 0.2));
    world.add(Sphere::new_boxed(Vec3(0., 0., -1.), 0.5));
    world.add(Sphere::new_boxed(Vec3(0., -100.5, -1.), 100.));


    let horizontal: Vec3 = Vec3(viewport_width, 0., 0.);
    let vertical: Vec3 = Vec3(0., viewport_height, 0.);
    let lower_left_corner: Vec3 =
        origin - horizontal / 2. - vertical / 2. - Vec3(0., 0., focal_length);

    let mut file = File::create("test1.ppm")?;
    file.write_all(format!("P3\n{} {}\n{}\n", image_width, image_height, max_color).as_bytes())?;

    let start_time = time::Instant::now();
    for i_y in (0..image_height).rev() {
        // go from top to bottom
        print!("\rLines remaining: {}  ", i_y);
        for i_x in 0..image_width {
            let u: f64 = i_x as f64 / (image_width - 1) as f64;
            let v: f64 = i_y as f64 / (image_height - 1) as f64;

            let r: Ray = Ray {
                origin: origin,
                direction: lower_left_corner + u * horizontal + v * vertical - origin,
            };

            let col: Color = r.ray_color(&world);
            // println!("r = {}, g = {}, b = {}", r, g, b);

            file.write_all(format!("{}", col).as_bytes())?;
        }
    }
    let duration = time::Instant::elapsed(&start_time);

    println!("\nDone.");
    println!("Execution time: {:?}", duration);

    Ok(())
}
