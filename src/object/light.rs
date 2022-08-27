use crate::vec3::{Vec3, Vec3n};
use crate::color::Color;
use crate::ray::{Ray, Intersection};
use crate::object::RAY_START_EPSILON;

pub struct AmbientLight {
    pub color: Color,
}

pub enum Light {
    Directional {direction: Vec3n, color: Color},
    Point {pos: Vec3, color: Color}
}

impl Light {
    pub fn shadow_ray(&self, intersection: &Intersection) -> Ray {
        match self {
            Light::Directional{direction,..} => Ray{
                origin: intersection.pos + RAY_START_EPSILON * intersection.normal,
                direction: *direction,
                inside: false
            },
            Light::Point{pos,..} => Ray{
                origin: *pos,
                direction: Vec3n::from(intersection.pos - pos),
                inside: false
            },
        }
    }

    pub fn is_in_shadow(&self, intersection: &Intersection, shadow_intersection: &Intersection) -> bool {
        match self {
            Light::Directional{..} => true,
            Light::Point{pos,..} => shadow_intersection.t + RAY_START_EPSILON < (intersection.pos - pos).len()
        }
    }
}