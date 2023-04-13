use crate::hittable::hit_record::HitRecord;
use crate::hittable::hittable_list::HittableList;
use crate::vec3::{Color, Point3, Vec3};

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Ray {
    pub(crate) origin: Point3,
    pub(crate) direction: Vec3,
}

impl Ray {
    pub fn at(&self, t: f64) -> Point3 {
        self.origin + self.direction * t
    }

    pub fn ray_color(&self, rec: &mut HitRecord, world: &HittableList, depth: usize) -> Color {
        if depth == 0 {
            return Color::BLACK;
        }

        if world.hit_anything(self, rec, 0.001, f64::INFINITY) {
            return if let Some((ray, color)) = rec.material.scatter(self, &rec) {
                color * ray.ray_color(rec, world, depth - 1)
            } else {
                Color::BLACK
            };
        }

        // if we don't hit anything, draw the background
        let unit_direction: Vec3 = self.direction.unit_vector();
        let t: f64 = 0.5 * (unit_direction.y + 1.);
        (1. - t) * Color::WHITE + t * Color::new_color(0.5, 0.7, 1.)
    }
}
