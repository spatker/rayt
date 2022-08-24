use crate::color::Color;
use crate::ray::Intersection;
use crate::object::light::Light;
use crate::object::Shade;

use rayon::prelude::*;

pub enum Material {
    Diffuse {color: Color}
}

impl Shade for Material {
    fn get_color(&self, intersection: &Intersection, lights: &[Light]) -> Color
    {
        lights.par_iter().map(|light|{
            match light {
                Light::Ambient{..} => todo!("Ambient ligth"),
                Light::Directional{dir, color: light_color} => {
                    let intensity = f32::max(intersection.normal * dir, 0.0);
                    match self {
                        Material::Diffuse{color: material_color} => intensity * light_color * material_color,
                    }
                },
                Light::Point{pos, dir, color} => todo!("Point ligth"),
            }
        }).reduce(|| Color::default(), |a, b| {
            a + b
        })
    }
}