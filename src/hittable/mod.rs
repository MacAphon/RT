use crate::hittable::hit_record::HitRecord;
use crate::ray::Ray;

pub mod hit_record;
pub mod hittable_list;
pub mod sphere;

pub trait Hittable: CloneHittable + Send + Sync {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}

pub trait CloneHittable {
    fn clone_hittable<'a>(&self) -> Box<dyn Hittable>;
}

impl<T> CloneHittable for T
    where
        T: Hittable + Clone + 'static,
{
    fn clone_hittable<'a>(&self) -> Box<dyn Hittable> {
        Box::new(self.clone())
    }
}

impl Clone for Box<dyn Hittable> {
    fn clone(&self) -> Self {
        self.clone_hittable()
    }
}