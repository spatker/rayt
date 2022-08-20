use crate::vec3::{Vec3, Vec3n};
use crate::object::{Object, Intersect, Solution, solve_quadratic};
use crate::ray::{Ray, Intersection};

pub struct Sphere{
    pos: Vec3,
    r: f32,
}

impl Sphere {
    pub fn new(pos: Vec3, r: f32) -> Sphere {
        Sphere{pos, r}
    }
}

impl Intersect for Sphere {
    fn intersect(&self, ray: &Ray) -> Intersection {
        let a = ray.direction * ray.direction;
        let b = 2.0*((ray.origin - self.pos)*ray.direction);
        let c = (ray.origin - self.pos)*(ray.origin - self.pos) - self.r*self.r;
        let solution = solve_quadratic(a, b, c);
        match solution {
            Solution::None => Intersection::Miss,
            Solution::OneRoot{t:_} => Intersection::Miss,
            Solution::TwoRoots{t1, t2} => {
                let t = f32::min(t1,t2);
                let normal = Vec3n::from((ray.at(t) - self.pos)/self.r);
                let pos = ray.at(t);
                Intersection::Hit{normal, pos, t}
            },
        }
    }
}

impl Object for Sphere {}