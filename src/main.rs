mod color;
mod hittable;
mod ray;
mod sphere;
mod vec3;
mod camera;
mod util;
mod material;

use std::{fs::File, io, io::Write, time};
use rand::random;

use crate::vec3::Vec3;
use crate::hittable::HittableList;
use crate::camera::Camera;
use crate::color::Color;
use crate::material::dielectric::Dielectric;
use crate::sphere::Sphere;
use crate::material::lambertian::Lambertian;
use crate::material::metal::Metal;

const ASPECT_RATIO: f64 = 16./9.;
// const ASPECT_RATIO: f64 = 4. / 3.;
const IMAGE_HEIGHT: u32 = 1080;
// const IMAGE_HEIGHT: u32 = 256;
const VIEWPORT_HEIGHT: f64 = 2.;
const FOCAL_LENGTH: f64 = 1.;
const ORIGIN: Vec3 = Vec3(0., 0., 0.);
const SAMPLES_PER_PIXEL: u32 = 50;
const MAX_DEPTH: u32 = 16;

fn main() -> io::Result<()> {
    let aspect_ratio = ASPECT_RATIO;
    let image_height: u32 = IMAGE_HEIGHT;
    let image_width: u32 = (IMAGE_HEIGHT as f64 * ASPECT_RATIO) as u32;
    let viewport_height: f64 = VIEWPORT_HEIGHT;
    let max_color: u32 = 255;
    let focal_length: f64 = FOCAL_LENGTH;
    let origin: Vec3 = ORIGIN;
    let samples_per_pixel: u32 = SAMPLES_PER_PIXEL;
    let max_depth: u32 = MAX_DEPTH;

    // World setup

    let ground_material = Lambertian::new(
            Color::new_from_vec3(
                Vec3(0.5, 0.5, 0.5)
            )
        );
    let center_material = Lambertian::new(
            Color::new_from_vec3(
                Vec3(1., 0.5, 0.2)
            )
    );
    let left_material = Metal::new(
        Color::new_from_vec3(
            Vec3(1., 0.5, 0.2)
        ),
        0.1,
    );
    let right_material = Metal::new(
        Color::new_from_vec3(
            Vec3(0.5, 0.7, 1.)
        ),
        1.,
    );
    let dielectric_left = Dielectric::new(1.5);

    let mut world: HittableList = Default::default();
    world.add(
        Sphere::new_boxed(
            Vec3(0., 0., -2.),
            0.5,
            Box::new(left_material),
        )
    );
    world.add(
        Sphere::new_boxed(
            Vec3(-1., 0., -2.),
            0.5,
            Box::new(dielectric_left),
        )
    );
    world.add(
        Sphere::new_boxed(
            Vec3(1., 0., -2.),
            0.5,
            Box::new(right_material),
        )
    );
    world.add(
        Sphere::new_boxed(
            Vec3(0., -100.5, -2.),
            100.,
            Box::new(ground_material),
        )
    );

    // Camera

    let cam: Camera = Camera::new(aspect_ratio, viewport_height, focal_length, origin);

    // prepare output ppm-file

    let mut file = File::create("test1.ppm")?;
    file.write_all(format!("P3\n{} {}\n{}\n", image_width, image_height, max_color).as_bytes())?;

    let start_time = time::Instant::now();

    // go from top to bottom
    for i_y in (0..image_height).rev() {

        // Write to stdout on every iteration. `print!()` will only write sometimes
        io::stdout().write_all(format!("\rLines remaining: {}  ", i_y).as_bytes())?;
        io::stdout().flush()?;

        for i_x in 0..image_width {
            let mut pixel_color = Color::new(0., 0., 0., samples_per_pixel, 255.999);

            for _ in 0..samples_per_pixel {
                let u: f64 = (i_x as f64 + random::<f64>()) / (image_width) as f64;
                let v: f64 = (i_y as f64 + random::<f64>()) / (image_height) as f64;
                let ray = cam.get_ray(u, v);
                pixel_color.color += ray.ray_color(&world, max_depth).color;
            }

            file.write_all(format!("{}", pixel_color).as_bytes())?;
        }
    }

    let duration = time::Instant::elapsed(&start_time);

    println!("\nDone.");
    println!("Execution time: {:?}", duration);

    Ok(())
}
