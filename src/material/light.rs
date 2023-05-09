use crate::{vec3::Color, ray::Ray};

pub struct Light {
    pub albedo: Color,
}

impl Light {
    pub fn scatter(&self, r_in: &Ray) -> Option<(Ray, Color)> {
        Some((r_in.clone(), self.albedo))
    }

    pub fn new(albedo: Color) -> Light {
        Light { albedo }
    }
}