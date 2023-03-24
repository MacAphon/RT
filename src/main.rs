mod camera;
mod hittable;
mod material;
mod ray;
mod util;
mod vec3;

use crate::camera::Camera;
use crate::util::print_progress;
use crate::vec3::{Color, Point3, Vec3};
use clap::Parser;
use std::path::PathBuf;
use std::thread::sleep;
use std::time::Duration;
use crate::hittable::hittable_list::HittableList;
use crate::hittable::sphere::Sphere;
use crate::ray::Ray;

const DEFAULT_PATH: &str = "output/out.png";
const DEFAULT_HEIGHT: u32 = 256;
const DEFAULT_WIDTH: u32 = 341; // 4/3 * 256
const DEFAULT_SAMPLES: usize = 50;
const DEFAULT_DEPTH: usize = 16;

const CAMERA_ORIGIN: Point3 = Point3 {
    x: 0.,
    y: 0.,
    z: 0.,
};
const CAMERA_TARGET: Point3 = Point3 {
    x: 0.,
    y: 0.,
    z: -2.,
};
const V_UP: Vec3 = Vec3 {
    x: 0.,
    y: 1.,
    z: 0.,
};
const V_FOV: f64 = 90.;

#[derive(Parser)]
struct Cli {
    /// Output file
    #[arg(short, long, default_value_t=DEFAULT_PATH.into())]
    write: String,

    /// Height of the rendered image
    #[arg(short='H', long, default_value_t=DEFAULT_HEIGHT)]
    height: u32,

    /// Width of the rendered image
    #[arg(short='W', long, default_value_t=DEFAULT_WIDTH)]
    width: u32,

    /// Number of samples per Pixel.
    /// A higher number means higher image quality, but also increased rendering times.
    #[arg(short, long, default_value_t=DEFAULT_SAMPLES)]
    samples: usize,

    /// Maximum number of bounces per ray.
    /// A higher number means higher image quality, but also increased rendering times.
    #[arg(short, long, default_value_t=DEFAULT_DEPTH)]
    depth: usize,
}

fn main() -> Result<(), ()> {
    let cli = Cli::parse();

    let out_path: PathBuf = cli.write.into();
    let height: u32 = cli.height;
    let width: u32 = cli.width;
    let samples: usize = cli.samples;
    let max_depth: usize = cli.depth;

    eprintln!("Starting render:");
    eprintln!("- {} by {} pixels", height, width);
    eprintln!("- {} samples per pixel", samples);
    eprintln!("- up to {} bounces per sample\n", max_depth);

    let mut imgbuf = image::ImageBuffer::new(width, height);

    let cam: Camera = Camera::new(
        CAMERA_ORIGIN,
        CAMERA_TARGET,
        V_UP,
        width as f64 / height as f64,
        V_FOV,
    );

    let mut world: HittableList = HittableList::new();

    world.add(
        Box::new(Sphere {
            center: Point3 {
                x: 0.,
                y: 0.,
                z: -2.,
            },
            radius: 1.,
        })
    );

    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        let u: f64 = (width - x) as f64 / width as f64;
        let v: f64 = (height - y) as f64 / height as f64;

        let mut col: Color = Color::new(0., 0., 0.);
        let ray: Ray = cam.get_ray(u, v);
        col += ray.ray_color(&world, 1);
        *pixel = col.to_rgb_pixel();

        print_progress(y * width + x, width * height)
    }

    /*    for i in 0..101 {
        print_progress(i, 100);
        sleep(Duration::from_millis(100));
    }*/

    match imgbuf.save(out_path) {
        Ok(_) => {}
        Err(e) => {
            eprintln!("{}", e);
            return Err(())
        },
    }

    Ok(())
}
