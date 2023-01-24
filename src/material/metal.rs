use crate::color::Color;
use crate::hittable::HitRecord;
use crate::material::Material;
use crate::ray::Ray;

#[derive(Debug, Default, Clone, Copy)]
pub struct Metal {
    albedo: Color,
}

impl Material for Metal {
    fn scatter(&self, r_in: &Ray, record: &HitRecord) -> (bool, Ray, Color) {
        let reflected = r_in.direction.unit_vector().reflect(record.normal);
        let scattered = Ray{ origin: record.p, direction: reflected };
        let attenuation = self.albedo;
        let reflected = scattered.direction.dot(record.normal) > 0.;
        (reflected, scattered, attenuation)
    }
}

impl Metal {
    pub fn new(albedo: Color) -> Metal {
        Metal{ albedo }
    }
}