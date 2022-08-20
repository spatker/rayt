pub mod sphere;
use crate::ray::{Ray, Intersection};

pub trait Intersect {
    fn intersect(&self, ray: &Ray) -> Intersection;
}

pub trait Object: Intersect {}

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