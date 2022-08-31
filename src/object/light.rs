use crate::color::Color;
use crate::image::{Image, Resolution, ImageError};
use crate::object::{Shade, ScatteredRay};
use crate::ray::{Intersection, Ray};
use crate::vec3::{Vec3, Vec3n};

use std::path::PathBuf;

pub struct AmbientLight {
    image: Image
}

impl AmbientLight {

    pub fn new() -> AmbientLight {
        let resolution = Resolution{height: 1024, width: 1024};
        let mut image = Image::new(resolution);
        let color_sky = Color::from_hex("#0396A6").unwrap();
        let color_orange = Color::from_hex("#D14F0F").unwrap();
        image.get_data_mut().iter_mut().enumerate().for_each(|(idx, color)| {
            let (h,w) = resolution.get_height_width(idx);
            let f = (h as f32 / resolution.height as f32);
            *color = (color_sky * (f)  + color_orange * (1.0 - f)) / 1.0
        });

        AmbientLight{image}
    }

    pub fn load(filename: &PathBuf) -> Result<AmbientLight, ImageError> {
        let image = Image::load(filename)?;
        Ok(AmbientLight{image})
    }

    pub fn get_color(&self, ray: &Ray) -> Color {
        let x = ((ray.direction * Vec3n::new(1.0,0.0,0.0)) + 1.0) / 2.0 ;
        //let y = (ray.direction * Vec3n::new(0.0,1.0,0.0));
        let z = ((ray.direction * Vec3n::new(0.0,0.0,-1.0))+ 1.0) / 2.0 ;
        self.image.get_at(x,z)
    }
}

impl Image {
    fn get_at(&self, u: f32, w: f32) -> Color {
        assert!(u >= 0.0 && u <= 1.0);
        assert!(w >= 0.0 && w <= 1.0);
        let width = (u * self.resolution.width as f32) as usize;
        let height = (w * self.resolution.height as f32) as usize;
        self.get_data()[self.resolution.get_index(width, height)]
    }
}

pub struct Emissive {
    pub color: Color
}

impl Shade for Emissive {

    fn scatter(&self, intersection: &Intersection, ray: &Ray) -> Vec<ScatteredRay> {
        let v = ray.origin - intersection.pos;
        let attenuation = (1.0/v.len())*(1.0/v.len());
        let color = attenuation*self.color;
        vec![ScatteredRay::Absorbed{color}]
    }
}
