use std::cmp::*;
use crate::util::*;
use std::ops::*;
use rand::random;

#[derive(Debug, Clone, Copy, PartialOrd, PartialEq)]
pub struct Vec3 {
    x: f64,
    y: f64,
    z: f64,
}

impl Add for Vec3 {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl Sub for Vec3 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        self + -rhs
    }
}

impl SubAssign for Vec3 {
    fn sub_assign(&mut self, rhs: Self) {
        *self += -rhs;
    }
}

impl Mul for Vec3 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
        }
    }
}

impl MulAssign for Vec3 {
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs;
    }
}

impl Mul<f64> for Vec3 {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, rhs: f64) {
        *self = *self * rhs;
    }
}

impl Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        rhs * self
    }
}

impl Div<f64> for Vec3 {
    type Output = Self;
    fn div(self, rhs: f64) -> Self::Output {
        self * (1. / rhs)
    }
}

impl DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, rhs: f64) {
        *self = *self / rhs;
    }
}

impl Vec3 {
    pub fn length_squared(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn dot(self, rhs: Self) -> f64 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }

    pub fn cross(self, rhs: Self) -> Vec3 {
        Vec3 {
            x: self.y * rhs.z - self.z * rhs.y,
            y: self.z * rhs.x - self.x * rhs.z,
            z: self.x * rhs.y - self.y * rhs.x,
        }
    }

    pub fn unit_vector(self) -> Vec3 {
        self / self.length()
    }

    pub fn is_near_zero(self) -> bool {
        let s = 1e-8;
        (self.x.abs() < s) && (self.y.abs() < s) && (self.z.abs() < s)
    }

    pub fn reflect(self, n: Vec3) -> Vec3 {
        self - 2.*self.dot(n)*n
    }

    pub fn refract(self, n: Vec3, etai_over_etet: f64) -> Vec3 {
        let cos_theta = min_f64((-self).dot(n), 1.);
        let r_out_perp = etai_over_etet * (self + cos_theta*n);
        let r_out_parallel = -((1.-r_out_perp.length_squared()).abs().sqrt()) * n;
        r_out_perp + r_out_parallel
    }

    pub fn new_random() -> Vec3 {
        Vec3 {
            x: random(),
            y: random(),
            z: random(),
        }
    }

    pub fn new_random_in_range(min: f64, max: f64) -> Vec3 {
        Vec3 {
            x: rand_f64(min, max),
            y: rand_f64(min, max),
            z: rand_f64(min, max),
        }
    }

    /// Generate a random Vector with length < 1
    pub fn new_random_inside_unit_sphere() -> Vec3 {
        let mut p: Vec3;
        loop {
            p = Vec3::new_random_in_range(-1., 1.);
            if p.length_squared() < 1. { break }
        }
        p
    }

    pub fn new_random_inside_hemisphere(normal: Vec3) -> Vec3 {
        let v = Vec3::new_random_inside_unit_sphere();
        if v.dot(normal) > 0. {
            v
        }
        else {
            -v
        }
    }

    /// Generate a random Vector with length = 1
    pub fn new_random_unit_vector() -> Vec3 {
        Vec3::new_random_inside_unit_sphere().unit_vector()
    }

}

pub type Point3 = Vec3;
pub type Color = Vec3;