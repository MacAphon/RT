use std::fmt;
use crate::vec3::Vec3;

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Color {
    pub color: Vec3,
    pub samples: usize,
    pub max: f64,
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut r = self.color.x();
        let mut g = self.color.y();
        let mut b = self.color.z();

        let scale = 1. / self.samples as f64;

        r = (r * scale).sqrt();
        g = (g * scale).sqrt();
        b = (b * scale).sqrt();

        writeln!(f, "{} {} {}",
                 (r * self.max) as usize,
                 (g * self.max) as usize,
                 (b * self.max) as usize,
        )
    }
}

impl Color {
    pub fn new(r: f64, g: f64, b: f64, samples: usize, max: f64) -> Color {
        Color { color: Vec3(r, g, b), samples, max}
    }
    pub fn new_with_default(r: f64, g: f64, b: f64) -> Color {
        Color { color: Vec3(r, g, b), samples: 1, max: 255.999 }
    }
    pub fn new_from_vec3(v: Vec3) -> Color {
        Color {color: v, samples: 1, max: 255.999}
    }
    pub fn set_color(mut self, r: f64, g: f64, b: f64) {
        self.color.0 = r;
        self.color.1 = g;
        self.color.2 = b;

    }
    pub fn set_color_vec3(mut self, c: Vec3) {
        self.color = c;
    }
}