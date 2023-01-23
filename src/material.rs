use crate::color::Color;
use crate::hittable::HitRecord;
use crate::ray::Ray;

pub trait Material: CloneMaterial {
    fn scatter(&self, attenuation: Color, scattered: &Ray, record: HitRecord) -> bool;
}

// A lot of work, just to clone a boxed Trait Object
// Many thanks to camsteffen on the rust users forum for providing this code
// The thread can be found at
// https://users.rust-lang.org/t/solved-is-it-possible-to-clone-a-boxed-trait-object/1714/6
trait CloneMaterial {
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