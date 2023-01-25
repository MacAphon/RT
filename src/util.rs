use float_ord::sort;
use rand::random;

pub fn clamp<T: PartialOrd>(v: T, min: T, max: T) -> T {
    if v < min { min }
    else if v > max { max }
    else { v }
}

pub fn rand_f64(min: f64, max: f64) -> f64 {
    // [min, max) other notation: [min, max[
    min + (max - min) * random::<f64>()
}

pub fn min_f64(x: f64, y: f64) -> f64 {
    let mut t = vec![x, y];
    sort(&mut t);
    t[0]
}