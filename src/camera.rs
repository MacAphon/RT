use crate::ray::Ray;
use crate::vec3::Vec3;

#[derive(Debug, Copy, Clone)]
pub struct Camera {
    origin: Vec3,
    lower_left_corner: Vec3,
    aspect_ratio: f64,
    horizontal: Vec3,
    vertical: Vec3,
    viewport_width: f64,
    viewport_height: f64,
}

impl Camera {
    pub fn new(aspect_ratio: f64, origin: Vec3, target: Vec3, vup: Vec3, vfov: f64) -> Camera {

        let w = (origin - target).unit_vector();
        let u = vup.cross(w).unit_vector();
        let v = w.cross(u);

        let theta = vfov.to_radians();
        let h = (theta/2.).tan();
        let viewport_height = 2. * h;
        let viewport_width = viewport_height * aspect_ratio;

        let vertical = viewport_height * v;
        let horizontal = viewport_width * u;

        let lower_left_corner = origin - horizontal/2. - vertical/2. - w;
        Camera {
            origin,
            viewport_height,
            viewport_width,
            vertical,
            horizontal,
            lower_left_corner,
            aspect_ratio,
        }
    }
    pub fn get_ray(&self, s: f64, t: f64) -> Ray {
        Ray {
            origin: self.origin,
            direction: self.lower_left_corner + s * self.horizontal + t*self.vertical - self.origin,
        }
    }
}
