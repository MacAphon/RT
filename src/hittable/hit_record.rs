use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::{Point3, Vec3};

pub struct HitRecord<'a> {
    pub pos: Point3,
    pub normal: Vec3,
    pub t: f64,
    pub material: &'a Box<dyn Material>,
    pub front_face: bool,
}

impl HitRecord<'_> {
    pub fn new<'a>(
        r: &Ray,
        pos: Vec3,
        t: f64,
        material: &'a Box<dyn Material>,
        outward_normal: Vec3,
    ) -> HitRecord<'a> {
        let front_face: bool = r.direction.dot(outward_normal) < 0.;
        let normal: Vec3 = if front_face {
            outward_normal
        } else {
            -outward_normal
        };

        HitRecord {
            pos,
            normal,
            t,
            material,
            front_face,
        }
    }
}
