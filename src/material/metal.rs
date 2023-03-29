use crate::hittable::hit_record::HitRecord;
use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::{Color, Vec3};

#[derive(Clone)]
pub struct Metal {
    albedo: Color,
    fuzz: f64,
}

unsafe impl Send for Metal {}
unsafe impl Sync for Metal {}

impl Material for Metal {
    fn scatter(&self, r_in: &Ray, record: &HitRecord) -> Option<(Ray, Color)> {
        let reflected: Vec3 = r_in.direction.unit_vector().reflect(record.normal);
        let scattered: Ray = Ray {
            origin: record.pos,
            direction: reflected + self.fuzz * Vec3::new_random_inside_unit_sphere(),
        };
        let attenuation: Color = self.albedo;
        let reflected: bool = scattered.direction.dot(record.normal) > 0.;
        if reflected {
            Some((scattered, attenuation))
        } else {
            None
        }
    }
}

impl Metal {
    pub fn new(albedo: Color, fuzz: f64) -> Metal {
        Metal { albedo, fuzz }
    }
}
