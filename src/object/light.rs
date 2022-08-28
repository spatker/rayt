use crate::color::Color;
use crate::object::{Shade, ScatteredRay};
use crate::ray::{Intersection, Ray};

pub struct AmbientLight {
    pub color: Color,
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