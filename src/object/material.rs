use crate::color::Color;
use crate::ray::Intersection;
use crate::object::light::Light;
use crate::object::Shade;
use crate::vec3::{Vec3, Vec3n};

pub enum Material {
    Diffuse {color_diffuse: Color, color_ambient: Color}
}

impl Shade for Material {
    fn get_color(&self, intersection: &Intersection, light: &Light) -> Color
    {
        match light {
            Light::Ambient{color: light_color} => {
                match self {
                    Material::Diffuse{color_ambient: material_color, ..} =>  light_color * material_color,
                }
            }
            Light::Directional{direction, color: light_color} => {
                let intensity = f32::max(intersection.normal * direction, 0.0);
                match self {
                    Material::Diffuse{color_diffuse: material_color, ..} => intensity * light_color * material_color,
                }
            },
            Light::Point{pos, color: light_color} => {
                let v = pos - intersection.pos;
                let attenuation = (1.0/v.len())*(1.0/v.len());
                let intensity = f32::max(intersection.normal * Vec3n::from(v), 0.0);
                match self {
                    Material::Diffuse{color_diffuse: material_color, ..} =>
                        attenuation * intensity * light_color * material_color,
                }
            },
        }
    }
}