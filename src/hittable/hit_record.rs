use crate::material::Material;
use crate::vec3::{Point3, Vec3};

pub struct HitRecord<'a> {
    pub pos: Point3,
    pub normal: Vec3,
    pub t: f64,
    pub material: &'a Box<dyn Material>,
}
