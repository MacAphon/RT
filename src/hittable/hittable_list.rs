use crate::hittable::hit_record::HitRecord;
use crate::hittable::Hittable;
use crate::ray::Ray;

#[derive(Default, Clone)]
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