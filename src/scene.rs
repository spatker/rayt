use crate::camera::Camera;
use crate::render::{Image, Resolution};
use crate::vec3::{Vec3, Vec3n};
use crate::object::Object;
use crate::object::sphere::{Sphere};
use crate::ray::{Ray, Intersection};
use crate::color::Color;

use rayon::prelude::*;


pub struct Scene {
    camera: Camera,
    objs: Vec<Box<dyn Object + Sync>>,
}

impl Scene {
    pub fn new() -> Scene {
        let camera = Camera::new(
            90.0, 
            &Vec3{x: 0.0, y: -10.0, z: 0.0},
            &Vec3{x: 0.0, y: 0.0, z: 0.0},
            &Vec3n::new(0.0, 0.0, 1.0)
        );

        let sphere = Sphere::new(Vec3{x: 0.0, y: 0.0, z: 0.0}, 1.0);
        let mut objs: Vec<Box<dyn Object + Sync>> = Vec::new();
        objs.push(Box::new(sphere));
        Scene{camera, objs}
    }

    pub fn render(&self, resolution: Resolution) -> Image {
        self.camera.take_picture(resolution, &self)
    }

    pub fn first_intersect(&self, ray: &Ray) -> Intersection {
        self.objs.par_iter().map(|o|{
            o.intersect(ray)
        }).reduce(|| Intersection::default(), |a, b| {
            match Intersection::min(&a,&b) {
                true => a,
                false => b
            }
        })
    }

    pub fn trace(&self, ray: &Ray) -> Color {
        let intersection = self.first_intersect(ray);
        match intersection {
            Intersection::Miss => Color::new(0.0),
            Intersection::Hit{pos, normal, t} => Color::new(1.0)
        }
    }
}