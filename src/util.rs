use rand::random;
use std::io;
use std::io::Write;

pub fn clamp<T: PartialOrd>(v: T, min: T, max: T) -> T {
    if v < min {
        min
    } else if v > max {
        max
    } else {
        v
    }
}

/// Returns a random value in the range [min, max)
/// ( other notation: [min, max[ )
pub fn rand_f64(min: f64, max: f64) -> f64 {
    min + (max - min) * random::<f64>()
}

pub fn min_f64(x: f64, y: f64) -> f64 {
    if x < y {
        x
    } else {
        y
    }
}

/// Draw a simple progress bar to stderr, writing over the current line.\
/// Example output:\
/// ` 0,   5 -> \r  0% [                                        ]`\
/// `77, 154 -> \r 50% [====================                    ]`\
/// `13,  13 -> \r100% [========================================]`
pub fn print_progress<T: Into<f64>>(progress: T, max: T) {
    let progress: f64 = progress.into() / max.into();
    let progress_number: String = (progress * 100.).round().to_string();
    let number_padding: String = " ".repeat(3 - progress_number.len());
    let progress_bar: String = "=".repeat((progress * 40.).round() as usize);
    let bar_padding: String = " ".repeat(40 - progress_bar.len());

    io::stderr()
        .write_all(
            format!(
                "\r{}{}% [{}{}]",
                number_padding, progress_number, progress_bar, bar_padding
            )
            .as_bytes(),
        )
        .unwrap();
    io::stderr().flush().unwrap();
}

/*pub fn build_world() -> HittableList {
    let mut list: HittableList = Default::default();

    list.add(
        Sphere::ne
    )

    list
}*/
