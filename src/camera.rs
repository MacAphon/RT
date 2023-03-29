use crate::ray::Ray;
use crate::vec3::{Point3, Vec3};

#[derive(Copy, Clone)]
pub struct Camera {
    origin: Point3,
    lower_left_corner: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
    u: Vec3,
    v: Vec3,
    w: Vec3,
    lens_radius: f64,
}

impl Camera {
    pub fn new(
        origin: Point3,
        target: Point3,
        vector_up: Vec3,
        aspect_ratio: f64,
        vertical_fov: f64, // in degrees
        aperture: f64,
        focus_dist: f64,
    ) -> Camera {
        let w: Vec3 = (origin - target).unit_vector();
        let u: Vec3 = vector_up.cross(w).unit_vector();
        let v: Vec3 = w.cross(u);

        let theta: f64 = vertical_fov.to_radians();
        let h: f64 = (theta / 2.).tan();

        let viewport_height: f64 = 2. * h;
        let viewport_width: f64 = aspect_ratio * viewport_height;

        let vertical: Vec3 = focus_dist * viewport_height * v;
        let horizontal: Vec3 = focus_dist * viewport_width * u;

        let lower_left_corner: Vec3 = origin - horizontal / 2. - vertical / 2. - focus_dist * w;

        let lens_radius: f64 = aperture / 2.;

        Camera {
            origin,
            lower_left_corner,
            horizontal,
            vertical,
            w,
            u,
            v,
            lens_radius,
        }
    }

    pub fn get_ray(&self, s: f64, t: f64) -> Ray {
        let rd: Vec3 = self.lens_radius * Vec3::new_random_in_unit_disk();
        let offset: Vec3 = self.u * rd.x + self.v * rd.y;
        Ray {
            origin: self.origin + offset,
            direction: self.lower_left_corner + s * self.horizontal + t * self.vertical
                - self.origin
                - offset,
        }
    }
}
