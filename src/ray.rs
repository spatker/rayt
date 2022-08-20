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
    Hit { pos: Vec3, normal: Vec3n, t: f32},
    Miss,
}

impl Default for Intersection {
    fn default() -> Intersection {
        Intersection::Miss
    }
}

impl Intersection {
    pub fn min(a: &Intersection, b: &Intersection) -> bool {
        match (a,b) {
            (Intersection::Miss, _) => false,
            (_, Intersection::Miss) => true,
            (Intersection::Hit{t:ta, ..}, Intersection::Hit{t:tb, ..}) => { ta < tb },
        }
    }
}