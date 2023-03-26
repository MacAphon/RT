use crate::hittable::hit_record::HitRecord;
use crate::ray::Ray;
use crate::vec3::Color;

pub mod metal;
pub mod diffuse;
pub mod dielectric;

pub trait Material {
    fn scatter(&self, r_in: &Ray, record: &HitRecord) -> Option<(Ray, Color)>;
}
