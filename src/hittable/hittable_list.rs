use crate::hittable::*;
use crate::hittable::hit_record::HitRecord;
use crate::hittable::sphere::Sphere;
use crate::hittable::Hittable;
use crate::material::dielectric::Dielectric;
use crate::material::diffuse::Diffuse;
use crate::material::light::Light;
use crate::material::metal::Metal;
use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::{Color, Point3};
use std::sync::Arc;

use super::distance;

#[derive(Default)]
pub struct HittableList {
    pub objects: Vec<Arc<Hittable>>,
    pub lights: Vec<Arc<Hittable>>,
}

impl HittableList {
    pub fn new() -> HittableList {
        HittableList {
            objects: Vec::new(),
            lights: Vec::new(),
        }
    }

    pub fn clear(&mut self) {
        self.objects.clear();
    }

    pub fn add(&mut self, rhs: Arc<Hittable>) {
        self.objects.push(rhs);
    }

    pub fn add_light(&mut self, other: Arc<Hittable>) {
        self.lights.push(other.clone());
        self.objects.push(other);
    }

    pub fn hit_anything<'a>(&'a self, r: &'a Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut closest_so_far: f64 = t_max;
        let mut rec: Option<HitRecord> = None;

        for element in &self.objects {
            if let Some(temp_r) = hit(element, r, t_min, closest_so_far) {
                closest_so_far = temp_r.t;
                rec = Some(temp_r);
            }
        }
        rec
    }

    pub fn light_ray(&self, origin: Point3) -> (Color, f64) {
        let mut closest_so_far: (f64, Option<Arc<Hittable>>) = (f64::INFINITY, None);

        for light in &self.lights {
            let dist = distance(&light, origin);
            if dist < closest_so_far.0 {
                closest_so_far = (dist, Some(light.clone()));
            }
        }

        if let Some(o) = closest_so_far.1 {
            let ray_to_object = Ray{origin, direction: direction_to(&o, origin)};

            if let Some(rec) = self.hit_anything(&ray_to_object, 0., f64::INFINITY) {
                match rec.material {
                    Material::Light(l) => return (l.albedo, closest_so_far.0),
                    _ => return (Color::BLACK, 0.),
                }
            }
        }

        (Color::BLACK, 0.)
    }
}

