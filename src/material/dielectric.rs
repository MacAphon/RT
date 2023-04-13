use crate::hittable::hit_record::HitRecord;
use crate::material::Material;
use crate::ray::Ray;
use crate::util::min_v;
use crate::vec3::{Color, Vec3};
use rand::random;

pub struct Dielectric {
    ir: f64,
    attenuation: Color,
}

unsafe impl Send for Dielectric {}
unsafe impl Sync for Dielectric {}

impl Material for Dielectric {
    fn scatter(&self, r_in: &Ray, record: &HitRecord) -> Option<(Ray, Color)> {
        let refraction_ratio: f64 = if record.front_face {
            1. / self.ir
        } else {
            self.ir
        };
        let unit_direction: Vec3 = r_in.direction.unit_vector();

        let cos_theta: f64 = min_v((-unit_direction).dot(record.normal), 1.);
        let sin_theta: f64 = (1. - cos_theta.powi(2)).sqrt();

        let cannot_refract: bool = refraction_ratio * sin_theta > 1.;

        let direction: Vec3 = if cannot_refract
            || Dielectric::reflectance(cos_theta, refraction_ratio) > random::<f64>()
        {
            // must reflect at shallow angles
            unit_direction.reflect(record.normal)
        } else {
            // can refract
            unit_direction.refract(record.normal, refraction_ratio)
        };

        let scattered = Ray {
            origin: record.pos,
            direction,
        };

        Some((scattered, self.attenuation))
    }
}

impl Dielectric {
    pub fn new(ir: f64, attenuation: Color) -> Dielectric {
        Dielectric { ir, attenuation }
    }

    pub fn new_clear(ir: f64) -> Dielectric {
        Dielectric {
            ir,
            attenuation: Color::new_color(1., 1., 1.),
        }
    }

    pub fn reflectance(cos: f64, ref_idx: f64) -> f64 {
        // Schlick's approximation for reflectance
        let r0: f64 = ((1. - ref_idx) / (1. + ref_idx)).powi(2);
        r0 + (1. - r0) * (1. - cos).powi(5)
    }
}
