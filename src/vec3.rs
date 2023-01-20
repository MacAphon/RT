use std::ops::*;

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Vec3(pub f64, pub f64, pub f64);

impl Add for Vec3 {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Self {
            0: self.0 + rhs.0,
            1: self.1 + rhs.1,
            2: self.2 + rhs.2,
        }
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        *self = Self {
            0: self.0 + rhs.0,
            1: self.1 + rhs.1,
            2: self.2 + rhs.2,
        };
    }
}

impl Sub for Vec3 {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        Self {
            0: self.0 - rhs.0,
            1: self.1 - rhs.1,
            2: self.2 - rhs.2,
        }
    }
}

impl SubAssign for Vec3 {
    fn sub_assign(&mut self, rhs: Self) {
        *self = Self {
            0: self.0 - rhs.0,
            1: self.1 - rhs.1,
            2: self.2 - rhs.2,
        };
    }
}

impl Neg for Vec3 {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Self {
            0: -self.0,
            1: -self.1,
            2: -self.2,
        }
    }
}

impl Mul for Vec3 {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self {
        Self {
            0: self.0 * rhs.0,
            1: self.1 * rhs.1,
            2: self.2 * rhs.2,
        }
    }
}

impl MulAssign for Vec3 {
    fn mul_assign(&mut self, rhs: Self) {
        *self = Self {
            0: self.0 * rhs.0,
            1: self.1 * rhs.1,
            2: self.2 * rhs.2,
        };
    }
}

impl Mul<f64> for Vec3 {
    type Output = Self;
    fn mul(self, rhs: f64) -> Self::Output {
        Self {
            0: self.0 * rhs,
            1: self.1 * rhs,
            2: self.2 * rhs,
        }
    }
}

impl MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, rhs: f64) {
        *self = Self {
            0: self.0 * rhs,
            1: self.1 * rhs,
            2: self.2 * rhs,
        };
    }
}

impl Mul<Vec3> for f64 {
    type Output = Vec3;
    fn mul(self, rhs: Vec3) -> Self::Output {
        Vec3 {
            0: self * rhs.0,
            1: self * rhs.1,
            2: self * rhs.2,
        }
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
    pub fn length_squared(self: Self) -> f64 {
        self.0.powi(2) + self.1.powi(2) + self.2.powi(2)
    }

    pub fn length(self: Self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn dot(self: Self, rhs: Self) -> f64 {
        self.0 * rhs.0 + self.1 * rhs.1 + self.2 * rhs.2
    }

    pub fn cross(self: Self, rhs: Self) -> Vec3 {
        Vec3 {
            0: self.1 * rhs.2 - self.2 * rhs.1,
            1: self.2 * rhs.0 - self.0 * rhs.2,
            2: self.0 * rhs.1 - self.1 * rhs.0,
        }
    }

    pub fn unit_vector(self: Vec3) -> Vec3 {
        self / self.length()
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
