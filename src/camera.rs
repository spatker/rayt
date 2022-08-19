use crate::vec3::{Vec3, Vec3n};
use crate::render::Image;
use crate::color::Color;

pub struct Camera {
    pos: Vec3,
    plane_pos: Vec3,
    rigth: Vec3n, 
    up: Vec3n,
}

impl Camera {
    pub fn new(fov: f32, eye: &Vec3, target: &Vec3, plane_up: &Vec3n) -> Self {
        let pos = eye;
        let plane_pos = eye + &(target - eye).norm();
        let fwd = Vec3n::from(plane_pos - *pos);
        let plane_half_size = ((fov*std::f32::consts::PI/180.0)/2.0).tan();
        let rigth = Vec3n::from(Vec3::from(fwd.cross(plane_up)) * plane_half_size);
        let up = Vec3n::from(Vec3::from(rigth.cross(&fwd)) * plane_half_size);
        Camera {pos: *pos, plane_pos, rigth, up}
    }

    pub fn take_picture(&self) -> Image {
        let mut img = Image::new(600, 600);
        for x in 0..img.get_height(){
            for y in 0.. img.get_width(){
                let color = self.capture_pixel(x as f32, y as f32);
                img.set_color(x, y, color);
            }
        }

        img
    }

    fn capture_pixel(&self, x: f32, y: f32) -> Color {
        Color{r: x/600.0, g: y/600.0, b: 0.0}
    }
}