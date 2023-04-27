use crate::hittable::hit_record::HitRecord;
use crate::hittable::sphere::Sphere;
use crate::hittable::Hittable;
use crate::material::dielectric::Dielectric;
use crate::material::diffuse::Diffuse;
use crate::material::metal::Metal;
use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::{Color, Point3};
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use crate::hittable;

#[derive(Default)]
pub struct HittableList {
    pub objects: Vec<Hittable>,
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

    pub fn add(&mut self, rhs: Hittable) {
        self.objects.push(rhs);
    }

    pub fn hit_anything<'a>(&'a self, r: &'a Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut closest_so_far: f64 = t_max;
        let mut rec: Option<HitRecord> = None;

        for element in &self.objects {
            if let Some(temp_r) = hittable::hit(element, r, t_min, closest_so_far) {
                closest_so_far = temp_r.t;
                rec = Some(temp_r);
            }
        }
        rec
    }
}

pub fn generate_world() -> HittableList {
    let mut world: HittableList = HittableList::new();

    world.add(Hittable::Sphere(Sphere{
        center: Point3::new(0., -1000., 0.),
        radius: 1000.,
        material: Material::Diffuse(Diffuse::new(Color::new_color(0.5, 0.5, 0.5))),
    }));

    let mut rng = StdRng::from_seed([6; 32]);

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
    }));

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
