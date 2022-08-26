use crate::vec3::{Vec3, Vec3n};
use crate::color::Color;
use crate::ray::{Ray, Intersection};

pub struct AmbientLight {
    pub color: Color,
}

pub enum Light {
    Directional {direction: Vec3n, color: Color},
    Point {pos: Vec3, color: Color}
}

const SHADOW_EPSILON: f32 = 1e-3;

impl Light {
    pub fn shadow_ray(&self, intersection: &Intersection) -> Ray {
        match self {
            Light::Directional{direction,..} => Ray{
                origin: intersection.pos + SHADOW_EPSILON * intersection.normal,
                direction: *direction
            },
            Light::Point{pos,..} => Ray{
                origin: *pos,
                direction: Vec3n::from(intersection.pos - pos)
            },
        }
    }

    pub fn is_in_shadow(&self, intersection: &Intersection, shadow_intersection: &Intersection) -> bool {
        match self {
            Light::Directional{..} => true,
            Light::Point{pos,..} => shadow_intersection.t + SHADOW_EPSILON < (intersection.pos - pos).len()
        }
    }
}