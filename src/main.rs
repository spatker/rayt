mod color;
mod vec3;
mod render;
mod camera;
mod ray;
mod scene;
mod object;

#[macro_use] extern crate impl_ops;

use scene::Scene;

fn main() {
    let scene = Scene::new();
    scene.render(render::Resolution::new(600,600)).save("img.ppm").unwrap();
}
