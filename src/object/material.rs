use crate::color::Color;
use crate::ray::{Intersection, Ray};
use crate::object::light::Light;
use crate::object::Shade;
use crate::vec3::{Vec3, Vec3n};

pub enum Material {
    DiffuseSpecular {
        color_diffuse: Color,
        color_ambient: Color,
        color_specular: Color,
        shineness: f32
    }
}

impl Shade for Material {
    fn get_color(&self, intersection: &Intersection, ray: &Ray, light: &Light) -> Color
    {
        match light {
            Light::Ambient{color: light_color} => {
                match self {
                    Material::DiffuseSpecular{color_ambient: material_color, ..} =>  light_color * material_color,
                }
            }
            Light::Directional{direction, color: light_color} => {
                let diffuse_intensity = f32::max(intersection.normal * direction, 0.0);
                let half = Vec3n::from(direction + Vec3n::from(ray.origin - intersection.pos));
                match self {
                    Material::DiffuseSpecular{color_diffuse, color_specular, shineness, ..} => {
                        let specular_intensity = f32::powf(f32::max(half * intersection.normal, 0.0), *shineness);
                        diffuse_intensity * light_color * color_diffuse +
                        specular_intensity * light_color * color_specular
                    },
                }
            },
            Light::Point{pos, color: light_color} => {
                let v = pos - intersection.pos;
                let attenuation = (1.0/v.len())*(1.0/v.len());
                let diffuse_intensity = f32::max(intersection.normal * Vec3n::from(v), 0.0);
                let half = Vec3n::from(Vec3n::from(pos - intersection.pos) + Vec3n::from(ray.origin - intersection.pos));
                match self {
                    Material::DiffuseSpecular{color_diffuse, color_specular, shineness, ..} => {
                        let specular_intensity = f32::powf(f32::max(half * intersection.normal, 0.0), *shineness);
                        attenuation * diffuse_intensity * light_color * color_diffuse +
                        attenuation * specular_intensity * light_color * color_specular
                    },
                }
            },
        }
    }
}