use crate::camera::Camera;
use crate::render::{Image, Resolution};
use crate::vec3::{Vec3, Vec3n};


pub struct Scene {
    camera: Camera,
}

impl Scene {
    pub fn new() -> Scene {
        let camera = Camera::new(
            90.0, 
            &Vec3{x: 0.0, y: -10.0, z: 0.0},
            &Vec3{x: 0.0, y: 0.0, z: 0.0},
            &Vec3n::new(0.0, 0.0, 1.0));
        Scene{camera}
    }

    pub fn render(&self, resolution: Resolution) -> Image {
        self.camera.take_picture(resolution)
    }
}