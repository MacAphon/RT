use crate::color::Color;
use crate::hittable::hit_record::HitRecord;
use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::Vec3;

#[derive(Debug, Default, Clone, Copy)]
pub struct Metal {
    albedo: Color,
    fuzz: f64,
}

impl Material for Metal {
    fn scatter(&self, r_in: &Ray, record: &HitRecord) -> (bool, Ray, Color) {
        let reflected = r_in.direction.unit_vector().reflect(record.normal);
        let scattered = Ray{ origin: record.p, direction: reflected + self.fuzz*Vec3::new_random_in_unit_sphere() };
        let attenuation = self.albedo;
        let reflected = scattered.direction.dot(record.normal) > 0.;
        (reflected, scattered, attenuation)
    }
}

impl Metal {
    pub fn new(albedo: Color, fuzz: f64) -> Metal {
        Metal{ albedo, fuzz: if fuzz < 1. { fuzz } else { 1. } }
    }
}