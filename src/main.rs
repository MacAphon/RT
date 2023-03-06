mod hittable;
mod material;
mod vec3;
mod util;

use std::path::PathBuf;
use clap::Parser;
use image;

const DEFAULT_PATH: &str = "out.png";
const DEFAULT_HEIGHT: usize = 256;
const DEFAULT_WIDTH: usize = 341;
const DEFAULT_SAMPLES: usize = 50;

#[derive(Parser)]
struct Cli {
    /// Output file, default is out.png
    #[arg(short, long)]
    write: Option<PathBuf>,

    /// Height of the rendered image, default is 256
    #[arg(short, long)]
    height: Option<usize>,

    /// Width of the rendered image, default is 341
    #[arg(long, short)]
    width: Option<usize>,

    /// Number of samples per Pixel, default is 50
    /// A higher number means higher image quality, but also higher calculation times.
    #[arg(long, short)]
    samples: Option<usize>
}

fn main() -> Result<(), ()> {
    let cli = Cli::parse();

    let out_path: PathBuf = match cli.write {
        Some(path) => path,
        None => DEFAULT_PATH.parse().unwrap(),
    };

    let height: usize = match cli.height {
        Some(height) => height,
        None => DEFAULT_HEIGHT,
    };



    Ok(())
}