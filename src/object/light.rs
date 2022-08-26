use crate::vec3::{Vec3, Vec3n};
use crate::color::Color;

pub enum Light {
    Ambient {color: Color},
    Directional {dir: Vec3n, color: Color},
    Point {pos: Vec3, color: Color}
}