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