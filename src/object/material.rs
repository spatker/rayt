use crate::color::Color;
use crate::ray::{Intersection, Ray};
use crate::object::light::{Light, AmbientLight};
use crate::object::Shade;
use crate::vec3::{Vec3, Vec3n};
use crate::object::RAY_START_EPSILON;

pub struct DiffuseSpecular {
    pub diffuse: Color,
    pub ambient: Color,
    pub specular: Color,
    pub shineness: f32
}

impl Shade for DiffuseSpecular {
    fn get_color(&self, intersection: &Intersection, ray: &Ray, light: &Light) -> Color
    {
        match light {
            Light::Directional{direction, color: light_color} => {
                let diffuse_intensity = f32::max(intersection.normal * direction, 0.0);
                let half = Vec3n::from(direction + Vec3n::from(ray.origin - intersection.pos));
                let specular_intensity = f32::powf(f32::max(half * intersection.normal, 0.0), self.shineness);
                diffuse_intensity * light_color * self.diffuse +
                specular_intensity * light_color * self.specular
            }
            Light::Point{pos, color: light_color} => {
                let v = pos - intersection.pos;
                let attenuation = (1.0/v.len())*(1.0/v.len());
                let diffuse_intensity = f32::max(intersection.normal * Vec3n::from(v), 0.0);
                let half = Vec3n::from(Vec3n::from(pos - intersection.pos) + Vec3n::from(ray.origin - intersection.pos));
                let specular_intensity = f32::powf(f32::max(half * intersection.normal, 0.0), self.shineness);
                attenuation * diffuse_intensity * light_color * self.diffuse +
                attenuation * specular_intensity * light_color * self.specular
            },
        }
    }

    fn get_color_ambient(&self, intersection: &Intersection, ray: &Ray, light: &AmbientLight) -> Color {
        light.color * self.ambient
    }

    fn next_ray(&self, intersection: &Intersection, ray: &Ray) -> Option<(Color, Ray)> { None }
}

pub struct Metalic {
    f0: Color
}

impl Metalic {
    pub fn new(n: &Color, k: &Color) -> Metalic {
        Metalic {
            f0: (((n-1.0)*(n-1.0) + k*k) /
                ((n+1.0)*(n+1.0) + k*k))
        }
    }

    pub fn fresnel(&self, cos_theta: f32) -> Color {
        self.f0 + (1.0-self.f0) * f32::powf(1.0-cos_theta, 5.0)
    }

    pub fn gold() -> Metalic {
        Metalic::new(
            &Color{ r: 0.17, g:0.35, b: 1.5},
            &Color{ r: 3.1, g: 2.7, b: 1.9},
        )
    }

    pub fn silver() -> Metalic {
        Metalic::new(
            &Color{ r: 0.14, g: 0.16, b: 0.13},
            &Color{ r: 4.1, g: 2.3, b: 3.1},
        )
    }
}

impl Shade for Metalic {
    fn get_color(&self, intersection: &Intersection, ray: &Ray, light: &Light) -> Color
    {
        Color::default()
    }

    fn next_ray(&self, intersection: &Intersection, ray: &Ray) -> Option<(Color, Ray)> {
        let direction = intersection.normal.reflect(&ray.direction);
        Some(
            (self.fresnel(-ray.direction * intersection.normal),
            Ray {
                origin:intersection.pos + RAY_START_EPSILON * direction,
                direction
            })
        )
    }

    fn get_color_ambient(&self, intersection: &Intersection, ray: &Ray, light: &AmbientLight) -> Color {
        let f = self.fresnel(-ray.direction * intersection.normal);
        f  * light.color
    }
}