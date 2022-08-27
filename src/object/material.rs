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

    fn scatter(&self, intersection: &Intersection, ray: &Ray) -> Option<Vec<(Color, Ray)>> { None }
}

fn fresnel(f0: Color, cos_theta: f32) -> Color {
    f0 + (Color::new(1.0)-f0) * f32::powf(1.0-cos_theta, 5.0)
}

fn calc_f0(n: &Color, k: &Color) -> Color {
    ((n-1.0)*(n-1.0) + k*k) /
    ((n+1.0)*(n+1.0) + k*k)
}

pub struct Metalic {
    f0: Color
}

impl Metalic {
    pub fn new(n: &Color, k: &Color) -> Metalic {
        Metalic {
            f0: calc_f0(n,k)
        }
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

    fn scatter(&self, intersection: &Intersection, ray: &Ray) -> Option<Vec<(Color, Ray)>> {
        let direction = intersection.normal.reflect(&ray.direction);
        Some(vec![
            (fresnel(self.f0, -ray.direction * intersection.normal),
            Ray {
                origin: intersection.pos + RAY_START_EPSILON * direction,
                direction,
                inside: false
            })
        ])
    }

    fn get_color_ambient(&self, intersection: &Intersection, ray: &Ray, light: &AmbientLight) -> Color {
        Color::default()
    }
}


pub struct Refractive {
    f0: Color,
    n: f32,
    n_rec: f32,
}

impl Refractive {
    pub fn new(n: &Color) -> Refractive {
        Refractive {
            f0: calc_f0(n, n),
            n: n.r,
            n_rec: 1.0 / n.r,
        }
    }

    pub fn glass() -> Refractive {
        Refractive::new(&Color::new(1.5))
    }
}

impl Shade for Refractive {
    fn get_color(&self, intersection: &Intersection, ray: &Ray, light: &Light) -> Color
    {
        Color::default()
    }

    fn scatter(&self, intersection: &Intersection, ray: &Ray) -> Option<Vec<(Color, Ray)>>  {
        let normal = if !ray.inside { intersection.normal } else { - intersection.normal };
        //TODO: feels like this is a bug, and should be swapped, but then the whole thing breaks..
        let n = if ray.inside {self.n} else {self.n_rec};

        let reflected_direction = normal.reflect(&ray.direction);
        let reflected_ray = Ray {
            direction: reflected_direction,
            origin: intersection.pos + RAY_START_EPSILON * reflected_direction,
            inside: ray.inside
        };

        if let Some(refracted_direction) = normal.refract(&ray.direction, n) {
            let refracted_ray = Ray {
                direction: refracted_direction,
                origin: intersection.pos + RAY_START_EPSILON * refracted_direction,
                inside: !ray.inside
            };

            let f = fresnel(self.f0, -ray.direction * normal);
            Some(vec![(f, reflected_ray), (Color::new(1.0) -f, refracted_ray)])
        } else {
            let f = fresnel(self.f0, -ray.direction * normal);
            Some(vec![(Color::new(1.), reflected_ray)])
        }
    }

    fn get_color_ambient(&self, intersection: &Intersection, ray: &Ray, light: &AmbientLight) -> Color {
        Color::default()
    }
}