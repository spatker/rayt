pub mod sphere;
pub mod light;
pub mod material;
use crate::ray::{Ray, Intersection};
use crate::color::Color;
use crate::object::light::Light;

pub trait Intersect {
    fn intersect(&self, ray: &Ray) -> Option<Intersection>;
}

pub trait Shade {
    fn get_color(&self, intersection: &Intersection, lights: &[Light]) -> Color;
}

pub trait Object: Intersect + Shade {}

pub enum Solution {
    TwoRoots{t1: f32, t2: f32},
    OneRoot{t: f32},
    None
}

pub fn solve_quadratic(a: f32, b: f32, c: f32) -> Solution {
    let discriminant = b*b - 4.0*a*c;
    match discriminant {
        d if d < 0.0 => Solution::None,
        d if d == 0.0 => Solution::OneRoot{t: (-b + discriminant.sqrt())/2.0*a},
        _ => Solution::TwoRoots{t1: (-b + discriminant.sqrt())/2.0*a, t2: (-b - discriminant.sqrt())/2.0*a},
    }
}