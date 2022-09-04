use crate::camera::Camera;
use crate::image::{Image, Resolution};
use crate::vec3::{Vec3, Vec3n};
use crate::object::{Object, ScatteredRay};
use crate::object::light::{AmbientLight, Emissive};
use crate::object::material::{DiffuseSpecular, Metalic, Refractive};
use crate::object::{sphere::Sphere, plane::Plane};
use crate::ray::{Ray, Intersection};
use crate::color::Color;

use std::path::PathBuf;

const MAX_RECURSION_DEPTH : u8 = 8;


pub struct Scene {
    camera: Camera,
    objs: Vec<Box<dyn Object + Sync>>,
    ambient_light: AmbientLight,
    rays_per_pixel: u32
}

impl Scene {
    pub fn new(rays_per_pixel: u32, hdr_path: &PathBuf) -> Scene {
        let camera = Camera::new(
            90.0, 
            &Vec3{x: 0.0, y: -10.0, z: 3.0},
            &Vec3{x: 0.0, y: -1.5, z: 3.0},
            &Vec3n::new(0.0, 0.0, 1.0),
            0.
        );

        let color_orange = Color::from_hex("#F29F80").unwrap();
        let color_red = Color::from_hex("#D95578").unwrap();
        let color_white = Color::from_hex("#FFFFFF").unwrap();


        let mut objs: Vec<Box<dyn Object + Sync>> = Vec::new();

        let color = color_red * color_orange;
        let material = DiffuseSpecular{diffuse: color, ambient: color * 0.2, specular: color, shineness: 128.0};
        let material = Refractive::new(&Color::new(1.1));
        let sphere = Sphere::new(Vec3{x: 0.0, y: -3.0, z: 5.0}, 3.0, Box::new(material));
        objs.push(Box::new(sphere));
        let color = color_red * 2.0;
        let material = DiffuseSpecular{diffuse: color, ambient: color * 0.2, specular: color, shineness: 128.0};
        //let material = Metalic::silver();
        let sphere = Sphere::new(Vec3{x: 4.0, y: 2.0, z: 3.0}, 3.0, Box::new(material));
        objs.push(Box::new(sphere));
        let color = color_red * color_red;
        let material = DiffuseSpecular{diffuse: color, ambient: color, specular: color, shineness: 128.0};
        let material = Metalic::gold();
        let sphere = Sphere::new(Vec3{x: -4.0, y: 2.0, z: 3.0}, 3.0, Box::new(material));
        objs.push(Box::new(sphere));

        let color = color_white;
        let box_size = 30.0;
        let material = DiffuseSpecular{diffuse: color, ambient: color, specular: color, shineness: 2.0};
        let plane = Plane::new(Vec3{x: 0.0, y: 0.0, z: 0.0}, Vec3n::from(Vec3{x: 0.0, y: 0.0, z: 1.0}), (box_size, box_size), Box::new(material));
        objs.push(Box::new(plane));

        // let material = DiffuseSpecular{diffuse: color, ambient: color, specular: color, shineness: 2.0};
        // let plane = Plane::new(Vec3{x: 0.0, y: box_size/2., z: 0.0}, Vec3n::from(Vec3{x: 0.0, y: -1.0, z: 0.0}), (box_size, box_size), Box::new(material));
        // objs.push(Box::new(plane));
        // let color = color_red;
        // let material = DiffuseSpecular{diffuse: color, ambient: color, specular: color, shineness: 2.0};
        // let plane = Plane::new(Vec3{x: box_size/2., y: 0.0, z: 0.0}, Vec3n::from(Vec3{x: -1.0, y: 0.0, z: 0.0}), (box_size, box_size), Box::new(material));
        // objs.push(Box::new(plane));
        // let color = color_orange;
        // let material = DiffuseSpecular{diffuse: color, ambient: color, specular: color, shineness: 2.0};
        // let plane = Plane::new(Vec3{x: -box_size/2., y: 0.0, z: 0.0}, Vec3n::from(Vec3{x: 1.0, y: 0.0, z: 0.0}), (box_size, box_size), Box::new(material));
        // objs.push(Box::new(plane));

        let material = Emissive{color: Color::new(400.0)};
        let plane = Plane::new(Vec3{x: -15.0, y: -5.0, z: 10.0}, Vec3n::from(Vec3{x: -1.0, y: -1.0, z: -1.0}), (10., 10.), Box::new(material));
        objs.push(Box::new(plane));
        let material = Emissive{color: Color::new(400.0)};
        let plane = Plane::new(Vec3{x: 15.0, y: -5.0, z: 10.0}, Vec3n::from(Vec3{x: 1.0, y: -1.0, z: -1.0}), (10., 10.), Box::new(material));
        objs.push(Box::new(plane));

        let ambient_light = AmbientLight::load(hdr_path).unwrap_or(AmbientLight::new());
        Scene{camera, objs, ambient_light, rays_per_pixel}
    }

    pub fn render(&self, resolution: Resolution) -> Image {
        self.camera.take_picture(resolution, &self, self.rays_per_pixel)
    }

    pub fn first_intersect(&self, ray: &Ray) -> Option<(&Box<dyn Object + Sync>, Intersection)> {
        self.objs.iter().map(|o|{
            match o.intersect(ray) {
                Some(intersection) => Some((o, intersection)),
                None => None
            }
        }).reduce(|a, b| {
            match (&a, &b) {
                (Some((_, Intersection{t: ta,..})), Some((_, Intersection{t: tb,..}))) => {
                    if ta < tb { a } else { b }
                },
                (Some(_), None) => a,
                (None, Some(_)) => b,
                _ => None,
            }
        }).unwrap_or(None)
    }

    pub fn trace(&self, ray: &Ray, depth: u8) -> Color {
        if depth > MAX_RECURSION_DEPTH { return self.ambient_light.get_color(ray) }

        if let Some((object, intersection)) = self.first_intersect(ray) {
            object.scatter(&intersection, ray).iter().map(|scattered_ray| {
                match scattered_ray {
                    ScatteredRay::Scattered{attenuation, ray: next_ray} =>
                        attenuation * self.trace(&next_ray, depth + 1),
                    ScatteredRay::Absorbed{color} => *color
                }
                
            }).reduce(|a, b| { a + b }).unwrap_or_default()
        } else {
            self.ambient_light.get_color(ray)
        }
    }
}