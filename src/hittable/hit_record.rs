use crate::vec3::{Point3, Vec3};

pub struct HitRecord {
    pub(crate) pos: Point3,
    pub(crate) normal: Vec3,
    pub(crate) t: f64,
}