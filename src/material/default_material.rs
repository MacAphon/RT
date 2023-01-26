use crate::color::Color;
use crate::hittable::hit_record::HitRecord;
use crate::material::Material;
use crate::ray::Ray;

#[derive(Debug, Default, Clone, Copy)]
pub struct DefaultMaterial {}

impl Material for DefaultMaterial {
    fn scatter(&self, _: &Ray, _: &HitRecord) -> (bool, Ray, Color) {
        (true, Ray::default(), Color::default())
    }
}