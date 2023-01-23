use crate::vec3::Vec3;
use crate::color::Color;
use crate::hittable::*;

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Ray{
    pub origin: Vec3,
    pub direction: Vec3,
}

impl Ray {
    pub fn at(&self, t: f64) -> Vec3 {
        self.origin + self.direction * t
    }

    pub fn ray_color(&self, world: &HittableList, depth: u32) -> Color {
        if depth == 0 {
            return Color::new_from_vec3(Vec3(0., 0., 0.));
        }
        let mut rec : HitRecord = Default::default();
        if world.hit(self, 0., f64::INFINITY, &mut rec) {
            /* Solid color
            return Color::new_from_vec3(Vec3(1., 0., 0.));
            */
            let target = rec.p + rec.normal + Vec3::new_random_unit_vector();
            return  Color::new_from_vec3( 0.5 * Ray{origin: rec.p, direction: target-rec.p}.ray_color(&world, depth-1).color );
            /* Color with normal vector
            return Color::new_from_vec3( 0.5 * (rec.normal + Vec3(1., 1., 1.)) );
            */
        }
        // if we don't hit anything, draw the background
        let unit_direction = self.direction.unit_vector();
        let t = 0.5 * (unit_direction.y() + 1.);
        Color::new_from_vec3( (1. - t) * Vec3(1., 1., 1.) + t * Vec3(0.5, 0.7, 1.0), )
    }
}