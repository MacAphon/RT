mod default_material;
pub mod metal;
pub mod lambertian;
pub mod dielectric;

use std::fmt::Debug;
use crate::color::Color;
use crate::hittable::HitRecord;
use crate::material::default_material::DefaultMaterial;
use crate::ray::Ray;


pub trait Material: CloneMaterial + Debug {
    fn scatter(&self, r_in: &Ray, record: &HitRecord) -> (bool, Ray, Color);
}

// A lot of work, just to clone a boxed Trait Object
// Many thanks to user camsteffen on the rust users forum for providing this code
// The thread can be found at
// https://users.rust-lang.org/t/solved-is-it-possible-to-clone-a-boxed-trait-object/1714/6
trait CloneMaterial {
    fn clone_material<'a>(&self) -> Box<dyn Material>;
}

impl<T> CloneMaterial for T
    where
        T: Material + Clone + Default + 'static,
{
    fn clone_material<'a>(&self) -> Box<dyn Material> {
        Box::new(self.clone())
    }
}

impl Clone for Box<dyn Material> {
    fn clone(&self) -> Self {
        self.clone_material()
    }
}

impl Default for Box<dyn Material> {
    fn default() -> Self {
        Box::new(DefaultMaterial::default())
    }
}
