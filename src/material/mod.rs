use crate::hittable::hit_record::HitRecord;
use crate::ray::Ray;
use crate::vec3::Color;

pub mod dielectric;
pub mod diffuse;
pub mod metal;

pub trait Material: CloneMaterial + Send + Sync {
    fn scatter(&self, r_in: &Ray, record: &HitRecord) -> Option<(Ray, Color)>;
}

// A lot of work, just to clone a boxed Trait Object
// Many thanks to user camsteffen on the rust users forum for providing this code
// The thread can be found at
// https://users.rust-lang.org/t/solved-is-it-possible-to-clone-a-boxed-trait-object/1714/6
pub trait CloneMaterial {
    fn clone_material<'a>(&self) -> Box<dyn Material>;
}

impl<T> CloneMaterial for T
    where
        T: Material + Clone + 'static,
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