mod color;
mod hittable;
mod ray;
mod sphere;
mod vec3;

use ray::Ray;
use vec3::Vec3;

use crate::color::Color;
use std::{fs::File, io::Write};

const ASPECT_RATIO: f64 = 4. / 3.;
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
    let origin: Vec3 = Vec3(0., 0., 0.);

    let mut horizontal: Vec3 = Vec3(viewport_width, 0., 0.);
    let mut vertical: Vec3 = Vec3(0., viewport_height, 0.);
    let lower_left_corner: Vec3 =
        origin - horizontal / 2. - vertical / 2. - Vec3(0., 0., focal_length);

    let mut file = File::create("test1.ppm")?;
    file.write_all(format!("P3\n{} {}\n{}\n", image_width, image_height, max_color).as_bytes())?;

    for i_y in (0..image_height).rev() {
        // go from top to bottom
        print!("\rLines remaining: {}  ", image_height - i_y);
        for i_x in 0..image_width {
            let u: f64 = i_x as f64 / (image_width - 1) as f64;
            let v: f64 = i_y as f64 / (image_height - 1) as f64;

            let r: Ray = Ray {
                origin: origin,
                direction: lower_left_corner + u * horizontal + v * vertical - origin,
            };

            let col: Color = r.ray_color();

            // println!("r = {}, g = {}, b = {}", r, g, b);

            file.write_all(format!("{}", col).as_bytes())?;
        }
    }
    println!("\nDone.");

    Ok(())
}
