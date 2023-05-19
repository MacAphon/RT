mod camera;
mod hittable;
mod material;
mod ray;
mod util;
mod vec3;

use crate::camera::Camera;
use crate::hittable::hittable_list::HittableList;
use crate::ray::Ray;
use crate::util::print_progress;
use crate::vec3::{Color, Point3, Vec3};
use clap::Parser;
use image::Rgb;
use rand::random;
use scoped_threadpool::Pool;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};

const DEFAULT_PATH: &str = "output/out.png";
const DEFAULT_HEIGHT: u32 = 256;
const DEFAULT_WIDTH: u32 = 341; // 4/3 * 256
const DEFAULT_SAMPLES: usize = 100;
const DEFAULT_DEPTH: usize = 16;
const DEFAULT_POOLS: u32 = 8;

const CAMERA_ORIGIN: Point3 = Point3::new(2., 0.5, 0.);
const CAMERA_TARGET: Point3 = Point3::new(0., 0.5, 0.);
//const CAMERA_ORIGIN: Point3 = Point3::new(13., 2., 3.);
//const CAMERA_TARGET: Point3 = Point3::new(0., 0., 0.);
//const V_FOV: f64 = 15.;
const V_UP: Vec3 = Vec3::new(0., 1., 0.);
const V_FOV: f64 = 40.; // FOV in the vertical axis
const APERTURE: f64 = 0.02;
const FOCUS_DIST: f64 = 2.;

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

    /// Number of parallel threads
    #[arg(short, long, default_value_t=DEFAULT_POOLS)]
    threads: u32,
}

fn main() -> Result<(), ()> {
    let cli = Cli::parse();

    let out_path: PathBuf = cli.write.into();
    let height: u32 = cli.height;
    let width: u32 = cli.width;
    let samples: usize = cli.samples;
    let max_depth: usize = cli.depth;
    let num_pools: u32 = cli.threads;

    eprintln!("Starting render:");
    eprintln!("- {} by {} pixels", height, width);
    eprintln!("- {} samples per pixel", samples);
    eprintln!("- up to {} bounces per sample\n", max_depth);

    let cam: Camera = Camera::new(
        CAMERA_ORIGIN,
        CAMERA_TARGET,
        V_UP,
        width as f64 / height as f64,
        V_FOV,
        APERTURE,
        FOCUS_DIST,
    );

    let world: Arc<HittableList> = Arc::new(hittable::hittable_list::generate_world());

    /* let mut children = vec![];
        let mut finished: usize = 0;
        print_progress(finished as u32, height);
    */
    let mut pool = Pool::new(num_pools);

    let image: Arc<Mutex<Vec<Vec<Rgb<u8>>>>> =
        Arc::new(Mutex::new(vec![
            vec![Rgb::from([0, 0, 0]); width as usize];
            height as usize
        ]));

    print_progress(0, 1);

    pool.scoped(|scope| {
        for y in 0..height {
            for x in 0..width {
                let world_cl = world.clone();
                let image_cl = image.clone();
                scope.execute(move || {
                    let mut col: Color = Color::new(0., 0., 0.);

                    for _ in 0..samples {
                        let u: f64 = (x as f64 + random::<f64>()) / width as f64;
                        let v: f64 = ((height - y as u32) as f64 + random::<f64>()) / height as f64;

                        let ray: Ray = cam.get_ray(u, v);

                        col += ray.ray_color(&world_cl, 16);
                    }

                    let mut image_cl = image_cl.lock().unwrap();
                    image_cl[y as usize][x as usize] = col.to_rgb_pixel(samples);
                    print_progress(y*width + x, width*height);
                })
            }
        }
    });

    let mut imgbuf = image::ImageBuffer::new(width, height);

    for (y, row) in Arc::<Mutex<Vec<Vec<Rgb<u8>>>>>::try_unwrap(image)
        .ok()
        .unwrap()
        .into_inner()
        .unwrap()
        .iter()
        .enumerate()
    {
        for (x, pixel) in row.iter().enumerate() {
            imgbuf.put_pixel(x as u32, y as u32, pixel.to_owned())
        }
    }

    match imgbuf.save(out_path) {
        Ok(_) => {}
        Err(e) => {
            eprintln!("{}", e);
            return Err(());
        }
    }

    Ok(())
}
