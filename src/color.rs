use std::fmt;
use crate::vec3::Vec3;

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Color(pub Vec3);

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "{} {} {}",
               (self.0.x() * 255.999) as u8,
               (self.0.y() * 255.999) as u8,
               (self.0.z() * 255.999) as u8,
        )
    }
}

impl Color {
    pub fn new(x: f64, y: f64, z: f64) -> Color {
        Color(Vec3(x, y, z))
    }
}