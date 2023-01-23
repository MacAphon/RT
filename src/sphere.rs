use crate::vec3::Vec3;
use crate::ray::Ray;
use crate::hittable::*;

pub struct Sphere{
    pub center: Vec3,
    pub radius: f64,
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = r.origin - self.center;
        let a = r.direction.length_squared();
        let half_b = oc.dot(r.direction);
        let c = oc.length_squared() - self.radius*self.radius;

        let discriminant = half_b*half_b - a*c;
        if discriminant < 0. {
            return None;
        }
        let sqrt_d = discriminant.sqrt();

        let mut root = (-half_b - sqrt_d) / a;
        if root < t_min || root > t_max {
            root = (-half_b + sqrt_d) / a;
            if root < t_min || root > t_max {
                return None;
            }
        }
        let p = r.at(root);
        let outward_normal = (p - self.center) / self.radius;

        Some(HitRecord::new(
            root,  // t
            p,
            r,
            outward_normal,
        ))
    }
}

impl Sphere {
    pub fn new_boxed(center: Vec3, radius: f64) -> Box<dyn Hittable> {
        Box::new(Sphere { center, radius })
    }
}
