use crate::color::Color;
use crate::hittable::hit_record::HitRecord;
use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::Vec3;

#[derive(Debug, Default, Clone, Copy)]
pub struct Lambertian {
    albedo: Color,
}

impl Material for Lambertian {
    fn scatter(&self, _: &Ray, record: &HitRecord) -> (bool, Ray, Color) {
        let mut scatter_direction = record.normal + Vec3::new_random_unit_vector();
        // Catch degenerate scatter direction
        if scatter_direction.is_near_zero() {
            scatter_direction = record.normal;
        }
        let scattered = Ray{ origin: record.p, direction: scatter_direction };
        let attenuation = self.albedo;
        (true, scattered, attenuation)
    }
}

impl Lambertian {
    pub fn new(albedo: Color) -> Lambertian {
        Lambertian{ albedo }
    }
}