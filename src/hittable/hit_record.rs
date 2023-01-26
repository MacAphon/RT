use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::Vec3;

#[derive(Debug, Default, Clone)]
pub struct HitRecord {
    pub p: Vec3,
    pub normal: Vec3,
    pub mat: Box<dyn Material>,
    pub t: f64,
    pub front_face: bool,
}

impl HitRecord {
    pub fn new(t: f64,
               p: Vec3,
               r: &Ray,
               mat: Box<dyn Material>,
               outward_normal: Vec3
    ) -> HitRecord {

        let front_face = r.direction.dot(outward_normal) < 0.;
        let normal = if front_face { outward_normal } else { -outward_normal };

        HitRecord{
            p,
            normal,
            mat,
            t,
            front_face,
        }
    }
}