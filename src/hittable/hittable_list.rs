use crate::hittable::hit_record::HitRecord;
use crate::hittable::Hittable;
use crate::ray::Ray;

#[derive(Default)]
pub struct HittableList {
    pub objects: Vec<Box<dyn Hittable>>,
}

impl HittableList {
    pub fn new() -> HittableList {
        HittableList {
            objects: Vec::new(),
        }
    }

    pub fn clear(&mut self) {
        self.objects.clear();
    }

    pub fn add(&mut self, rhs: Box<dyn Hittable>) {
        self.objects.push(rhs);
    }

    pub fn hit_anything(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut closest_so_far: f64 = t_max;
        let mut rec: Option<HitRecord> = None;

        for element in &self.objects {
            if let Some(temp_r) = element.hit(r, t_min, closest_so_far) {
                closest_so_far = temp_r.t;
                rec = Some(temp_r);
            }
        }
        rec
    }
}
