use crate::camera::Camera;
use crate::render::{Image, Resolution};
use crate::vec3::{Vec3, Vec3n};
use crate::object::Object;
use crate::object::light::Light;
use crate::object::material::Material;
use crate::object::sphere::{Sphere};
use crate::ray::{Ray, Intersection};
use crate::color::Color;

use rayon::prelude::*;


pub struct Scene {
    camera: Camera,
    objs: Vec<Box<dyn Object + Sync>>,
    lights: Vec<Light>,
}

impl Scene {
    pub fn new() -> Scene {
        let camera = Camera::new(
            90.0, 
            &Vec3{x: 0.0, y: -10.0, z: 0.0},
            &Vec3{x: 0.0, y: 0.0, z: 0.0},
            &Vec3n::new(0.0, 0.0, 1.0)
        );

        let sphere = Sphere::new(Vec3{x: 0.0, y: 0.0, z: 0.0}, 3.0, Material::Diffuse{color: Color::new(0.5)});
        let mut objs: Vec<Box<dyn Object + Sync>> = Vec::new();
        objs.push(Box::new(sphere));
        let lights = vec![
            Light::Directional{dir: Vec3n::new(0.0, 0.0, -1.0), color: Color::new(0.5)}
        ];
        Scene{camera, objs, lights}
    }

    pub fn render(&self, resolution: Resolution) -> Image {
        self.camera.take_picture(resolution, &self)
    }

    pub fn first_intersect(&self, ray: &Ray) -> Option<(&Box<dyn Object + Sync>, Intersection)> {
        self.objs.par_iter().map(|o|{
            match o.intersect(ray) {
                Some(intersection) => Some((o, intersection)),
                None => None
            }
        }).reduce(|| None, |a, b| {
            match (&a, &b) {
                (Some((_, Intersection{t: ta,..})), Some((_, Intersection{t: tb,..}))) => {
                    if ta < tb { a } else { b }
                },
                (Some(_), None) => a,
                (None, Some(_)) => b,
                _ => None,
            }
        })
    }

    pub fn trace(&self, ray: &Ray) -> Color {
        if let Some((object, intersection)) = self.first_intersect(ray){
            object.get_color(&intersection, &self.lights)
        } else {
            Color::new(0.0)
        }
    }
}