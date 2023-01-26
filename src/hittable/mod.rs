pub mod sphere;
pub mod hit_record;
pub mod hittable_list;

use crate::hittable::hit_record::HitRecord;
use crate::ray::Ray;

pub trait Hittable {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}
