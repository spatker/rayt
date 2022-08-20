use crate::camera::Camera;
use crate::render::{Image, Resolution};
use crate::vec3::{Vec3, Vec3n};
use crate::object::Object;
use crate::object::sphere::{Sphere};
use crate::ray::{Ray, Intersection};
use crate::color::Color;


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

    pub fn trace(&self, ray: &Ray) -> Color {
        self.objs.iter().map(|o|{
            let intersection = o.intersect(ray);
            match intersection {
                Intersection::Miss => Color::new(0.0),
                Intersection::Hit{pos, normal} =>  Color::new(1.0)
            }
        }).last().unwrap()
    }
}