pub fn generate_world() -> HittableList {
    let mut world: HittableList = HittableList::new();

    // #########################################################################
    // Cornell Box

    world.add(Arc::new(Hittable::Sphere(Sphere { 
        center: Point3::new(0., -1000., 0.), 
        radius: 1000., 
        material: Material::Diffuse(Diffuse::new(Color::WHITE)), 
    })));

    world.add(Arc::new(Hittable::Sphere(Sphere { 
        center: Point3::new(0., 1001., 0.), 
        radius: 1000., 
        material: Material::Diffuse(Diffuse::new(Color::WHITE)), 
    })));

    world.add(Arc::new(Hittable::Sphere(Sphere { 
        center: Point3::new(-1000.5, 0.5, 0.), 
        radius: 1000., 
        material: Material::Diffuse(Diffuse::new(Color::WHITE)), 
    })));

    world.add(Arc::new(Hittable::Sphere(Sphere { 
        center: Point3::new(0., 0.5, 1000.5), 
        radius: 1000., 
        material: Material::Diffuse(Diffuse::new(Color::new_color(1., 0., 0.))), 
    })));

    world.add(Arc::new(Hittable::Sphere(Sphere { 
        center: Point3::new(0., 0.5, -1000.5), 
        radius: 1000., 
        material: Material::Diffuse(Diffuse::new(Color::new_color(0., 1., 0.))), 
    })));

    world.add_light(Arc::new(Hittable::Sphere(Sphere { 
        center: Point3::new(0., 1.3, 0.), 
        radius: 0.35, 
        material: Material::Light(Light::new(Color::new_color(2., 2., 2.))), 
    })));

    world.add(Arc::new(Hittable::Sphere(Sphere {
        center: Point3::new(-0.2, 0.15, 0.1),
        radius: 0.15,
        material: Material::Metal(Metal::new(Color::new_color(0.8, 0.2, 0.1), 0.3)),
    })));

    world.add(Arc::new(Hittable::Sphere(Sphere {
        center: Point3::new(0.2, 0.2, -0.1),
        radius: 0.2,
        material: Material::Dielectric(Dielectric::new_clear(1.5)),
    })));

    world.add(Arc::new(Hittable::Sphere(Sphere {
        center: Point3::new(0.2, 0.2, -0.1),
        radius: -0.17,
        material: Material::Dielectric(Dielectric::new_clear(1.5)),
    })));


    // #########################################################################
    // Random Spheres

    /* world.add(Hittable::Sphere(Sphere {
        center: Point3::new(0., -1000., 0.),
        radius: 1000.,
        material: Material::Diffuse(Diffuse::new(Color::new_color(0.5, 0.5, 0.5))),
    }));

    //let mut rng = StdRng::from_seed([6; 32]);
    let mut rng = StdRng::from_seed([
        6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6,
        6, 6,
    ]);

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat: f64 = rng.gen::<f64>();
            let center: Point3 = Point3 {
                x: a as f64 + 0.9 * rng.gen::<f64>(),

                y: 0.2,
                z: b as f64 + 0.9 * rng.gen::<f64>(),
            };

            if (center - Point3::new(4., 0.2, 0.)).length() > 0.9 {
                let mat: Material;

                if choose_mat < 0.8 {
                    mat = Material::Diffuse(Diffuse::new(Color::new_color(
                        rng.gen::<f64>(),
                        rng.gen::<f64>(),
                        rng.gen::<f64>(),
                    )));
                } else if choose_mat < 0.95 {
                    mat = Material::Metal(Metal::new(
                        Color::new_color(rng.gen::<f64>(), rng.gen::<f64>(), rng.gen::<f64>()),
                        rng.gen::<f64>(),
                    ));
                } else {
                    mat = Material::Dielectric(Dielectric::new_clear(1.5));
                }

                world.add(Hittable::Sphere(Sphere {
                    center,
                    radius: 0.2,
                    material: mat,
                }))
            }
        }
    }

    world.add(Hittable::Sphere(Sphere {
        center: Point3::new(0., 1., 0.),
        radius: 1.,
        material: Material::Dielectric(Dielectric::new_clear(1.5)),
    }));
    world.add(Hittable::Sphere(Sphere {
        center: Point3::new(0., 1., 0.),
        radius: -0.95,
        material: Material::Dielectric(Dielectric::new_clear(1.5)),
    }));
    world.add(Hittable::Sphere(Sphere {
        center: Point3::new(-4., 1., 0.),
        radius: 1.,
        material: Material::Diffuse(Diffuse::new(Color::new_color(0.4, 0.2, 0.1))),
    }));
    world.add(Hittable::Sphere(Sphere {
        center: Point3::new(4., 1., 0.),
        radius: 1.,
        material: Material::Metal(Metal::new(Color::new_color(0.7, 0.6, 0.5), 0.)),
    })); */

    // ########################################################################
    // Three Spheres

    /*    world.add(Box::new(Sphere {
            center: Point3::new(0., 0., -2.),
            radius: 0.499,
            material: Box::new(Metal::new(Color::new_color(0.65, 0.02, 0.08), 0.0)),
        }));
        world.add(Box::new(Sphere {
            center: Point3::new(-1., 0., -2.),
            radius: 0.499,
            material: Box::new(Dielectric::new_clear(1.5)),
        }));
        world.add(Box::new(Sphere {
            center: Point3::new(-1., 0., -2.),
            radius: -0.47,
            material: Box::new(Dielectric::new_clear(1.5)),
        }));
        world.add(Box::new(Sphere {
            center: Point3::new(1., 0., -2.),
            radius: 0.499,
            material: Box::new(Metal::new(Color::new_color(0.2, 0.8, 0.6), 1.)),
        }));
        world.add(Box::new(Sphere {
            center: Point3::new(0., -1000.5, -2.),
            radius: 1000.,
            material: Box::new(Diffuse::new(Color::new_color(0.1, 0.05, 0.5))),
        }));
    */
    world
}
