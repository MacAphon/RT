use crate::hittable::hit_record::HitRecord;
use crate::ray::Ray;
use crate::vec3::Color;

pub mod dielectric;
pub mod diffuse;
pub mod metal;

pub trait Material: Send + Sync {
    fn scatter(&self, r_in: &Ray, record: &HitRecord) -> Option<(Ray, Color)>;
}
