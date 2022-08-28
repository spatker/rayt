use crate::vec3::{Vec3, Vec3n};
use crate::object::{Object, Intersect, Shade};
use crate::ray::{Ray, Intersection};
use crate::color::Color;

struct Size {
    x: f32,
    y: f32,
}

pub struct Plane{
    pos: Vec3,
    normal: Vec3n,
    size: Size,
    material: Box::<dyn Shade + Sync>,
}

impl Plane {
    pub fn new(pos: Vec3, normal: Vec3n,  size: (f32, f32), material: Box::<dyn Shade+ Sync>) -> Plane {
        Plane{pos, normal, size: Size{x: size.0, y: size.1}, material }
    }
}

impl Intersect for Plane {
    fn intersect(&self, ray: &Ray) -> Option<Intersection> {
        let t = ((self.pos - ray.origin) * self.normal) / (ray.direction * self.normal);
        let pos = ray.at(t);
        if ( t > 0. && !f32::is_nan(t) && 
            ((self.pos.x - pos.x).abs() < self.size.x/2. &&
            (self.pos.y - pos.y).abs() < self.size.y/2. )) {
            Some(Intersection{normal: self.normal, t, pos})
        }
        else {
            None
        }
    }
}

impl Shade for Plane {
    fn scatter(&self, intersection: &Intersection, ray: &Ray) -> Vec<(Color, Ray)> {
        self.material.scatter(intersection, ray)
    }
}

impl Object for Plane {}
