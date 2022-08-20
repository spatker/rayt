use crate::vec3::{Vec3, Vec3n};

pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3n
}

impl Ray {
    pub fn at(&self, t: f32) -> Vec3 {
        self.origin + Vec3::from(self.direction)*t
    }
}

pub enum Intersection {
    Hit { pos: Vec3, normal: Vec3n},
    Miss,
}