use crate::hittable::hit_record::HitRecord;
use crate::hittable::Hittable;
use crate::ray::Ray;
use crate::vec3::{Point3, Vec3};

#[derive(Clone)]
pub struct Sphere {
    pub(crate) center: Point3,
    pub(crate) radius: f64,
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc: Vec3 = r.origin - self.center;
        let a: f64 = r.direction.length_squared();
        let half_b: f64 = oc.dot(r.direction);
        let c: f64 = oc.length_squared() - self.radius*self.radius;

        let discriminant: f64 = half_b*half_b - a*c;
        if discriminant < 0. {
            return None;
        }
        let sqrt_d: f64 = discriminant.sqrt();

        let mut root: f64 = (-half_b - sqrt_d) / a;
        if root < t_min || root > t_max {
            root = (-half_b + sqrt_d) / a;
            if root < t_min || root > t_max {
                return None;
            }
        }
        let pos: Point3 = r.at(root);
        let outward_normal: Vec3 = (pos - self.center) / self.radius;

        Some(HitRecord{
            pos,
            normal: outward_normal,
            t: root,
        })
    }
}