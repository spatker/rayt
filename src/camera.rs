use crate::vec3::{Vec3, Vec3n};
use crate::render::{Image, Resolution};
use crate::color::Color;
use crate::scene::Scene;
use crate::ray::Ray;

use rayon::prelude::*;

#[derive(Debug)]
pub struct Camera {
    pos: Vec3,
    plane_pos: Vec3,
    rigth: Vec3n, 
    up: Vec3n,
    plane_half_size: f32,
}

impl Camera {
    pub fn new(fov: f32, eye: &Vec3, target: &Vec3, plane_up: &Vec3n) -> Self {
        let pos = eye;
        let plane_pos = eye + (target - eye).norm();
        let fwd = Vec3n::from(plane_pos - pos);
        let plane_half_size = ((fov*std::f32::consts::PI/180.0)/2.0).tan();
        let rigth = Vec3n::from(Vec3::from(fwd.cross(plane_up)));
        let up = Vec3n::from(Vec3::from(rigth.cross(&fwd)));
        Camera {pos: *pos, plane_pos, rigth, up, plane_half_size}
    }

    pub fn take_picture(&self, resolution: Resolution, scene: &Scene) -> Image {
        let mut img = Image::new(resolution);
        img.get_data().par_iter_mut().enumerate().for_each(|(idx, color)| {
            let (h,w) = resolution.get_height_width(idx);
            *color = self.capture_pixel(h as f32, w as f32, &resolution, scene);
        });

        img
    }

    fn capture_pixel(&self, h: f32, w: f32, resolution: &Resolution,  scene: &Scene) -> Color {
        let pos_on_plane = Vec3{
            x: (w + 0.5 - (resolution.width/2) as f32) / (resolution.width/2) as f32,
            y: (h + 0.5 - (resolution.height/2) as f32) / (resolution.height/2) as f32,
            z: 0.0
        };
      
        let plane_intersection =
            self.plane_pos +
            self.plane_half_size * pos_on_plane.x * self.rigth +
            self.plane_half_size * pos_on_plane.y * self.up;

        let ray = Ray {
            origin: self.pos,
            direction: Vec3n::from(plane_intersection - self.pos),
        };
        scene.trace(&ray, 0)
    }
}