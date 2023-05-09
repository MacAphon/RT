use crate::hittable::hit_record::HitRecord;
use crate::ray::Ray;
use crate::vec3::{Point3, Vec3};

pub mod hit_record;
pub mod hittable_list;
pub mod sphere;

/*pub trait Hittable: Send + Sync + Sized {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}*/

pub enum Hittable {
    Sphere(sphere::Sphere),
}

pub fn hit<'a>(obj: &'a Hittable, r: &'a Ray, t_min: f64, t_max: f64) -> Option<HitRecord<'a>> {
    match obj {
        Hittable::Sphere(o) => o.hit(r, t_min, t_max),
    }
}

pub fn distance(obj: &Hittable, start: Point3) -> f64 {
    match obj {
        Hittable::Sphere(o) => o.distance(start),
    }
}

pub fn direction_to(obj: &Hittable, origin: Point3) -> Vec3 {
    match obj {
        Hittable::Sphere(o) => o.direction_to(origin),
    }
}