use crate::ray::Ray;
use crate::vec3::Vec3;

pub struct HitRecord {
    pub p: Vec3,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool,
}

impl HitRecord {
    pub fn new(p: Vec3,
               t: f64,
               r: Ray,
               outward_normal: Vec3
    ) -> HitRecord {

        let front_face = r.direction.dot(outward_normal) < 0.;
        let normal = if front_face { outward_normal } else { -outward_normal };

        HitRecord{
            p,
            t,
            normal,
            front_face,
        }
    }
}

pub trait Hittable {
    fn hit(self, r: Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}

pub struct HittableList {
    objects: Vec<Box<dyn Hittable>>
}

impl Hittable for HittableList {
    fn hit(self, r: Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        panic!();
    }
}