use crate::hittable::hit_record::HitRecord;
use crate::ray::Ray;
use crate::vec3::Color;

pub mod dielectric;
pub mod diffuse;
pub mod metal;
pub mod light;

pub enum Material {
    Metal(metal::Metal),
    Diffuse(diffuse::Diffuse),
    Dielectric(dielectric::Dielectric),
    Light(light::Light),
}

pub(crate) fn scatter(mat: &Material, r_in: &Ray, record: &HitRecord) -> Option<(Ray, Color)> {
    match mat {
        Material::Metal(m) => m.scatter(r_in, record),
        Material::Diffuse(m) => m.scatter(record),
        Material::Dielectric(m) => m.scatter(r_in, record),
        Material::Light(m) => m.scatter(r_in),
    }
}