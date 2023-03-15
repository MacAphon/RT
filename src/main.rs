mod hittable;
mod material;
mod util;
mod vec3;
mod camera;
mod ray;

use clap::Parser;
use std::path::PathBuf;
use crate::vec3::Color;

const DEFAULT_PATH: &str = "output/out.png";
const DEFAULT_HEIGHT: u32 = 256;
const DEFAULT_WIDTH: u32 = 341; // 4/3
const DEFAULT_SAMPLES: usize = 50;
const DEFAULT_DEPTH: usize = 16;

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
    eprintln!("- up to {} bounces per sample", max_depth);

    let mut imgbuf = image::ImageBuffer::new(width, height);

    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        let col = Color::new(x as f64 / width as f64, y as f64 / height as f64, 0.25);
        *pixel = col.to_rgb_pixel();
    }

    match imgbuf.save(out_path) {
        Ok(_) => {},
        Err(e) => eprintln!("{}", e),
    }

    Ok(())
}
