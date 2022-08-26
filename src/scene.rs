use crate::camera::Camera;
use crate::render::{Image, Resolution};
use crate::vec3::{Vec3, Vec3n};
use crate::object::Object;
use crate::object::light::Light;
use crate::object::material::Material;
use crate::object::{sphere::Sphere, plane::Plane};
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
            &Vec3{x: 0.0, y: -10.0, z: 3.0},
            &Vec3{x: 0.0, y: -3.0, z: 4.0},
            &Vec3n::new(0.0, 0.0, 1.0)
        );

        let color_sky = Color::from_hex("#0396A6").unwrap();
        let color_orange = Color::from_hex("#F29F80").unwrap();
        let color_red = Color::from_hex("#D95578").unwrap();


        let mut objs: Vec<Box<dyn Object + Sync>> = Vec::new();
        let sphere = Sphere::new(Vec3{x: 0.0, y: -3.0, z: 5.0}, 3.0, Material::Diffuse{color_diffuse: color_red, color_ambient: color_red * 0.4});
        objs.push(Box::new(sphere));
        let sphere = Sphere::new(Vec3{x: 4.0, y: 0.0, z: 3.0}, 3.0, Material::Diffuse{color_diffuse: color_red, color_ambient: color_red * 0.4});
        objs.push(Box::new(sphere));
        let sphere = Sphere::new(Vec3{x: -4.0, y: 0.0, z: 3.0}, 3.0, Material::Diffuse{color_diffuse: color_red, color_ambient: color_red * 0.4});
        objs.push(Box::new(sphere));
        let plane = Plane::new(Vec3{x: 0.0, y: 0.0, z: 0.0}, Vec3n::from(Vec3{x: 0.0, y: 0.0, z: 1.0}), (30., 30.), Material::Diffuse{color_diffuse: color_orange, color_ambient: color_orange * 0.4});
        objs.push(Box::new(plane));

        let lights = vec![
            Light::Directional{direction: Vec3n::new(1.0, 1.0, 1.0), color: Color::new(0.2)},
            Light::Point{pos: Vec3{x: 3.0, y: -7.0, z: 8.0}, color: color_orange*20.0},
            Light::Point{pos: Vec3{x: -10.0, y: -7.0, z: 8.0}, color: color_red*20.0},
            Light::Ambient{color: color_sky}
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

    pub fn in_shadow(&self, intersection: &Intersection, light: &Light) -> bool {
        if let Some(shadow_ray) = light.shadow_ray(&intersection) {
            if let Some((_, shadow_intersection)) = self.first_intersect(&shadow_ray) {
                light.is_in_shadow(intersection, &shadow_intersection)
            } else {
                true
            }
        } else {
            false
        }
    }

    pub fn trace(&self, ray: &Ray) -> Color {
        if let Some((object, intersection)) = self.first_intersect(ray){
            self.lights.par_iter().map(|light|{
                if self.in_shadow(&intersection, light) {
                    Color::default()
                } else {
                    object.get_color(&intersection, light)
                }
            }).reduce(|| Color::default(), |a, b| {
                a + b
            })
        } else {
            self.lights.par_iter().map(|light|{
                match light {
                    Light::Ambient{color: light_color} => *light_color,
                    _ => Color::default()
                }
            }).reduce(|| Color::default(), |a, b| {
                a + b
            })
        }
    }
}