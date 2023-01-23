use crate::ray::Ray;
use crate::vec3::Vec3;
use crate::color::Color;
use crate::material::Material;

#[derive(Debug, Default, Clone, PartialEq)]
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

pub trait Hittable {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}


#[derive(Default)]
pub struct HittableList {
    pub objects: Vec<Box<dyn Hittable>>,
}

impl HittableList {
    pub fn clear(&mut self) {
        self.objects.clear();
    }

    pub fn add(&mut self, rhs: Box<dyn Hittable>) {
        self.objects.push(rhs);
    }

    pub fn hit(&self, r: &Ray, t_min: f64, t_max: f64, record: &mut HitRecord) -> bool {
        let mut hit_anything: bool = false;
        let mut closest_so_far = t_max;

        for element in self.objects.iter() {
            if let Some(temp_r) = element.hit(r, t_min, closest_so_far) {
                hit_anything = true;
                closest_so_far = temp_r.t;
                temp_r.clone_into(record);
            }
        }
        hit_anything
    }
}