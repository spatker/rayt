use crate::vec3::{Vec3, Vec3n};
use crate::color::Color;
use crate::ray::{Ray, Intersection};

pub enum Light {
    Ambient {color: Color},
    Directional {direction: Vec3n, color: Color},
    Point {pos: Vec3, color: Color}
}

const SHADOW_EPSILON: f32 = 1e-3;

impl Light {
    pub fn shadow_ray(&self, intersection: &Intersection) -> Option<Ray> {
        match self {
            Light::Ambient{..} => None,
            Light::Directional{direction,..} =>
            Some(Ray{
                origin: intersection.pos + SHADOW_EPSILON * intersection.normal,
                direction: *direction
            }),
            Light::Point{pos,..} =>
            Some(Ray{
                origin: *pos,
                direction: Vec3n::from(intersection.pos - pos)
            }),
        }
    }

    pub fn is_in_shadow(&self, intersection: &Intersection, shadow_intersection: &Intersection) -> bool {
        match self {
            Light::Ambient{..} => false,
            Light::Directional{..} => true,
            Light::Point{pos,..} => {
                shadow_intersection.t + 1e-3 < (intersection.pos - pos).len()
            }
        }
    }
}