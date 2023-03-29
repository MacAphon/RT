use crate::hittable::hit_record::HitRecord;
use crate::ray::Ray;

pub mod hit_record;
pub mod hittable_list;
pub mod sphere;

pub trait Hittable: Send + Sync {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}
