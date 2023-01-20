use crate::vec3::Vec3;
use crate::color::Color;

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Ray{
    pub origin: Vec3,
    pub direction: Vec3,
}

impl Ray {
    pub fn at(self, t: f64) -> Vec3 {
        self.origin + self.direction * t
    }

    pub fn ray_color(self) -> Color {
        let t = self.hit_sphere(Vec3(0., 0., -1.), 0.5);
        if t > 0. {
            let n = (self.at(t) - Vec3(0., 0., -1.)).unit_vector();
            return Color(0.5*Vec3(n.x()+1., n.y()+1., n.z()+1.));
        }
        let unit_direction = self.direction.unit_vector();
        let t = 0.5 * (unit_direction.y() + 1.);
        Color((1.-t) * Vec3(1., 1., 1.) + t * Color::new(0.5, 0.7, 1.0).0)
    }

    fn hit_sphere(self, center: Vec3, radius: f64) -> f64 {
        let oc = self.origin - center;
        let a = self.direction.length_squared();
        let half_b = oc.dot(self.direction);
        let c = oc.length_squared() - radius.powi(2);
        let discriminant = half_b.powi(2) - a*c;

        if discriminant < 0. {
            return -1.;
        }
        (-half_b - discriminant.sqrt()) / a
    }
}