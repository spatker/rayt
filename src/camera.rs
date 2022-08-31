use crate::vec3::{Vec3, Vec3n};
use crate::image::{Image, Resolution};
use crate::color::Color;
use crate::scene::Scene;
use crate::ray::Ray;

use indicatif::{ProgressStyle, ParallelProgressIterator};
use rayon::prelude::*;
use rand::prelude::*;

#[derive(Debug)]
pub struct Camera {
    pos: Vec3,
    plane_pos: Vec3,
    rigth: Vec3n, 
    up: Vec3n,
    plane_half_size: f32,
    aperture: f32,
}

impl Camera {
    pub fn new(fov: f32, eye: &Vec3, target: &Vec3, plane_up: &Vec3n, aperture: f32) -> Self {
        let pos = eye;
        let focus_distance = (target - eye).len();
        let plane_pos = eye + (target - eye);
        let forward = (plane_pos - pos).norm();
        let plane_half_size = (f32::to_radians(fov) / 2.0).tan() * focus_distance;
        let rigth = forward.cross(plane_up);
        let up = rigth.cross(&forward);
        Camera {pos: *pos, plane_pos, rigth, up, plane_half_size, aperture}
    }

    pub fn take_picture(&self, resolution: Resolution, scene: &Scene, rays: u32) -> Image {
        let mut img = Image::new(resolution);
        let style = ProgressStyle::with_template("[{elapsed}] {bar:40.cyan/blue} {pos:>7}/{len:7} {eta}").unwrap();

        img.get_data_mut().par_iter_mut().progress_with_style(style).enumerate().for_each(|(idx, color)| {
            let (h,w) = resolution.get_height_width(idx);
            *color = self.capture_pixel(h as f32, w as f32, &resolution, scene, rays);
        });

        img
    }

    fn capture_pixel(&self, h: f32, w: f32, resolution: &Resolution,  scene: &Scene, rays: u32) -> Color {

        let mut rng = thread_rng();


        (0..rays).map(|_| {
            let rand_x = rng.gen_range(0.0..1.0);
            let rand_y = rng.gen_range(0.0..1.0);
            let pos_on_plane = Vec3{
                x: (w + rand_x - (resolution.width/2) as f32) / (resolution.width/2) as f32,
                y: (h + rand_y - (resolution.height/2) as f32) / (resolution.height/2) as f32,
                z: 0.0
            };

            let blur_offset = Vec3n::random_disc() * (self.aperture / 2.0);
            let blur_offset = self.rigth * blur_offset.x + self.up * blur_offset.y;

            let plane_intersection =
                self.plane_pos +
                self.plane_half_size * pos_on_plane.x * self.rigth +
                self.plane_half_size * pos_on_plane.y * self.up;

            let ray = Ray {
                origin: self.pos + blur_offset,
                direction: Vec3n::from(plane_intersection  - self.pos - blur_offset),
                inside: false
            };
            scene.trace(&ray, 0).fix()
        }).fold(Color::default(), |a,b| { a + b }) / rays as f32
    }
}