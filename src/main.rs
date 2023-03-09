mod hittable;
mod material;
mod util;
mod vec3;

use clap::Parser;
use std::path::PathBuf;

const DEFAULT_PATH: &str = "output/out.png";
const DEFAULT_HEIGHT: u32 = 256;
const DEFAULT_WIDTH: u32 = 341; // 4/3
const DEFAULT_SAMPLES: usize = 50;

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
}

fn main() -> Result<(), ()> {
    let cli = Cli::parse();

    let out_path: PathBuf = cli.write.into();
    let height: u32 = cli.height;
    let width: u32 = cli.width;
    let samples: usize = cli.samples;

    let mut imgbuf = image::ImageBuffer::new(width, height);

    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        let r = (255. * x as f64 / width as f64) as u8;
        let g = (255. * y as f64 / height as f64) as u8;
        *pixel = image::Rgb([r, g, 0]);
    }

    match imgbuf.save(out_path) {
        Ok(_) => {},
        Err(e) => eprintln!("{}", e),
    }

    Ok(())
}
