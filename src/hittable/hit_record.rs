use std::rc::Rc;
use std::sync::Arc;
use crate::material::{DefaultMaterial, Material};
use crate::ray::Ray;
use crate::vec3::{Point3, Vec3};

pub struct HitRecord {
    pub pos: Point3,
    pub normal: Vec3,
    pub t: f64,
    pub material: Arc<Box<dyn Material>>,
    pub front_face: bool,
}

impl HitRecord {
    pub fn new<'a>(
        r: &Ray,
        pos: Vec3,
        t: f64,
        material: Arc<Box<dyn Material>>,
        outward_normal: Vec3,
    ) -> HitRecord {
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

    pub fn set_front_face(&mut self, r: &Ray, outward_normal: Vec3) {
        {
            self.front_face = r.direction.dot(outward_normal) < 0.;
            self.normal = if self.front_face {
                outward_normal
            } else {
                -outward_normal
            };
        }
    }

    pub fn new_default() -> HitRecord {
        HitRecord {
            pos: Default::default(),
            normal: Default::default(),
            t: 0.0,
            material: Arc::new(Box::new(DefaultMaterial {})),
            front_face: false,
        }
    }
}
