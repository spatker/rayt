use std::ops::{Add, Sub, Neg, Div, Mul};

#[derive(Debug, Clone, Copy)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32
}

#[derive(Debug, Clone, Copy)]
pub struct Vec3n {
    x: f32,
    y: f32,
    z: f32
}

impl Vec3 {
    pub fn len(&self) -> f32 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    pub fn norm(&self) -> Self {
        self / self.len()
    }

    pub fn cross(&self, rhs: &Self) -> Self {
        Vec3 {
            x: self.y*rhs.z - self.z*rhs.y,
            y: self.z*rhs.x - self.x*rhs.z,
            z: self.x*rhs.y - self.y*rhs.x
        }
    }
}

impl Vec3n {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Vec3n::from(&Vec3{x, y, z})
    }

    pub fn cross(&self, rhs: &Self) -> Self {
        Vec3n::from(
            &Vec3::from(self).cross(&Vec3::from(rhs))
        )
    }
}

impl From<f32> for Vec3 {
    fn from(a: f32) -> Self {
        Vec3{x: a, y: a, z:a}
    }
}

impl From<&Vec3n> for Vec3 {
    fn from(v: &Vec3n) -> Self {
        Vec3{x: v.x, y: v.y, z: v.z}
    }
}

impl From<Vec3n> for Vec3 {
    fn from(v: Vec3n) -> Self {
        Vec3{x: v.x, y: v.y, z: v.z}
    }
}

impl From<&Vec3> for Vec3n {
    fn from(v: &Vec3) -> Self {
        let n = v.norm();
        Vec3n {x: n.x, y: n.y, z: n.z}
    }
}

impl From<Vec3> for Vec3n {
    fn from(v: Vec3) -> Self {
        let n = v.norm();
        Vec3n {x: n.x, y: n.y, z: n.z}
    }
}

impl Add for &Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Self) -> Vec3 {
        Vec3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl Add for Vec3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Vec3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl Mul<f32> for Vec3 {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self {
        Vec3 {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl Mul<Vec3> for f32 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Vec3 {
        Vec3 {
            x: rhs.x * self,
            y: rhs.y * self,
            z: rhs.z * self,
        }
    }
}

impl Mul<Vec3n> for f32 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3n) -> Vec3 {
        Vec3 {
            x: rhs.x * self,
            y: rhs.y * self,
            z: rhs.z * self,
        }
    }
}

impl Mul for Vec3 {
    type Output = f32;

    fn mul(self, rhs: Self) -> f32 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z 
    }
}

impl Mul for Vec3n {
    type Output = f32;

    fn mul(self, rhs: Self) -> f32 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z 
    }
}

impl Mul<Vec3n> for &Vec3n {
    type Output = f32;

    fn mul(self, rhs: Vec3n) -> f32 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z 
    }
}

impl Mul for &Vec3n {
    type Output = f32;

    fn mul(self, rhs: &Vec3n) -> f32 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z 
    }
}

impl Mul<&Vec3n> for Vec3n {
    type Output = f32;

    fn mul(self, rhs: &Vec3n) -> f32 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z 
    }
}

impl Mul<Vec3n> for Vec3 {
    type Output = f32;

    fn mul(self, rhs: Vec3n) -> f32 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z 
    }
}

impl Sub for &Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Self) -> Vec3 {
        Vec3 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl Sub for Vec3 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Vec3 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self {
        Vec3 {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl Div<f32> for &Vec3 {
    type Output = Vec3;

    fn div(self, rhs: f32) -> Vec3 {
        Vec3 {
            x: self.x/rhs,
            y: self.y/rhs,
            z: self.z/rhs,
        }
    }
}

impl Div<f32> for Vec3 {
    type Output = Self;

    fn div(self, rhs: f32) -> Self {
        Vec3 {
            x: self.x/rhs,
            y: self.y/rhs,
            z: self.z/rhs,
        }
    }
}