use crate::vec3::{Vec3, Vec3n};
use crate::object::{Object, Intersect, Shade, Solution, solve_quadratic};
use crate::ray::{Ray, Intersection};
use crate::object::light::{Light, AmbientLight};
use crate::color::Color;

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
    fn get_color(&self, intersection: &Intersection, ray: &Ray, light: &Light) -> Color {
        self.material.get_color(intersection, ray, light)
    }

    fn get_color_ambient(&self, intersection: &Intersection, ray: &Ray, light: &AmbientLight) -> Color {
        self.material.get_color_ambient(intersection, ray, light)
    }

    fn next_ray(&self, intersection: &Intersection, ray: &Ray) -> Option<(Color, Ray)>{
        self.material.next_ray(intersection, ray)
    }
}

impl Object for Sphere {}