use rand::random;
use crate::color::Color;
use crate::hittable::hit_record::HitRecord;
use crate::material::Material;
use crate::ray::Ray;
use crate::util::{min_f64, rand_f64};
use crate::vec3::Vec3;

#[derive(Debug, Default, Clone, Copy)]
pub struct Dielectric {
    ir: f64,
}

impl Material for Dielectric {
    fn scatter(&self, r_in: &Ray, record: &HitRecord) -> (bool, Ray, Color) {
        let attenuation = Color::new_from_vec3(Vec3(1., 1., 1.));
        let refraction_ratio = if record.front_face { 1./self.ir } else { self.ir };

        let unit_direction = r_in.direction.unit_vector();


        let cos_theta = min_f64((-unit_direction).dot(record.normal), 1.);
        let sin_theta = (1.-cos_theta.powi(2)).sqrt();

        let cannot_refract = refraction_ratio * sin_theta > 1.;

        let direction = if cannot_refract || Dielectric::reflectance(cos_theta, refraction_ratio) > random::<f64>() {
            // must reflect
            unit_direction.reflect(record.normal)
        }
        else {
            // can refract
            unit_direction.refract(record.normal, refraction_ratio)
        };

        let scattered = Ray{ origin: record.p, direction };
        (true, scattered, attenuation)
    }
}

impl Dielectric {
    pub fn new(ir: f64) -> Dielectric {
        Dielectric{ ir }
    }

    fn reflectance(cosine: f64, ref_idx: f64) -> f64{
        // Schlick's approximation for reflectance
        let r0 = ((1. - ref_idx) / (1. + ref_idx)).powi(2);
        r0 + (1. - r0)*(1. - cosine).powi(5)

    }
}