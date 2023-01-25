use std::cmp::*;
use crate::util::*;
use std::ops::*;
use float_ord::sort;
use rand::random;

#[derive(Debug, Default, Clone, Copy, PartialEq, PartialOrd)]
pub struct Vec3(pub f64, pub f64, pub f64);

impl Add for Vec3 {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Self(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        *self = Self(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2);
    }
}

impl Sub for Vec3 {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        Self(self.0 - rhs.0, self.1 - rhs.1, self.2 - rhs.2)
    }
}

impl SubAssign for Vec3 {
    fn sub_assign(&mut self, rhs: Self) {
        *self = Self(self.0 - rhs.0, self.1 - rhs.1, self.2 - rhs.2);
    }
}

impl Neg for Vec3 {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Self(-self.0, -self.1, -self.2)
    }
}

impl Mul for Vec3 {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self {
        Self(self.0 * rhs.0, self.1 * rhs.1, self.2 * rhs.2)
    }
}

impl MulAssign for Vec3 {
    fn mul_assign(&mut self, rhs: Self) {
        *self = Self(self.0 * rhs.0, self.1 * rhs.1, self.2 * rhs.2);
    }
}

impl Mul<f64> for Vec3 {
    type Output = Self;
    fn mul(self, rhs: f64) -> Self::Output {
        Self(self.0 * rhs, self.1 * rhs, self.2 * rhs)
    }
}

impl MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, rhs: f64) {
        *self = Self(self.0 * rhs, self.1 * rhs, self.2 * rhs);
    }
}

impl Mul<Vec3> for f64 {
    type Output = Vec3;
    fn mul(self, rhs: Vec3) -> Self::Output {
        Vec3(self * rhs.0, self * rhs.1, self * rhs.2)
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
        self.mul_assign(1. / rhs);
    }
}

impl Vec3 {
    pub fn length_squared(&self) -> f64 {
        self.0.powi(2) + self.1.powi(2) + self.2.powi(2)
    }

    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn dot(self, rhs: Self) -> f64 {
        self.0 * rhs.0 + self.1 * rhs.1 + self.2 * rhs.2
    }

    pub fn cross(self, rhs: Self) -> Vec3 {
        Vec3(self.1 * rhs.2 - self.2 * rhs.1,
             self.2 * rhs.0 - self.0 * rhs.2,
             self.0 * rhs.1 - self.1 * rhs.0)
    }

    pub fn unit_vector(self: Vec3) -> Vec3 {
        self / self.length()
    }

    pub fn is_near_zero(self) -> bool {
        let s = 1e-8;
        (self.0.abs() < s) && (self.1.abs() < s) && (self.2.abs() < s)
    }

    pub fn reflect(self, n: Vec3) -> Vec3 {
        self - 2.*self.dot(n)*n
    }

    pub fn refract(self, n: Vec3, etai_over_etat: f64) -> Vec3 {
        let cos_theta = min_f64((-self).dot(n), 1.);
        let r_out_perp = etai_over_etat * (self + cos_theta*n);
        let r_out_parallel = -((1.-r_out_perp.length_squared()).abs().sqrt()) * n;
        r_out_perp + r_out_parallel
    }

    pub fn new_random() -> Vec3 {
        Vec3(random(), random(), random())
    }

    pub fn new_random_range(min: f64, max: f64) -> Vec3 {
        Vec3(rand_f64(min, max), rand_f64(min, max), rand_f64(min, max))
    }

    pub fn new_random_in_unit_sphere() -> Vec3 {
        let mut p: Vec3;
        loop {
            p = Vec3::new_random_range(-1., 1.);
            if p.length_squared() < 1. { break }
        }
        p
    }

    pub fn new_random_in_hemisphere(normal: Vec3) -> Vec3 {
        let in_unit_sphere = Vec3::new_random_in_unit_sphere();
        if in_unit_sphere.dot(normal) > 0. {
            in_unit_sphere
        }
        else {
            -in_unit_sphere
        }
    }

    pub fn new_random_unit_vector() -> Vec3 {
        Vec3::new_random_in_unit_sphere().unit_vector()
    }

    pub fn x(self) -> f64 {
        self.0
    }

    pub fn y(self) -> f64 {
        self.1
    }

    pub fn z(self) -> f64 {
        self.2
    }
}
