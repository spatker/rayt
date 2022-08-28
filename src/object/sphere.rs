use crate::vec3::{Vec3, Vec3n};
use crate::object::{Object, Intersect, Shade, Solution, ScatteredRay, solve_quadratic};
use crate::ray::{Ray, Intersection};

pub struct Sphere{
    pos: Vec3,
    r: f32,
    material: Box::<dyn Shade + Sync>,
}

impl Sphere {
    pub fn new(pos: Vec3, r: f32, material: Box::<dyn Shade + Sync>) -> Sphere {
        Sphere{pos, r, material}
    }
}

impl Intersect for Sphere {
    fn intersect(&self, ray: &Ray) -> Option<Intersection> {
        let a = ray.direction * ray.direction;
        let b = 2.0*((ray.origin - self.pos)*ray.direction);
        let c = (ray.origin - self.pos)*(ray.origin - self.pos) - self.r*self.r;
        let solution = solve_quadratic(a, b, c);
        match solution {
            Solution::None => None,
            Solution::OneRoot{..} => None,
            Solution::TwoRoots{t1, t2} => {
                assert!(t1 >= t2);
                if t1 <= 0. {return None}
                let t = if t2 > 0. { t2 } else { t1 };
                let normal = Vec3n::from((ray.at(t) - self.pos)/self.r);
                let pos = ray.at(t);
                Some(Intersection{normal, pos, t})
            },
        }
    }
}

impl Shade for Sphere {
    fn scatter(&self, intersection: &Intersection, ray: &Ray) -> Vec<ScatteredRay>{
        self.material.scatter(intersection, ray)
    }
}

impl Object for Sphere {}