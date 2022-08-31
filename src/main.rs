mod color;
mod vec3;
mod render;
mod camera;
mod ray;
mod scene;
mod object;
mod image;

#[macro_use] extern crate impl_ops;

use scene::Scene;

use std::path::PathBuf;
use structopt::StructOpt;
#[derive(Debug, StructOpt)]
struct Opt {
    #[structopt(long, parse(from_os_str), default_value = "")]
    hdr_path: PathBuf,

    #[structopt(short, long, parse(from_os_str), default_value = "img.ppm")]
    out_path: PathBuf,

    #[structopt(short, long, default_value = "600")]
    resolution: usize,

    #[structopt(short, long, default_value = "128")]
    samples: u32,
}

fn main() {
    let opt = Opt::from_args();
    let scene = Scene::new(opt.samples, &opt.hdr_path);
    scene.render(image::Resolution::new(opt.resolution, opt.resolution)).save(&opt.out_path).unwrap();
}
