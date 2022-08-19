mod color;
mod vec3;
mod render;
mod camera;
mod ray;

use crate::vec3::{Vec3, Vec3n};

fn main() {
    let camera = camera::Camera::new(
        90.0, 
        &Vec3{x: 0.0, y: -10.0, z: 0.0},
        &Vec3{x: 0.0, y: 0.0, z: 0.0},
        &Vec3n::new(0.0, 0.0, 1.0));
    let img = camera.take_picture(render::Resolution::new(600,600));
    img.save("img.ppm").unwrap();
}
