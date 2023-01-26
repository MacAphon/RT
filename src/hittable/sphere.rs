use crate::vec3::Vec3;
use crate::ray::Ray;
use crate::hittable::*;
use crate::material::Material;

#[derive(Default, Debug, Clone)]
pub struct Sphere{
    pub center: Vec3,
    pub radius: f64,
    pub material: Box<dyn Material>,
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
        let mat = self.material.clone();

        Some(HitRecord::new(
            root,  // t
            p,
            r,
            mat,
            outward_normal,
        ))
    }
}

unsafe impl Send for Sphere {}

impl Sphere {
    pub fn new_boxed(center: Vec3, radius: f64, material: Box<dyn Material>) -> Box<dyn Hittable> {
        Box::new(Sphere { center, radius, material })
    }
}
