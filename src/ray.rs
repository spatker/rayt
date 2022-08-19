use crate::vec3::{Vec3, Vec3n};

struct Ray {
    origin: Vec3,
    direction: Vec3n
}

enum Intersection {
    Hit { pos: Vec3, normal: Vec3n},
    Miss,
}