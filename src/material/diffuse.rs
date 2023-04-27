use crate::hittable::hit_record::HitRecord;
use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::{Color, Vec3};

pub struct Diffuse {
    albedo: Color,
}

unsafe impl Send for Diffuse {}
unsafe impl Sync for Diffuse {}

impl  Diffuse {
    pub fn scatter(&self, record: &HitRecord) -> Option<(Ray, Color)> {
        let mut direction: Vec3 = record.normal + Vec3::new_random_unit_vector();
        // other possibilities:
        // new_random_in_unit_sphere();
        // new_random_in_hemisphere(record.normal);

        // catch degenerate scatter direction
        if direction.is_near_zero() {
            direction = record.normal;
        }
        let scattered = Ray {
            origin: record.pos,
            direction,
        };
        let attenuation: Color = self.albedo;
        Some((scattered, attenuation))
    }

    pub fn new(albedo: Color) -> Diffuse {
        Diffuse { albedo }
    }
}
