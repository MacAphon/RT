use crate::ray::Ray;
use crate::vec3::Vec3;

pub struct Camera {
    origin: Vec3,
    lower_left_corner: Vec3,
    aspect_ratio: f64,
    horizontal: Vec3,
    vertical: Vec3,
    viewport_width: f64,
    viewport_height: f64,
    focal_length: f64,
}

impl Camera {
    pub fn new(aspect_ratio: f64, viewport_height: f64, focal_length: f64, origin: Vec3) -> Camera {
        let viewport_width = viewport_height * aspect_ratio;
        let vertical = Vec3(0., viewport_height, 0.);
        let horizontal = Vec3(viewport_width, 0., 0.);
        let lower_left_corner = origin - horizontal/2. - vertical/2. - Vec3(0., 0., focal_length);
        Camera {
            origin,
            viewport_height,
            viewport_width,
            vertical,
            horizontal,
            lower_left_corner,
            aspect_ratio,
            focal_length,
        }
    }
    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        Ray {
            origin: self.origin,
            direction: self.lower_left_corner + u * self.horizontal + v*self.vertical - self.origin,
        }
    }
}
